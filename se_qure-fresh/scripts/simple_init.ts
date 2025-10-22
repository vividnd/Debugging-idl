import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Connection } from "@solana/web3.js";
import { SeQure } from "../target/types/se_qure";
import * as fs from "fs";
import * as os from "os";
import {
  getMXEAccAddress,
  getCompDefAccAddress,
  getCompDefAccOffset,
} from "@arcium-hq/client";

async function initializeCompDef() {
  try {
    // Setup connection
    const connection = new Connection("https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777", "confirmed");
    
    // Load wallet
    const payer = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(os.homedir() + "/.config/solana/id.json").toString()))
    );
    
    console.log("Wallet:", payer.publicKey.toBase58());
    
    // Setup provider
    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(payer),
      { commitment: "confirmed" }
    );
    anchor.setProvider(provider);
    
    // Load program from workspace
    const program = anchor.workspace.SeQure as Program<SeQure>;
    console.log("Program ID:", program.programId.toBase58());
    
    // Get MXE account
    const mxeAccount = getMXEAccAddress(program.programId);
    console.log("MXE Account:", mxeAccount.toBase58());
    
    // Get computation definition account for survey_analytics
    const compDefOffset = getCompDefAccOffset("survey_analytics");
    const compDefAccount = getCompDefAccAddress(program.programId, Buffer.from(compDefOffset).readUInt32LE());
    console.log("Comp Def Account:", compDefAccount.toBase58());
    
    // Setup accounts
    const accounts = {
      payer: payer.publicKey,
      mxeAccount: mxeAccount,
      compDefAccount: compDefAccount,
      arciumProgram: new PublicKey("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6"),
      systemProgram: anchor.web3.SystemProgram.programId
    };
    
    console.log("Accounts:", accounts);
    
    // Try to initialize
    console.log("Initializing survey_analytics computation definition...");
    const txSignature = await program.methods
      .initSurveyAnalyticsCompDef()
      .accounts(accounts)
      .signers([payer])
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    
    console.log("Success! Transaction signature:", txSignature);
    
  } catch (error) {
    console.error("Error:", error);
  }
}

initializeCompDef();
