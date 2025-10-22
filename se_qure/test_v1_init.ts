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

// Test v1 names
const COMP_DEF_NAMES = [
  "survey_analytics_v1",
  "quiz_evaluation_v1", 
  "analytics_computation_v1",
  "quiz_threshold_check_v1"
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

// Helper function to get computation definition PDA for v1 names
function getCompDefPDA(programId: PublicKey, compDefName: CompDefName): PublicKey {
  const offset = getCompDefAccOffset(compDefName);
  return getCompDefAccAddress(programId, Buffer.from(offset).readUInt32LE());
}

// Test initializing with v1 names
async function testV1Initialization() {
  try {
    console.log("🧪 Testing v1 initialization...");
    
    // Setup connection and program
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
    
    // Test survey_analytics_v1
    const compDefName = "survey_analytics_v1" as CompDefName;
    console.log(`\nTesting ${compDefName}...`);
    
    const mxeAccount = getMXEAccAddress(program.programId);
    const compDefAccount = getCompDefPDA(program.programId, compDefName);
    
    console.log(`MXE Account: ${mxeAccount.toBase58()}`);
    console.log(`Comp Def Account: ${compDefAccount.toBase58()}`);
    
    // Check if already exists
    const alreadyExists = await accountExists(connection, compDefAccount);
    if (alreadyExists) {
      console.log(`✅ ${compDefName} already exists`);
      return;
    }
    
    // Try to initialize
    const accounts = {
      payer: payer.publicKey,
      mxeAccount,
      compDefAccount,
      arciumProgram: ARCIUM_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    
    console.log(`Attempting to initialize ${compDefName}...`);
    
    const txSignature = await program.methods
      .initSurveyAnalyticsCompDef()
      .accounts(accounts)
      .signers([payer])
      .rpc({ 
        skipPreflight: true, 
        commitment: "confirmed",
        preflightCommitment: "confirmed"
      });
    
    console.log(`✅ ${compDefName} initialized successfully!`);
    console.log(`Transaction: ${txSignature}`);
    
  } catch (error: any) {
    console.error(`❌ Failed to initialize:`, error.message);
    if (error.logs) {
      console.error(`Transaction logs:`, error.logs);
    }
    console.error(`Full error:`, error);
  }
}

testV1Initialization();
