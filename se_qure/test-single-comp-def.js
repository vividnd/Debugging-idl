const { Connection, PublicKey, Keypair } = require('@solana/web3.js');
const { AnchorProvider, Wallet } = require('@coral-xyz/anchor');
const { Program } = require('@coral-xyz/anchor');
const { getMXEAccAddress, getCompDefAccAddress } = require('@arcium-hq/client');
const fs = require('fs');

async function testSingleCompDef() {
  try {
    console.log('🧪 Testing Single Computation Definition Initialization...');
    
    const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777', 'confirmed');
    const wallet = new Wallet(Keypair.generate());
    const provider = new AnchorProvider(connection, wallet, {});
    
    const idl = JSON.parse(fs.readFileSync('target/idl/se_qure.json', 'utf8'));
    const program = new Program(idl, provider);
    
    const programId = new PublicKey('7kwSwGW6RUHJJTtK4rKVkizmJnjCMSd97t36xjextr79');
    const mxeAccount = getMXEAccAddress(programId);
    const compDefAccount = getCompDefAccAddress(mxeAccount, 'survey_analytics');
    
    console.log('Program ID:', programId.toBase58());
    console.log('MXE Account:', mxeAccount.toBase58());
    console.log('Comp Def Account:', compDefAccount.toBase58());
    
    // Create accounts object
    const accounts = {
      payer: wallet.publicKey,
      mxeAccount: mxeAccount,
      compDefAccount: compDefAccount,
      arciumProgram: new PublicKey('BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6'),
      systemProgram: new PublicKey('11111111111111111111111111111111')
    };
    
    console.log('📋 Attempting to initialize survey_analytics...');
    
    // Try to call the method
    const txSignature = await program.methods
      .initSurveyAnalyticsCompDef()
      .accounts(accounts)
      .signers([wallet.payer])
      .rpc({ 
        skipPreflight: true, 
        commitment: "confirmed",
        preflightCommitment: "confirmed"
      });
    
    console.log('✅ Success! Transaction signature:', txSignature);
    
  } catch (error) {
    console.error('❌ Error:', error.message);
    console.error('Full error:', error);
    
    if (error.logs) {
      console.error('Transaction logs:', error.logs);
    }
  }
}

testSingleCompDef().catch(console.error);
