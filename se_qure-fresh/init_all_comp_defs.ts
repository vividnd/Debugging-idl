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

async function accountExists(connection: Connection, address: PublicKey): Promise<boolean> {
  try {
    const accountInfo = await connection.getAccountInfo(address);
    return accountInfo !== null;
  } catch (error) {
    return false;
  }
}

async function initializeCompDef(
  program: Program<SeQure>,
  connection: Connection,
  payer: anchor.web3.Keypair,
  mxeAccount: PublicKey,
  compDefName: string,
  methodName: string
): Promise<boolean> {
  try {
    console.log(`\n🔄 Initializing ${compDefName}...`);
    
    const offset = getCompDefAccOffset(compDefName);
    const compDefAccount = getCompDefAccAddress(program.programId, Buffer.from(offset).readUInt32LE());
    console.log(`   Comp Def Account: ${compDefAccount.toBase58()}`);
    
    // Check if already exists
    const exists = await accountExists(connection, compDefAccount);
    if (exists) {
      console.log(`   ✅ ${compDefName} already initialized`);
      return true;
    }
    
    // Prepare accounts
    const accounts = {
      payer: payer.publicKey,
      mxeAccount,
      compDefAccount,
      arciumProgram: ARCIUM_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    };
    
    // Get recent blockhash
    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    
    // Create instruction using the method name
    const instruction = await (program.methods as any)[methodName]()
      .accounts(accounts)
      .instruction();
    
    console.log("   ✅ Instruction created successfully");
    
    // Create and send transaction
    const transaction = new Transaction();
    transaction.add(instruction);
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    // Sign and send
    transaction.sign(payer);
    
    const signature = await connection.sendTransaction(transaction, [payer], {
      skipPreflight: true
    });
    
    console.log(`   ✅ Transaction sent successfully!`);
    console.log(`   📝 Signature: ${signature}`);
    
    // Wait for confirmation
    await connection.confirmTransaction(signature, 'confirmed');
    console.log(`   ✅ Transaction confirmed!`);
    
    return true;
    
  } catch (error) {
    console.error(`   ❌ Failed to initialize ${compDefName}:`, error.message);
    return false;
  }
}

async function initAllCompDefs() {
  console.log("🚀 Initializing All Computation Definitions");
  console.log("=============================================");
  
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
  
  // Define computation definitions to initialize
  const compDefs = [
    { name: 'quiz_evaluation', method: 'initQuizEvaluationCompDef' },
    { name: 'analytics_computation', method: 'initAnalyticsComputationCompDef' },
    { name: 'quiz_threshold_check', method: 'initQuizThresholdCheckCompDef' }
  ];
  
  console.log("\n🎯 Initializing computation definitions...");
  
  let successCount = 0;
  for (const compDef of compDefs) {
    const success = await initializeCompDef(
      program,
      connection,
      payer,
      mxeAccount,
      compDef.name,
      compDef.method
    );
    if (success) successCount++;
  }
  
  console.log(`\n📊 Summary: ${successCount}/${compDefs.length} computation definitions initialized successfully`);
  
  if (successCount === compDefs.length) {
    console.log("🎉 All computation definitions initialized successfully!");
  } else {
    console.log("⚠️  Some computation definitions failed to initialize.");
  }
}

initAllCompDefs().catch(console.error);
