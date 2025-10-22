import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SeQure } from "./target/types/se_qure";

async function debugMethods() {
  console.log("🔍 DEBUGGING AVAILABLE METHODS");
  console.log("=" .repeat(40));
  
  try {
    // Setup connection
    const connection = new anchor.web3.Connection("https://api.devnet.solana.com", "confirmed");
    
    // Load wallet
    const payer = anchor.web3.Keypair.generate(); // We don't need a real wallet for this
    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(payer),
      { commitment: "confirmed" }
    );
    anchor.setProvider(provider);
    
    const program = anchor.workspace.SeQure as Program<SeQure>;
    console.log(`Program ID: ${program.programId.toBase58()}`);
    
    // Check what methods are available
    console.log("\n📋 Available methods:");
    const methods = program.methods;
    console.log("Methods object keys:", Object.keys(methods));
    
    // Try to access specific methods
    console.log("\n🔍 Checking specific methods:");
    
    try {
      console.log("initSurveyAnalyticsCompDef:", typeof methods.initSurveyAnalyticsCompDef);
    } catch (e) {
      console.log("initSurveyAnalyticsCompDef: ERROR -", e.message);
    }
    
    try {
      console.log("initQuizEvaluationCompDef:", typeof methods.initQuizEvaluationCompDef);
    } catch (e) {
      console.log("initQuizEvaluationCompDef: ERROR -", e.message);
    }
    
    try {
      console.log("initAnalyticsComputationCompDef:", typeof methods.initAnalyticsComputationCompDef);
    } catch (e) {
      console.log("initAnalyticsComputationCompDef: ERROR -", e.message);
    }
    
    try {
      console.log("initQuizThresholdCheckCompDef:", typeof methods.initQuizThresholdCheckCompDef);
    } catch (e) {
      console.log("initQuizThresholdCheckCompDef: ERROR -", e.message);
    }
    
    // Check the program interface
    console.log("\n🔍 Program interface:");
    console.log("Program methods:", Object.keys(program.methods));
    
  } catch (error) {
    console.error("❌ Error:", error.message);
  }
}

debugMethods().catch(console.error);