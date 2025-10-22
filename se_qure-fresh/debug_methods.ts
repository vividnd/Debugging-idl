import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Connection } from "@solana/web3.js";
import { SeQure } from "./target/types/se_qure";
import * as fs from "fs";
import * as os from "os";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const WALLET_PATH = `${os.homedir()}/.config/solana/id.json`;

function readKeypair(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}

async function debugMethods() {
  console.log("🔍 Debugging program methods...");
  
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
  
  // Check if methods exist
  console.log("\n📋 Available methods:");
  const methods = Object.keys(program.methods);
  methods.forEach(method => {
    console.log(`  - ${method}`);
  });
  
  // Check specific init methods
  const initMethods = methods.filter(m => m.includes('init') && m.includes('CompDef'));
  console.log(`\n🎯 Init CompDef methods found: ${initMethods.length}`);
  initMethods.forEach(method => {
    console.log(`  - ${method}`);
  });
  
  // Try to access a specific method
  if (program.methods.initSurveyAnalyticsCompDef) {
    console.log("\n✅ initSurveyAnalyticsCompDef method exists");
    console.log("Method type:", typeof program.methods.initSurveyAnalyticsCompDef);
  } else {
    console.log("\n❌ initSurveyAnalyticsCompDef method does not exist");
  }
}

debugMethods().catch(console.error);
