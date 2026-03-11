import { BN, web3 } from "@coral-xyz/anchor";
import * as borsh from "@coral-xyz/borsh";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} = web3;

export { Keypair, PublicKey, SystemProgram, TransactionInstruction };

export const DEFAULT_PROGRAM_ID = new PublicKey(
  process.env.PROGRAM_ID ?? "AavECgzCbVhHeBGAfcUgT1tYEC4N4B96E8XtF9H1fMGt",
);
export const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

const GLOBAL_CONFIG_SEED = Buffer.from("global_config");
const RESERVATION_SEED = Buffer.from("reservation");
const COLLECTION_AUTHORITY_SEED = Buffer.from("collection_authority");
const GLOBAL_CONFIG_DISCRIMINATOR = accountDiscriminator("GlobalConfig");

const GLOBAL_CONFIG_LAYOUT = borsh.struct([
  borsh.publicKey("admin"),
  borsh.publicKey("upgradeAuthority"),
  borsh.u64("nextTokenId"),
  borsh.bool("mintPaused"),
  borsh.str("baseMetadataUrl"),
  borsh.publicKey("collection"),
  borsh.publicKey("collectionUpdateAuthority"),
  borsh.u8("bump"),
]);

export type GlobalConfigAccount = {
  admin: web3.PublicKey;
  upgradeAuthority: web3.PublicKey;
  nextTokenId: BN;
  mintPaused: boolean;
  baseMetadataUrl: string;
  collection: web3.PublicKey;
  collectionUpdateAuthority: web3.PublicKey;
  bump: number;
};

export function getConnection(): web3.Connection {
  return new Connection(process.env.ANCHOR_PROVIDER_URL ?? "https://api.devnet.solana.com", "confirmed");
}

export function loadWallet(): web3.Keypair {
  const walletPath = resolve(process.env.ANCHOR_WALLET ?? "~/.config/solana/id.json").replace(
    /^~(?=$|\/|\\)/,
    process.env.HOME ?? "",
  );
  return loadKeypair(walletPath);
}

export function loadKeypair(filePath: string): web3.Keypair {
  const secretKey = Uint8Array.from(JSON.parse(readFileSync(resolve(filePath), "utf8")) as number[]);
  return Keypair.fromSecretKey(secretKey);
}

export function loadOrCreateKeypair(filePath: string): web3.Keypair {
  const absolutePath = resolve(filePath);
  if (existsSync(absolutePath)) {
    return loadKeypair(absolutePath);
  }

  mkdirSync(dirname(absolutePath), { recursive: true });
  const keypair = Keypair.generate();
  writeFileSync(absolutePath, JSON.stringify(Array.from(keypair.secretKey)), "utf8");
  return keypair;
}

export function writeJson(filePath: string, payload: unknown): void {
  const absolutePath = resolve(filePath);
  mkdirSync(dirname(absolutePath), { recursive: true });
  writeFileSync(absolutePath, JSON.stringify(payload, null, 2), "utf8");
}

export function readJson<T>(filePath: string): T {
  return JSON.parse(readFileSync(resolve(filePath), "utf8")) as T;
}

export function globalConfigPda(programId: web3.PublicKey = DEFAULT_PROGRAM_ID): [web3.PublicKey, number] {
  return PublicKey.findProgramAddressSync([GLOBAL_CONFIG_SEED], programId);
}

export function collectionAuthorityPda(
  globalConfig: web3.PublicKey,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): [web3.PublicKey, number] {
  return PublicKey.findProgramAddressSync([COLLECTION_AUTHORITY_SEED, globalConfig.toBuffer()], programId);
}

export function reservationPda(
  tokenId: bigint,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): [web3.PublicKey, number] {
  const tokenIdBuffer = Buffer.alloc(8);
  tokenIdBuffer.writeBigUInt64LE(tokenId);
  return PublicKey.findProgramAddressSync([RESERVATION_SEED, tokenIdBuffer], programId);
}

export async function fetchGlobalConfig(
  connection: web3.Connection,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): Promise<GlobalConfigAccount> {
  const [config] = globalConfigPda(programId);
  const account = await connection.getAccountInfo(config, "confirmed");
  if (!account) {
    throw new Error(`GlobalConfig not found at ${config.toBase58()}`);
  }

  const discriminator = account.data.subarray(0, 8);
  if (!discriminator.equals(GLOBAL_CONFIG_DISCRIMINATOR)) {
    throw new Error("GlobalConfig discriminator mismatch");
  }

  return GLOBAL_CONFIG_LAYOUT.decode(account.data.subarray(8)) as GlobalConfigAccount;
}

