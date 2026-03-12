import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, test } from "node:test";

import { BN, web3 } from "@coral-xyz/anchor";

import { loadOrCreateKeypair, resolveWalletPath, reserveTokenId } from "./common";

const originalAnchorWallet = process.env.ANCHOR_WALLET;
const originalHome = process.env.HOME;

afterEach(() => {
  if (originalAnchorWallet === undefined) {
    delete process.env.ANCHOR_WALLET;
  } else {
    process.env.ANCHOR_WALLET = originalAnchorWallet;
  }

  if (originalHome === undefined) {
    delete process.env.HOME;
  } else {
    process.env.HOME = originalHome;
  }
});

describe("resolveWalletPath", () => {
  test("prefers ANCHOR_WALLET when it is set", () => {
    process.env.ANCHOR_WALLET = "/tmp/custom-wallet.json";

    assert.equal(resolveWalletPath("/tmp/unused/Anchor.toml"), "/tmp/custom-wallet.json");
  });

  test("uses provider.wallet from Anchor.toml when ANCHOR_WALLET is unset", () => {
    delete process.env.ANCHOR_WALLET;

    const tempDir = mkdtempSync(join(tmpdir(), "doom-anchor-wallet-"));
    const anchorTomlPath = join(tempDir, "Anchor.toml");

    try {
      writeFileSync(
        anchorTomlPath,
        ["[provider]", 'cluster = "devnet"', 'wallet = "target/devnet/deployer.json"', ""].join("\n"),
        "utf8",
      );

      assert.equal(resolveWalletPath(anchorTomlPath), join(tempDir, "target/devnet/deployer.json"));
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test("falls back to the stable default wallet when provider.wallet is missing", () => {
    delete process.env.ANCHOR_WALLET;
    process.env.HOME = "/tmp/home-dir";

    const tempDir = mkdtempSync(join(tmpdir(), "doom-anchor-wallet-"));
    const anchorTomlPath = join(tempDir, "Anchor.toml");

    try {
      writeFileSync(anchorTomlPath, ["[provider]", 'cluster = "localnet"', ""].join("\n"), "utf8");

      assert.equal(resolveWalletPath(anchorTomlPath), "/tmp/home-dir/.config/solana/id.json");
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});

describe("loadOrCreateKeypair", () => {
  test("creates a new keypair file with owner-only permissions", () => {
    const tempDir = mkdtempSync(join(tmpdir(), "doom-keypair-"));
    const keypairPath = join(tempDir, "keypair.json");

    try {
      const keypair = loadOrCreateKeypair(keypairPath);
      const storedSecret = JSON.parse(readFileSync(keypairPath, "utf8")) as number[];

      assert.deepEqual(storedSecret, Array.from(keypair.secretKey));
      assert.equal(statSync(keypairPath).mode & 0o777, 0o600);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});

describe("reserveTokenId", () => {
  test("retries after a reservation contention error and recomputes the reservation PDA", async () => {
    const payer = web3.Keypair.generate();
    const seenReservations: string[] = [];
    let fetchCount = 0;
    let sendCount = 0;

    const result = await reserveTokenId({} as web3.Connection, payer, undefined, {
      fetchGlobalConfig: async () => {
        fetchCount += 1;
        const nextTokenId = fetchCount === 1 ? new BN(1) : new BN(2);
        return {
          admin: payer.publicKey,
          upgradeAuthority: payer.publicKey,
          nextTokenId,
          mintPaused: false,
          baseMetadataUrl: "https://example.com/base",
          collection: payer.publicKey,
          collectionUpdateAuthority: payer.publicKey,
          bump: 255,
        };
      },
      sendInstructions: async (_connection, _payer, instructions) => {
        sendCount += 1;
        seenReservations.push(instructions[0].keys[1].pubkey.toBase58());
        if (sendCount === 1) {
          throw new Error("Allocate: account Address already in use");
        }

        return "retry-signature";
      },
      backoffMs: 0,
    });

    assert.equal(result.signature, "retry-signature");
    assert.equal(result.tokenId.toString(), "2");
    assert.equal(seenReservations.length, 2);
    assert.notEqual(seenReservations[0], seenReservations[1]);
    assert.equal(result.reservation.toBase58(), seenReservations[1]);
    assert.equal(result.globalConfig.nextTokenId.toString(), "2");
  });
});
