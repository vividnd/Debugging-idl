/**
 * Initialize V3 Computation Definitions
 * 
 * This script initializes the v3 computation definitions with the new program ID.
 */

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
const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777";
const WALLET_PATH = `${os.homedir()}/.config/solana/id.json`;

// V3 computation definition names
const V3_COMP_DEF_NAMES = [
  "survey_analytics_v3",
  "quiz_evaluation_v3", 
  "analytics_computation_v3",
  "quiz_threshold_check_v3"
] as const;

type V3CompDefName = typeof V3_COMP_DEF_NAMES[number];

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
function getCompDefPDA(programId: PublicKey, compDefName: V3CompDefName): PublicKey {
  const offset = getCompDefAccOffset(compDefName);
  return getCompDefAccAddress(programId, Buffer.from(offset).readUInt32LE());
}

// Helper function to initialize a single v3 computation definition
async function initializeV3CompDef(
  program: Program<SeQure>,
  payer: anchor.web3.Keypair,
  compDefName: V3CompDefName
): Promise<boolean> {
  try {
    console.log(`\n🔄 Initializing ${compDefName} computation definition...`);
    
    // Get account addresses
    const mxeAccount = getMXEAccAddress(program.programId);
    const compDefAccount = getCompDefPDA(program.programId, compDefName);
    
    console.log(`   MXE Account: ${mxeAccount.toBase58()}`);
    console.log(`   Comp Def Account: ${compDefAccount.toBase58()}`);
    
    // Check if already initialized
    const connection = program.provider.connection;
    const alreadyExists = await accountExists(connection, compDefAccount);
    
    if (alreadyExists) {
      console.log(`   ✅ ${compDefName} computation definition already initialized`);
      return true;
    }
    
    // Prepare accounts object
    const accounts = {
      payer: payer.publicKey,
      mxeAccount,
      compDefAccount,
      arciumProgram: ARCIUM_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    
    // Call the appropriate initialization function
    let txSignature: string;
    
    // Get recent blockhash to ensure transaction is valid
    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    
    switch (compDefName) {
      case "survey_analytics_v3":
        txSignature = await program.methods
          .initSurveyAnalyticsCompDef()
          .accounts(accounts)
          .signers([payer])
          .rpc({ 
            skipPreflight: true, 
            commitment: "confirmed",
            preflightCommitment: "confirmed"
          });
        break;
        
      case "quiz_evaluation_v3":
        txSignature = await program.methods
          .initQuizEvaluationCompDef()
          .accounts(accounts)
          .signers([payer])
          .rpc({ 
            skipPreflight: true, 
            commitment: "confirmed",
            preflightCommitment: "confirmed"
          });
        break;
        
      case "analytics_computation_v3":
        txSignature = await program.methods
          .initAnalyticsComputationCompDef()
          .accounts(accounts)
          .signers([payer])
          .rpc({ 
            skipPreflight: true, 
            commitment: "confirmed",
            preflightCommitment: "confirmed"
          });
        break;
        
      case "quiz_threshold_check_v3":
        txSignature = await program.methods
          .initQuizThresholdCheckCompDef()
          .accounts(accounts)
          .signers([payer])
          .rpc({ 
            skipPreflight: true, 
            commitment: "confirmed",
            preflightCommitment: "confirmed"
          });
        break;
        
      default:
        throw new Error(`Unknown v3 computation definition: ${compDefName}`);
    }
    
    console.log(`   ✅ ${compDefName} computation definition initialized successfully!`);
    console.log(`   📝 Transaction signature: ${txSignature}`);
    
    return true;
    
  } catch (error) {
    console.error(`   ❌ Failed to initialize ${compDefName}:`, error.message);
    if (error.logs) {
      console.error(`   📋 Transaction logs:`, error.logs);
    }
    console.error(`   🔍 Full error:`, error);
    return false;
  }
}

// Main initialization function
async function initializeAllV3CompDefs(): Promise<void> {
  console.log("🚀 SeQure V3 Computation Definitions Initialization");
  console.log("=" .repeat(60));
  
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
    
    // Check MXE account
    console.log("\n🔍 Checking MXE account...");
    const mxeAccount = getMXEAccAddress(program.programId);
    const mxeExists = await accountExists(connection, mxeAccount);
    
    if (!mxeExists) {
      console.log(`   ❌ MXE account not found at ${mxeAccount.toBase58()}`);
      console.log(`   🔧 MXE needs to be initialized before computation definitions can be initialized.`);
      throw new Error(`MXE account not initialized. Please initialize MXE first.`);
    }
    console.log(`   ✅ MXE account exists: ${mxeAccount.toBase58()}`);
    
    // Initialize all v3 computation definitions
    console.log("\n🎯 Initializing v3 computation definitions...");
    const results: { [key in V3CompDefName]: boolean } = {
      survey_analytics_v3: false,
      quiz_evaluation_v3: false,
      analytics_computation_v3: false,
      quiz_threshold_check_v3: false,
    };
    
    for (const compDefName of V3_COMP_DEF_NAMES) {
      results[compDefName] = await initializeV3CompDef(program, payer, compDefName);
    }
    
    // Summary
    console.log("\n📊 V3 Initialization Summary");
    console.log("=" .repeat(40));
    
    let successCount = 0;
    for (const [name, success] of Object.entries(results)) {
      const status = success ? "✅ SUCCESS" : "❌ FAILED";
      console.log(`   ${name}: ${status}`);
      if (success) successCount++;
    }
    
    console.log(`\n🎉 V3 initialization complete! ${successCount}/${V3_COMP_DEF_NAMES.length} computation definitions ready.`);
    
    if (successCount === V3_COMP_DEF_NAMES.length) {
      console.log("\n🚀 All v3 computation definitions are now ready for use!");
      console.log("   Your quiz grading should now work properly.");
    } else {
      console.log("\n⚠️  Some v3 computation definitions failed to initialize.");
      console.log("   Please check the error messages above and retry if needed.");
    }
    
  } catch (error) {
    console.error("\n❌ V3 initialization failed:", error.message);
    console.error("\n🔧 Troubleshooting tips:");
    console.error("   1. Ensure your program is deployed to devnet");
    console.error("   2. Check that your wallet has sufficient SOL balance");
    console.error("   3. Verify the MXE account is initialized on devnet");
    console.error("   4. Make sure the RPC endpoint is accessible");
    process.exit(1);
  }
}

// Run the initialization
if (require.main === module) {
  initializeAllV3CompDefs()
    .then(() => {
      console.log("\n✅ V3 script completed successfully!");
      process.exit(0);
    })
    .catch((error) => {
      console.error("\n❌ V3 script failed:", error);
      process.exit(1);
    });
}

export { initializeAllV3CompDefs };