export function buildMetadataUri(baseMetadataUrl: string, tokenId: bigint): string {
  return `${baseMetadataUrl}/${tokenId.toString()}.json`;
}

export function createInstruction(
  name: string,
  argsLayout: borsh.Layout<unknown>,
  args: Record<string, unknown>,
  keys: web3.AccountMeta[],
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): web3.TransactionInstruction {
  const discriminator = instructionDiscriminator(name);
  const argsBuffer = Buffer.alloc(1024);
  const encodedLength = argsLayout.encode(args, argsBuffer);
  const data = Buffer.concat([discriminator, argsBuffer.subarray(0, encodedLength)]);

  return new TransactionInstruction({ programId, keys, data });
}

export async function sendInstructions(
  connection: web3.Connection,
  payer: web3.Keypair,
  instructions: web3.TransactionInstruction[],
  signers: web3.Keypair[] = [],
): Promise<string> {
  const transaction = new Transaction().add(...instructions);
  const signature = await sendAndConfirmTransaction(connection, transaction, [payer, ...signers], {
    commitment: "confirmed",
  });

  return signature;
}

export function initializeGlobalConfigInstruction(
  admin: web3.PublicKey,
  baseMetadataUrl: string,
  upgradeAuthority: web3.PublicKey,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): web3.TransactionInstruction {
  const [globalConfig] = globalConfigPda(programId);
  return createInstruction(
    "initialize_global_config",
    borsh.struct([borsh.str("baseMetadataUrl"), borsh.publicKey("upgradeAuthority")]),
    { baseMetadataUrl, upgradeAuthority },
    [
      { pubkey: globalConfig, isSigner: false, isWritable: true },
      { pubkey: admin, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
  );
}

export function initializeCollectionInstruction(
  admin: web3.PublicKey,
  collection: web3.PublicKey,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): web3.TransactionInstruction {
  const [globalConfig] = globalConfigPda(programId);
  const [collectionAuthority] = collectionAuthorityPda(globalConfig, programId);
  return createInstruction(
    "initialize_collection",
    borsh.struct([]),
    {},
    [
      { pubkey: globalConfig, isSigner: false, isWritable: true },
      { pubkey: admin, isSigner: true, isWritable: true },
      { pubkey: collection, isSigner: true, isWritable: true },
      { pubkey: collectionAuthority, isSigner: false, isWritable: false },
      { pubkey: MPL_CORE_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
  );
}

export async function reserveTokenId(
  connection: web3.Connection,
  payer: web3.Keypair,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): Promise<{
  signature: string;
  tokenId: bigint;
  reservation: web3.PublicKey;
  globalConfig: GlobalConfigAccount;
}> {
  const before = await fetchGlobalConfig(connection, programId);
  const tokenId = BigInt(before.nextTokenId.toString());
  const [globalConfig] = globalConfigPda(programId);
  const [reservation] = reservationPda(tokenId, programId);

  const instruction = createInstruction(
    "reserve_token_id",
    borsh.struct([]),
    {},
    [
      { pubkey: globalConfig, isSigner: false, isWritable: true },
      { pubkey: reservation, isSigner: false, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
  );

  const signature = await sendInstructions(connection, payer, [instruction]);
  const after = await fetchGlobalConfig(connection, programId);

  return { signature, tokenId, reservation, globalConfig: after };
}

export function mintDoomIndexNftInstruction(
  user: web3.PublicKey,
  tokenId: bigint,
  asset: web3.PublicKey,
  collection: web3.PublicKey,
  programId: web3.PublicKey = DEFAULT_PROGRAM_ID,
): web3.TransactionInstruction {
  const [globalConfig] = globalConfigPda(programId);
  const [reservation] = reservationPda(tokenId, programId);
  const [collectionAuthority] = collectionAuthorityPda(globalConfig, programId);

  return createInstruction(
    "mint_doom_index_nft",
    borsh.struct([borsh.u64("tokenId")]),
    { tokenId: new BN(tokenId.toString()) },
    [
      { pubkey: globalConfig, isSigner: false, isWritable: false },
      { pubkey: reservation, isSigner: false, isWritable: true },
      { pubkey: user, isSigner: true, isWritable: true },
      { pubkey: asset, isSigner: true, isWritable: true },
      { pubkey: collectionAuthority, isSigner: false, isWritable: false },
      { pubkey: collection, isSigner: false, isWritable: true },
      { pubkey: MPL_CORE_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId,
  );
}

function instructionDiscriminator(name: string): Buffer {
  return createHash("sha256").update(`global:${name}`).digest().subarray(0, 8);
}

function accountDiscriminator(name: string): Buffer {
  return createHash("sha256").update(`account:${name}`).digest().subarray(0, 8);
}
