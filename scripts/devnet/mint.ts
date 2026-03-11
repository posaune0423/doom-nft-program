import {
  buildMetadataUri,
  fetchGlobalConfig,
  getConnection,
  Keypair,
  loadWallet,
  mintDoomIndexNftInstruction,
  readJson,
  sendInstructions,
  writeJson,
} from "./common";

type ReservationOutput = {
  tokenId: string;
};

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

  const metadataUri = buildMetadataUri(globalConfig.baseMetadataUrl, tokenId);
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

  const imageResponse = await fetch(metadata.image, { method: "HEAD" });
  if (!imageResponse.ok) {
    throw new Error(`Image fetch failed: ${imageResponse.status} ${imageResponse.statusText}`);
  }

  const animationResponse = await fetch(metadata.animation_url, {
    method: "HEAD",
  });
  if (!animationResponse.ok) {
    throw new Error(`animation_url fetch failed: ${animationResponse.status} ${animationResponse.statusText}`);
  }

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

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
