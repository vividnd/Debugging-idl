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

async function debugInitMethods() {
  console.log("🔍 DEBUGGING INITIALIZATION METHODS");
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
    
    // Test both v1 and v2 account derivation
    console.log("\n🔍 COMPARING V1 vs V2 ACCOUNT DERIVATION:");
    console.log("=" .repeat(50));
    
    const testNames = [
      { name: "survey_analytics", type: "v1" },
      { name: "survey_analytics_v2", type: "v2" },
      { name: "quiz_evaluation", type: "v1" },
      { name: "quiz_evaluation_v2", type: "v2" },
      { name: "quiz_threshold_check", type: "v1" },
      { name: "quiz_threshold_check_v2", type: "v2" }
    ];
    
    for (const test of testNames) {
      console.log(`\n📋 ${test.name} (${test.type}):`);
      
      try {
        const offset = getCompDefAccOffset(test.name);
        const compDefAccount = getCompDefPDA(program.programId, test.name);
        const exists = await accountExists(connection, compDefAccount);
        
        console.log(`   Offset: ${Buffer.from(offset).toString('hex')}`);
        console.log(`   Account: ${compDefAccount.toBase58()}`);
        console.log(`   Exists: ${exists}`);
        
        if (exists) {
          const accountInfo = await connection.getAccountInfo(compDefAccount);
          console.log(`   Data length: ${accountInfo?.data.length} bytes`);
          console.log(`   Owner: ${accountInfo?.owner.toBase58()}`);
        }
        
      } catch (error) {
        console.log(`   ❌ Error: ${error.message}`);
      }
    }
    
    // Test the actual method calls
    console.log("\n🔍 TESTING METHOD CALLS:");
    console.log("=" .repeat(30));
    
    // Test v1 method (should work)
    console.log("\n📋 Testing v1 method (survey_analytics):");
    try {
      const v1Account = getCompDefPDA(program.programId, "survey_analytics");
      const accounts = {
        payer: payer.publicKey,
        mxeAccount,
        compDefAccount: v1Account,
        arciumProgram: ARCIUM_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      };
      
      console.log("   Accounts:", {
        payer: accounts.payer.toBase58(),
        mxeAccount: accounts.mxeAccount.toBase58(),
        compDefAccount: accounts.compDefAccount.toBase58(),
        arciumProgram: accounts.arciumProgram.toBase58(),
        systemProgram: accounts.systemProgram.toBase58()
      });
      
      // Just test the method creation, don't actually call it
      const method = program.methods.initSurveyAnalyticsCompDef();
      console.log("   ✅ Method created successfully");
      console.log("   Method type:", typeof method);
      
    } catch (error) {
      console.log(`   ❌ Error: ${error.message}`);
    }
    
    // Test v2 method (should work the same)
    console.log("\n📋 Testing v2 method (survey_analytics_v2):");
    try {
      const v2Account = getCompDefPDA(program.programId, "survey_analytics_v2");
      const accounts = {
        payer: payer.publicKey,
        mxeAccount,
        compDefAccount: v2Account,
        arciumProgram: ARCIUM_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      };
      
      console.log("   Accounts:", {
        payer: accounts.payer.toBase58(),
        mxeAccount: accounts.mxeAccount.toBase58(),
        compDefAccount: accounts.compDefAccount.toBase58(),
        arciumProgram: accounts.arciumProgram.toBase58(),
        systemProgram: accounts.systemProgram.toBase58()
      });
      
      // Just test the method creation, don't actually call it
      const method = program.methods.initSurveyAnalyticsCompDef();
      console.log("   ✅ Method created successfully");
      console.log("   Method type:", typeof method);
      
    } catch (error) {
      console.log(`   ❌ Error: ${error.message}`);
    }
    
    // Check if there are any differences in the method signatures
    console.log("\n🔍 CHECKING METHOD SIGNATURES:");
    console.log("=" .repeat(40));
    
    const methods = [
      'initSurveyAnalyticsCompDef',
      'initQuizEvaluationCompDef', 
      'initAnalyticsComputationCompDef',
      'initQuizThresholdCheckCompDef'
    ];
    
    for (const methodName of methods) {
      try {
        const method = (program.methods as any)[methodName];
        console.log(`   ${methodName}: ${typeof method}`);
        
        if (typeof method === 'function') {
          console.log(`   ✅ Available`);
        } else {
          console.log(`   ❌ Not available or wrong type`);
        }
      } catch (error) {
        console.log(`   ❌ Error accessing ${methodName}: ${error.message}`);
      }
    }
    
  } catch (error) {
    console.error("❌ Debug failed:", error.message);
  }
}

debugInitMethods().catch(console.error);
