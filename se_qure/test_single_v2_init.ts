import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Connection } from "@solana/web3.js";
import { SeQure } from "./target/types/se_qure";
import * as fs from "fs";
import * as os from "os";
import {
  getMXEAccAddress,
  getCompDefAccAddress,
  getCompDefAccOffset,
} from "@arcium-hq/client";

// Configuration
const ARCIUM_PROGRAM_ID = new PublicKey("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6");
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const WALLET_PATH = `${os.homedir()}/.config/solana/id.json`;

// Helper function to read keypair from file
function readKeypair(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}

// Helper function to check if account exists
async function accountExists(connection: Connection, address: PublicKey): Promise<boolean> {
  try {
    const accountInfo = await connection.getAccountInfo(address);
    return accountInfo !== null;
  } catch (error) {
    return false;
  }
}

// Helper function to get computation definition PDA
function getCompDefPDA(programId: PublicKey, compDefName: string): PublicKey {
  const offset = getCompDefAccOffset(compDefName);
  return getCompDefAccAddress(programId, Buffer.from(offset).readUInt32LE());
}

async function testSingleV2Init() {
  console.log("🧪 TESTING SINGLE V2 INITIALIZATION");
  console.log("=" .repeat(50));
  
  try {
    // Setup connection and program
    console.log("\n📡 Setting up connection...");
    const connection = new Connection(RPC_ENDPOINT, "confirmed");
    const version = await connection.getVersion();
    console.log(`   ✅ Connected to Solana ${version['solana-core']} on devnet`);
    
    // Load wallet
    console.log("\n🔑 Loading wallet...");
    const payer = readKeypair(WALLET_PATH);
    console.log(`   ✅ Wallet loaded: ${payer.publicKey.toBase58()}`);
    
    // Check wallet balance
    const balance = await connection.getBalance(payer.publicKey);
    const solBalance = balance / anchor.web3.LAMPORTS_PER_SOL;
    console.log(`   💰 Wallet balance: ${solBalance.toFixed(4)} SOL`);
    
    // Setup Anchor provider and program
    console.log("\n🔧 Setting up Anchor program...");
    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(payer),
      { commitment: "confirmed" }
    );
    anchor.setProvider(provider);
    
    const program = anchor.workspace.SeQure as Program<SeQure>;
    console.log(`   ✅ Program loaded: ${program.programId.toBase58()}`);
    
    // Get MXE account
    const mxeAccount = getMXEAccAddress(program.programId);
    console.log(`   ✅ MXE Account: ${mxeAccount.toBase58()}`);
    
    // Test with quiz_threshold_check_v2 (the one your program expects)
    const compDefName = "quiz_threshold_check_v2";
    console.log(`\n🎯 Testing ${compDefName} initialization...`);
    
    const compDefAccount = getCompDefPDA(program.programId, compDefName);
    console.log(`   Comp Def Account: ${compDefAccount.toBase58()}`);
    
    // Check if already exists
    const alreadyExists = await accountExists(connection, compDefAccount);
    console.log(`   Already exists: ${alreadyExists}`);
    
    if (alreadyExists) {
      console.log("   ✅ Account already exists, skipping initialization");
      return;
    }
    
    // Prepare accounts object
    const accounts = {
      payer: payer.publicKey,
      mxeAccount,
      compDefAccount,
      arciumProgram: ARCIUM_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    
    console.log("\n📋 Account details:");
    console.log("   Payer:", accounts.payer.toBase58());
    console.log("   MXE Account:", accounts.mxeAccount.toBase58());
    console.log("   Comp Def Account:", accounts.compDefAccount.toBase58());
    console.log("   Arcium Program:", accounts.arciumProgram.toBase58());
    console.log("   System Program:", accounts.systemProgram.toBase58());
    
    // Get recent blockhash
    console.log("\n🔄 Getting recent blockhash...");
    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    console.log(`   Blockhash: ${blockhash}`);
    
    // Try to create the instruction first
    console.log("\n🔧 Creating instruction...");
    try {
      const instruction = await program.methods
        .initQuizThresholdCheckCompDef()
        .accounts(accounts)
        .instruction();
      
      console.log("   ✅ Instruction created successfully");
      console.log("   Instruction data length:", instruction.data.length);
      console.log("   Instruction program ID:", instruction.programId.toBase58());
      console.log("   Instruction accounts:", instruction.keys.length);
      
    } catch (error) {
      console.log("   ❌ Failed to create instruction:", error.message);
      console.log("   Error details:", error);
      return;
    }
    
    // Try to simulate the transaction
    console.log("\n🧪 Simulating transaction...");
    try {
      const transaction = new anchor.web3.Transaction();
      transaction.add(
        await program.methods
          .initQuizThresholdCheckCompDef()
          .accounts(accounts)
          .instruction()
      );
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      
      const simulation = await connection.simulateTransaction(transaction);
      
      console.log("   Simulation result:", simulation.value);
      console.log("   Logs:", simulation.value.logs);
      
      if (simulation.value.err) {
        console.log("   ❌ Simulation failed:", simulation.value.err);
        return;
      } else {
        console.log("   ✅ Simulation successful");
      }
      
    } catch (error) {
      console.log("   ❌ Simulation error:", error.message);
      console.log("   Error details:", error);
      return;
    }
    
    // If simulation passes, try the actual transaction
    console.log("\n🚀 Attempting actual transaction...");
    try {
      const txSignature = await program.methods
        .initQuizThresholdCheckCompDef()
        .accounts(accounts)
        .signers([payer])
        .rpc({ 
          skipPreflight: false, 
          commitment: "confirmed",
          preflightCommitment: "confirmed"
        });
      
      console.log("   ✅ Transaction successful!");
      console.log("   Signature:", txSignature);
      
    } catch (error) {
      console.log("   ❌ Transaction failed:", error.message);
      console.log("   Error details:", error);
      
      if (error.logs) {
        console.log("   Transaction logs:", error.logs);
      }
    }
    
  } catch (error) {
    console.error("❌ Test failed:", error.message);
    console.error("Full error:", error);
  }
}

testSingleV2Init().catch(console.error);
