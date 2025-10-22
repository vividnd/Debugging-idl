/**
 * Initialize Computation Definitions Script
 * 
 * This script initializes all 4 computation definitions for the SeQure program:
 * 1. survey_analytics
 * 2. quiz_evaluation  
 * 3. analytics_computation
 * 4. quiz_threshold_check
 * 
 * Usage:
 * ANCHOR_PROVIDER_URL=https://api.devnet.solana.com ANCHOR_WALLET=~/.config/solana/id.json npx ts-node scripts/init_comp_defs.ts
 */

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

// Configuration
const ARCIUM_PROGRAM_ID = new PublicKey("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6");
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const WALLET_PATH = `${os.homedir()}/.config/solana/id.json`;

// ✅ NICO FIX: Keep circuit names the same, only the OFFSETS changed (using _v2 suffix in comp_def_offset())
// This creates new comp def PDAs while keeping the circuit function names unchanged
const COMP_DEF_NAMES = [
  "survey_analytics",
  "quiz_evaluation", 
  "analytics_computation",
  "quiz_threshold_check"
] as const;

type CompDefName = typeof COMP_DEF_NAMES[number];

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
function getCompDefPDA(programId: PublicKey, compDefName: CompDefName): PublicKey {
  const offset = getCompDefAccOffset(compDefName);
  return getCompDefAccAddress(programId, Buffer.from(offset).readUInt32LE());
}

// Helper function to initialize a single computation definition
async function initializeCompDef(
  program: Program<SeQure>,
  payer: anchor.web3.Keypair,
  compDefName: CompDefName
): Promise<boolean> {
  try {
    console.log(`\n🔄 Initializing ${compDefName} computation definition...`);
    
    // Get account addresses - derive MXE from new program ID
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
      case "survey_analytics":
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
        
      case "quiz_evaluation":
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
        
      case "analytics_computation":
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
        
      case "quiz_threshold_check":
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
        throw new Error(`Unknown computation definition: ${compDefName}`);
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
async function initializeAllCompDefs(): Promise<void> {
  console.log("🚀 SeQure Computation Definitions Initialization");
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
    
    if (solBalance < 0.1) {
      console.log("   ⚠️  Warning: Low SOL balance. You may need more SOL for transaction fees.");
    }
    
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
    
    // Use existing MXE account that we found
    console.log("\n🔍 Using existing MXE account...");
    const mxeAccount = getMXEAccAddress(program.programId);
    const mxeExists = await accountExists(connection, mxeAccount);
    
    if (!mxeExists) {
      console.log(`   ❌ MXE account not found at ${mxeAccount.toBase58()}`);
      console.log(`   🔧 MXE needs to be initialized before computation definitions can be initialized.`);
      console.log(`   📝 To initialize MXE, you may need to:`);
      console.log(`      1. Use the Arcium CLI: \`arcium init-mxe\``);
      console.log(`      2. Or call the MXE initialization instruction directly`);
      console.log(`      3. Or use the Arcium dashboard to initialize MXE`);
      console.log(`   ⚠️  Please initialize MXE first, then re-run this script.`);
      throw new Error(`MXE account not initialized. Please initialize MXE first.`);
    }
    console.log(`   ✅ Using existing MXE account: ${mxeAccount.toBase58()}`);
    
    // Initialize all computation definitions
    console.log("\n🎯 Initializing computation definitions...");
    const results: { [key in CompDefName]: boolean } = {
      survey_analytics: false,
      quiz_evaluation: false,
      analytics_computation: false,
      quiz_threshold_check: false,
    };
    
    for (const compDefName of COMP_DEF_NAMES) {
      results[compDefName] = await initializeCompDef(program, payer, compDefName);
    }
    
    // Summary
    console.log("\n📊 Initialization Summary");
    console.log("=" .repeat(30));
    
    let successCount = 0;
    for (const [name, success] of Object.entries(results)) {
      const status = success ? "✅ SUCCESS" : "❌ FAILED";
      console.log(`   ${name}: ${status}`);
      if (success) successCount++;
    }
    
    console.log(`\n🎉 Initialization complete! ${successCount}/${COMP_DEF_NAMES.length} computation definitions ready.`);
    
    if (successCount === COMP_DEF_NAMES.length) {
      console.log("\n🚀 All computation definitions are now ready for use!");
      console.log("   You can now create surveys, quizzes, and submit responses with MPC processing.");
    } else {
      console.log("\n⚠️  Some computation definitions failed to initialize.");
      console.log("   Please check the error messages above and retry if needed.");
    }
    
  } catch (error) {
    console.error("\n❌ Initialization failed:", error.message);
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
  initializeAllCompDefs()
    .then(() => {
      console.log("\n✅ Script completed successfully!");
      process.exit(0);
    })
    .catch((error) => {
      console.error("\n❌ Script failed:", error);
      process.exit(1);
    });
}

export { initializeAllCompDefs };
