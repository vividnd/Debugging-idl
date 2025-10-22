import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Connection, Transaction, SystemProgram } from "@solana/web3.js";
import { SeQure } from "./target/types/se_qure";
import * as fs from "fs";
import * as os from "os";
import {
  getMXEAccAddress,
  getCompDefAccAddress,
  getCompDefAccOffset,
} from "@arcium-hq/client";

const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777";
const WALLET_PATH = `${os.homedir()}/.config/solana/id.json`;
const ARCIUM_PROGRAM_ID = new PublicKey("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6");

function readKeypair(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}

async function directInit() {
  console.log("🚀 Direct Computation Definition Initialization");
  console.log("================================================");
  
  const connection = new Connection(RPC_ENDPOINT, "confirmed");
  const payer = readKeypair(WALLET_PATH);
  
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(payer),
    { commitment: "confirmed" }
  );
  anchor.setProvider(provider);
  
  const program = anchor.workspace.SeQure as Program<SeQure>;
  console.log(`Program ID: ${program.programId.toBase58()}`);
  
  // Get MXE account
  const mxeAccount = getMXEAccAddress(program.programId);
  console.log(`MXE Account: ${mxeAccount.toBase58()}`);
  
  // Get computation definition account for survey_analytics
  const compDefName = "survey_analytics";
    const offset = getCompDefAccOffset(compDefName);
    const compDefAccount = getCompDefAccAddress(program.programId, Buffer.from(offset).readUInt32LE());
  console.log(`Comp Def Account: ${compDefAccount.toBase58()}`);
  
  // Check if already exists
  const accountInfo = await connection.getAccountInfo(compDefAccount);
  if (accountInfo) {
    console.log("✅ Comp def account already exists. Skipping initialization.");
    return;
  }
  
  // Prepare accounts
  const accounts = {
    payer: payer.publicKey,
    mxeAccount,
    compDefAccount,
    arciumProgram: ARCIUM_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  };
  
  console.log("\n📋 Account details:");
  console.log(`  Payer: ${accounts.payer.toBase58()}`);
  console.log(`  MXE: ${accounts.mxeAccount.toBase58()}`);
  console.log(`  Comp Def: ${accounts.compDefAccount.toBase58()}`);
  console.log(`  Arcium Program: ${accounts.arciumProgram.toBase58()}`);
  console.log(`  System Program: ${accounts.systemProgram.toBase58()}`);
  
  try {
    console.log("\n🚀 Attempting direct initialization...");
    
    // Get recent blockhash
    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    
    // Try using the instruction directly
    const instruction = await program.methods
      .initSurveyAnalyticsCompDef()
      .accounts(accounts)
      .instruction();
    
    console.log("✅ Instruction created successfully");
    
    // Create and send transaction
    const transaction = new Transaction();
    transaction.add(instruction);
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    // Sign and send
    transaction.sign(payer);
    
    console.log("✅ Transaction signed successfully");
    
    const signature = await connection.sendTransaction(transaction, [payer], {
      skipPreflight: true
    });
    
    console.log(`✅ Transaction sent successfully!`);
    console.log(`📝 Signature: ${signature}`);
    
    // Wait for confirmation
    await connection.confirmTransaction(signature, 'confirmed');
    console.log(`✅ Transaction confirmed!`);
    
  } catch (error) {
    console.error("❌ Error during initialization:", error.message);
    if (error.logs) {
      console.error("📋 Transaction logs:", error.logs);
    }
    console.error("🔍 Full error:", error);
  }
}

directInit().catch(console.error);
