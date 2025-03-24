import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftTransfer } from "../target/types/nft_transfer";
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { assert } from "chai";

describe("nft-transfer", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftTransfer as Program<NftTransfer>;
  const payer = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop SOL to the payer
    const signature = await provider.connection.requestAirdrop(
      payer.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);
  });

  it("Mints an NFT after payment", async () => {
    // Generate new keypair for the NFT mint
    const mint = anchor.web3.Keypair.generate();
    
    // Derive the associated token account address for the payer
    const tokenAccount = await getAssociatedTokenAddress(
      mint.publicKey,
      payer.publicKey
    );
    
    // Find the metadata account PDA
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    
    const [metadataAccount] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // Hard-coded payment receiver from the contract
    const paymentReceiver = new PublicKey("SPLEnDChZ5appDz8k61j18RCzMcHeER3Vh7tU46R"); // Updated address
    
    // NFT metadata
    const nftName = "Test NFT";
    const nftSymbol = "TNFT";
    const nftUri = "https://black-impossible-mink-832.mypinata.cloud/ipfs/bafkreidr7i2litxqwezg7njeei6a6oakewtnsbb7rcexqushw7jazzf6jy";
    
    try {
      // Call the mint_nft function
      await program.methods
        .mint_nft(nftName, nftSymbol, nftUri)
        .accounts({
          payer: payer.publicKey,
          mint: mint.publicKey,
          tokenAccount: tokenAccount,
          mintAuthority: payer.publicKey,
          metadata: metadataAccount,
          paymentReceiver: paymentReceiver,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([payer, mint])
        .rpc();
      
      console.log("NFT minted successfully!");
      
      // Verify the token balance in the token account
      const tokenBalance = await provider.connection.getTokenAccountBalance(tokenAccount);
      assert.equal(tokenBalance.value.amount, "1");
      
    } catch (error) {
      console.error("Error minting NFT:", error);
      throw error;
    }
  });
});
