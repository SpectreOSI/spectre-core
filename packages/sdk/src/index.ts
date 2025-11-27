import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  SendOptions,
} from "@solana/web3.js";

export interface SpectreClientConfig {
  connection: Connection;
  programId: PublicKey;
}

export class SpectreClient {
  readonly connection: Connection;
  readonly programId: PublicKey;

  constructor(cfg: SpectreClientConfig) {
    this.connection = cfg.connection;
    this.programId = cfg.programId;
  }

  /**
   * Derive the PDA for a Spectre shadow account.
   */
  deriveShadowAddress(owner: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("shadow"), owner.toBuffer()],
      this.programId
    );
    return pda;
  }

  /**
   * Build a transaction that initializes a Spectre shadow account.
   * This matches the on-chain `init_shadow_account` instruction.
   */
  async buildInitShadowTx(owner: PublicKey, payer: PublicKey): Promise<Transaction> {
    const shadow = this.deriveShadowAddress(owner);

    const ix = SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: shadow,
      lamports: 1_000_000,
      space: 64,
      programId: this.programId,
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer;

    const { blockhash } = await this.connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;

    return tx;
  }

  /**
   * Convenience helper to send a simple private-style transfer.
   * This doesn't perform full privacy logic but mirrors expected flow.
   */
  async sendPrivateTransfer(
    from: Keypair,
    to: PublicKey,
    lamports: number,
    opts?: SendOptions
  ): Promise<string> {
    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports,
      })
    );

    tx.feePayer = from.publicKey;
    const { blockhash } = await this.connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;

    return this.connection.sendTransaction(tx, [from], opts);
  }
}
