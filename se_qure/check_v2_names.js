const { getCompDefAccOffset, getCompDefAccAddress } = require('@arcium-hq/client');
const { PublicKey } = require('@solana/web3.js');

// Current v1 names and their addresses
const v1Names = [
  'survey_analytics',
  'quiz_evaluation', 
  'analytics_computation',
  'quiz_threshold_check'
];

// Current v2 names and their addresses  
const v2Names = [
  'survey_analytics_v2',
  'quiz_evaluation_v2',
  'analytics_computation_v2', 
  'quiz_threshold_check_v2'
];

// Proposed completely different names
const newNames = [
  'survey_analytics_v3',
  'quiz_evaluation_v3',
  'analytics_computation_v3',
  'quiz_threshold_check_v3'
];

// Alternative completely different names
const altNames = [
  'survey_analytics_fixed',
  'quiz_evaluation_fixed',
  'analytics_computation_fixed',
  'quiz_threshold_check_fixed'
];

// Another alternative
const alt2Names = [
  'survey_analytics_new',
  'quiz_evaluation_new', 
  'analytics_computation_new',
  'quiz_threshold_check_new'
];

const programId = new PublicKey('FRX31dtpjrY8ddjkBMzfrciMVCWAFcP5Sw5Szg2gLJc8');

function checkNames(names, label) {
  console.log(`\n🔍 ${label}:`);
  console.log('=' .repeat(50));
  
  for (const name of names) {
    try {
      const offset = getCompDefAccOffset(name);
      const account = getCompDefAccAddress(programId, Buffer.from(offset).readUInt32LE());
      console.log(`${name}:`);
      console.log(`  Offset: ${Buffer.from(offset).toString('hex')}`);
      console.log(`  Account: ${account.toBase58()}`);
    } catch (error) {
      console.log(`${name}: ERROR - ${error.message}`);
    }
  }
}

console.log('🔍 COMPUTATION DEFINITION NAME ANALYSIS');
console.log('=' .repeat(60));
console.log('Program ID:', programId.toBase58());

checkNames(v1Names, 'V1 Names (Current)');
checkNames(v2Names, 'V2 Names (Current)');
checkNames(newNames, 'V3 Names (Proposed)');
checkNames(altNames, 'Fixed Names (Alternative)');
checkNames(alt2Names, 'New Names (Alternative)');

console.log('\n📊 SUMMARY:');
console.log('=' .repeat(20));
console.log('Look for names that generate COMPLETELY DIFFERENT account addresses.');
console.log('These will avoid any conflicts with existing v1 accounts.');
console.log('\nRecommendation: Use v3 names or completely different naming scheme.');
