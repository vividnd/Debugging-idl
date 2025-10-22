const { Connection, PublicKey } = require('@solana/web3.js');
const { getCompDefAccAddress, getCompDefAccOffset, getMXEAccAddress } = require('@arcium-hq/client');

async function verifyOnChain() {
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  
  console.log('🔍 VERIFYING ON-CHAIN COMPUTATION DEFINITIONS');
  console.log('=' .repeat(60));
  
  // Current program ID from the error logs
  const PROGRAM_ID = new PublicKey('FRX31dtpjrY8ddjkBMzfrciMVCWAFcP5Sw5Szg2gLJc8');
  
  console.log(`\n📋 Program ID: ${PROGRAM_ID.toBase58()}`);
  
  // Get MXE account
  const mxeAccount = getMXEAccAddress(PROGRAM_ID);
  console.log(`📋 MXE Account: ${mxeAccount.toBase58()}`);
  
  // Check if MXE exists
  const mxeInfo = await connection.getAccountInfo(mxeAccount);
  console.log(`📋 MXE Account exists: ${!!mxeInfo}`);
  
  if (!mxeInfo) {
    console.log('❌ MXE account not found! This needs to be initialized first.');
    return;
  }
  
  console.log('\n🔍 CHECKING COMPUTATION DEFINITIONS:');
  console.log('=' .repeat(40));
  
  // Check both versions (with and without _v2)
  const compDefNames = [
    'survey_analytics',
    'survey_analytics_v2',
    'quiz_evaluation', 
    'quiz_evaluation_v2',
    'analytics_computation',
    'analytics_computation_v2',
    'quiz_threshold_check',
    'quiz_threshold_check_v2'
  ];
  
  for (const name of compDefNames) {
    try {
      console.log(`\n📋 Checking: ${name}`);
      
      // Get the offset and account address
      const offset = getCompDefAccOffset(name);
      const compDefAccount = getCompDefAccAddress(PROGRAM_ID, Buffer.from(offset).readUInt32LE());
      
      console.log(`   Offset: ${offset.toString('hex')}`);
      console.log(`   Account: ${compDefAccount.toBase58()}`);
      
      // Check if account exists
      const accountInfo = await connection.getAccountInfo(compDefAccount);
      console.log(`   Exists: ${!!accountInfo}`);
      
      if (accountInfo) {
        console.log(`   Data length: ${accountInfo.data.length} bytes`);
        console.log(`   Owner: ${accountInfo.owner.toBase58()}`);
        
        // Check for bytecode accounts
        let bytecodeCount = 0;
        const arciumProgramId = new PublicKey('BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6');
        
        for (let i = 0; i < 5; i++) {
          const [bytecodeAccount] = PublicKey.findProgramAddressSync(
            [Buffer.from('ComputationDefinitionRaw'), compDefAccount.toBuffer(), Buffer.from([i])],
            arciumProgramId
          );
          const bytecodeInfo = await connection.getAccountInfo(bytecodeAccount);
          if (bytecodeInfo) {
            bytecodeCount++;
          }
        }
        
        console.log(`   Bytecode accounts: ${bytecodeCount}/5`);
        
        if (bytecodeCount === 0) {
          console.log(`   ⚠️  WARNING: No bytecode accounts found!`);
        } else {
          console.log(`   ✅ Ready for use!`);
        }
      }
      
    } catch (error) {
      console.log(`   ❌ Error: ${error.message}`);
    }
  }
  
  console.log('\n📊 SUMMARY:');
  console.log('=' .repeat(20));
  console.log('Look for computation definitions that exist AND have bytecode accounts.');
  console.log('These are the ones that are properly initialized and ready to use.');
}

verifyOnChain().catch(console.error);
