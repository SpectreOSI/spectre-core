import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SpectreClient } from "../packages/sdk/src";

async function main() {
  const connection = new Connection("https://api.mainnet-beta.solana.com");

  // NOTE: demo only – in practice load from a wallet / key store
  const from = Keypair.generate();
  const to = new PublicKey("11111111111111111111111111111111");

  const programId = new PublicKey("SpEcTre11111111111111111111111111111111111");

  const client = new SpectreClient({
    connection,
    programId
  });

  console.log("shadow address:", client.deriveShadowAddress(from.publicKey).toBase58());

  const sig = await client.sendPrivateTransfer(from, to, 1000);
  console.log("sent private-style transfer with sig:", sig);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
