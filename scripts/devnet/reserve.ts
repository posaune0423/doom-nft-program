import { buildMetadataUri, getConnection, loadWallet, reserveTokenId, writeJson } from "./common";

async function main(): Promise<void> {
  const connection = getConnection();
  const payer = loadWallet();
  const { signature, tokenId, reservation, globalConfig } = await reserveTokenId(connection, payer);

  const output = {
    signature,
    user: payer.publicKey.toBase58(),
    tokenId: tokenId.toString(),
    reservation: reservation.toBase58(),
    metadataUri: buildMetadataUri(globalConfig.baseMetadataUrl, tokenId),
  };

  writeJson("target/devnet/latest-reservation.json", output);
  console.log(JSON.stringify(output, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
