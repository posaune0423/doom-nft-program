import {
  fetchGlobalConfig,
  getConnection,
  Keypair,
  MPL_CORE_PROGRAM_ID,
  loadWallet,
  mintDoomIndexNftInstruction,
  readJson,
  sendInstructions,
  writeJson,
} from "./common";

type ReservationOutput = {
  tokenId: string;
};

type FetchLike = typeof fetch;

export function decodeAssetUri(accountData: Buffer | Uint8Array): string {
  const data = Buffer.from(accountData);
  let offset = 0;

  const key = data.readUInt8(offset);
  offset += 1;
  if (key !== 1) {
    throw new Error(`Expected Metaplex Core AssetV1 account data, got key ${key}`);
  }

  offset += 32;

  const updateAuthorityKind = data.readUInt8(offset);
  offset += 1;
  if (updateAuthorityKind === 1 || updateAuthorityKind === 2) {
    offset += 32;
  } else if (updateAuthorityKind !== 0) {
    throw new Error(`Unknown Metaplex Core update authority kind ${updateAuthorityKind}`);
  }

  const [, afterName] = readBorshString(data, offset);
  const [uri] = readBorshString(data, afterName);
  return uri;
}

export async function assertUrlReachable(url: string, label: string, fetchImpl: FetchLike = fetch): Promise<void> {
  const headResponse = await fetchImpl(url, { method: "HEAD" });
  if (headResponse.ok) {
    return;
  }

  const getResponse = await fetchImpl(url, { method: "GET" });
  if (!getResponse.ok) {
    throw new Error(`${label} fetch failed: ${getResponse.status} ${getResponse.statusText}`);
  }
}

async function main(): Promise<void> {
  const connection = getConnection();
  const payer = loadWallet();
  const globalConfig = await fetchGlobalConfig(connection);
  const reservation =
    process.env.TOKEN_ID !== undefined
      ? { tokenId: process.env.TOKEN_ID }
      : readJson<ReservationOutput>("target/devnet/latest-reservation.json");
  const tokenId = BigInt(reservation.tokenId);
  const asset = Keypair.generate();

  const mintInstruction = mintDoomIndexNftInstruction(
    payer.publicKey,
    tokenId,
    asset.publicKey,
    globalConfig.collection,
  );
  const signature = await sendInstructions(connection, payer, [mintInstruction], [asset]);

  const assetAccount = await connection.getAccountInfo(asset.publicKey, "confirmed");
  if (!assetAccount) {
    throw new Error(`Asset account not found at ${asset.publicKey.toBase58()}`);
  }
  if (!assetAccount.owner.equals(MPL_CORE_PROGRAM_ID)) {
    throw new Error(`Asset account ${asset.publicKey.toBase58()} is not owned by Metaplex Core`);
  }

  const metadataUri = decodeAssetUri(assetAccount.data);
  const metadataResponse = await fetch(metadataUri);
  if (!metadataResponse.ok) {
    throw new Error(`Metadata fetch failed: ${metadataResponse.status} ${metadataResponse.statusText}`);
  }
  const metadata = (await metadataResponse.json()) as {
    image?: string;
    animation_url?: string;
  };
  if (!metadata.image || !metadata.animation_url) {
    throw new Error("Metadata must include both image and animation_url");
  }

  await assertUrlReachable(metadata.image, "Image");
  await assertUrlReachable(metadata.animation_url, "animation_url");

  const output = {
    signature,
    tokenId: tokenId.toString(),
    asset: asset.publicKey.toBase58(),
    metadataUri,
    image: metadata.image,
    animationUrl: metadata.animation_url,
  };

  writeJson("target/devnet/latest-mint.json", output);
  console.log(JSON.stringify(output, null, 2));
}

function readBorshString(data: Buffer, offset: number): [string, number] {
  const length = data.readUInt32LE(offset);
  const start = offset + 4;
  const end = start + length;
  return [data.subarray(start, end).toString("utf8"), end];
}

if (require.main === module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}
