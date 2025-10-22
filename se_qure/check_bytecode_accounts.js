const { Connection, PublicKey } = require('@solana/web3.js');

async function checkBytecodeAccounts() {
  const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777', 'confirmed');
  
  console.log('=== CHECKING MPC BYTECODE ACCOUNTS ===');
  
  // Get the computation definition accounts from current deployment
  const compDefAccounts = [
    { name: 'survey_analytics', addr: 'CzorH6VpK7tPbNDDxf2PXTNEq5ypWs5SP6WA2v79U7WG' },
    { name: 'quiz_evaluation', addr: 'DTVVdFzASBDpyiDRB6QSfutjPrXSNFxurDby29nSQ8PC' },
    { name: 'analytics_computation', addr: '7SHSVATHZHzyHDqSKZg3aBHdzCn6xN7rHX3JPxDxEX8q' },
    { name: 'quiz_threshold_check', addr: '7XJ98ikbcNYSK7gCfvRJDVvaX1ZBCb2a3aRDjC9gzfbx' }
  ];
  
  const arciumProgramId = new PublicKey('BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6');
  
  for (const compDef of compDefAccounts) {
    const compDefAccount = new PublicKey(compDef.addr);
    console.log(`\n=== ${compDef.name.toUpperCase()} ===`);
    console.log(`Computation Definition: ${compDef.addr}`);
    
    // Check main computation definition account
    const mainAccount = await connection.getAccountInfo(compDefAccount);
    console.log(`  Main account exists: ${!!mainAccount}`);
    if (mainAccount) {
      console.log(`  Data length: ${mainAccount.data.length} bytes`);
      console.log(`  Owner: ${mainAccount.owner.toBase58()}`);
    }
    
    // Check for bytecode accounts (usually 0-4)
    let bytecodeCount = 0;
    for (let i = 0; i < 5; i++) {
      const [bytecodeAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from('ComputationDefinitionRaw'), compDefAccount.toBuffer(), Buffer.from([i])],
        arciumProgramId
      );
      const accountInfo = await connection.getAccountInfo(bytecodeAccount);
      if (accountInfo) {
        bytecodeCount++;
        console.log(`  ✅ Bytecode account ${i}: EXISTS (${accountInfo.data.length} bytes)`);
      } else {
        console.log(`  ❌ Bytecode account ${i}: MISSING`);
      }
    }
    
    console.log(`  📊 Total bytecode accounts: ${bytecodeCount}/5`);
    
    if (bytecodeCount === 0) {
      console.log(`  🚨 ISSUE: No bytecode accounts found for ${compDef.name}!`);
    }
  }
  
  console.log('\n=== SUMMARY ===');
  console.log('If any computation definition shows 0 bytecode accounts,');
  console.log('that explains the ComputationDefinitionNotCompleted error.');
}

checkBytecodeAccounts().catch(console.error);
