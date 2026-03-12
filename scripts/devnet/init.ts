import {
  DEFAULT_PROGRAM_ID,
  fetchGlobalConfig,
  getConnection,
  initializeCollectionInstruction,
  initializeGlobalConfigInstruction,
  loadOrCreateKeypair,
  loadWallet,
  sendInstructions,
  writeJson,
} from "./common";

async function main(): Promise<void> {
  const baseMetadataUrl = process.env.BASE_METADATA_URL;
  if (!baseMetadataUrl) {
    throw new Error("BASE_METADATA_URL is required");
  }
  if (baseMetadataUrl.trim() !== baseMetadataUrl) {
    throw new Error("BASE_METADATA_URL must not contain leading or trailing whitespace");
  }
  if (!baseMetadataUrl.startsWith("https://")) {
    throw new Error("BASE_METADATA_URL must use https");
  }
  try {
    new URL(baseMetadataUrl);
  } catch {
    throw new Error("BASE_METADATA_URL must be a valid URL");
  }
  if (baseMetadataUrl.endsWith("/")) {
    throw new Error("BASE_METADATA_URL must not end with a trailing slash");
  }
  if (Buffer.byteLength(baseMetadataUrl, "utf8") > 256) {
    throw new Error("BASE_METADATA_URL must be 256 bytes or fewer");
  }

  const connection = getConnection();
  const payer = loadWallet();
  const upgradeAuthority = loadOrCreateKeypair(
    process.env.UPGRADE_AUTHORITY_KEYPAIR ?? "target/devnet/upgrade-authority.json",
  );
  const collection = loadOrCreateKeypair(process.env.COLLECTION_KEYPAIR ?? "target/devnet/collection.json");

  const initializeConfigIx = initializeGlobalConfigInstruction(
    payer.publicKey,
    baseMetadataUrl,
    upgradeAuthority.publicKey,
    DEFAULT_PROGRAM_ID,
  );
  const initializeCollectionIx = initializeCollectionInstruction(
    payer.publicKey,
    collection.publicKey,
    DEFAULT_PROGRAM_ID,
  );

  const configSignature = await sendInstructions(connection, payer, [initializeConfigIx]);
  const collectionSignature = await sendInstructions(connection, payer, [initializeCollectionIx], [collection]);

  const globalConfig = await fetchGlobalConfig(connection);
  const output = {
    programId: DEFAULT_PROGRAM_ID.toBase58(),
    configSignature,
    collectionSignature,
    admin: payer.publicKey.toBase58(),
    upgradeAuthority: upgradeAuthority.publicKey.toBase58(),
    collection: collection.publicKey.toBase58(),
    baseMetadataUrl: globalConfig.baseMetadataUrl,
    collectionUpdateAuthority: globalConfig.collectionUpdateAuthority.toBase58(),
  };

  writeJson("target/devnet/init.json", output);
  console.log(JSON.stringify(output, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
