const { PublicKey } = require('@solana/web3.js');

// Program ID
const PROGRAM_ID = new PublicKey('5V48eqzYSLNhAKPmFeR3483npbMNgB12RUhKhC4Nhvtu');

// Function to derive PDA for different naming schemes
function derivePDA(name) {
    const [pda] = PublicKey.findProgramAddressSync(
        [Buffer.from(name)],
        PROGRAM_ID
    );
    return pda;
}

// Test different naming schemes
const schemes = [
    'survey_analytics',
    'survey_analytics_v1', 
    'survey_analytics_v2',
    'survey_analytics_v3',
    'quiz_evaluation',
    'quiz_evaluation_v1',
    'quiz_evaluation_v2', 
    'quiz_evaluation_v3',
    'analytics_computation',
    'analytics_computation_v1',
    'analytics_computation_v2',
    'analytics_computation_v3',
    'quiz_threshold_check',
    'quiz_threshold_check_v1',
    'quiz_threshold_check_v2',
    'quiz_threshold_check_v3'
];

console.log('Testing PDA derivation for different naming schemes:');
console.log('='.repeat(80));

schemes.forEach(name => {
    const pda = derivePDA(name);
    console.log(`${name.padEnd(30)} -> ${pda.toString()}`);
});

// Check if any v3 names derive to the same address as v1 names
console.log('\nChecking for address collisions:');
console.log('='.repeat(50));

const v1Addresses = {};
const v3Addresses = {};

schemes.forEach(name => {
    const pda = derivePDA(name);
    if (name.includes('_v1')) {
        const baseName = name.replace('_v1', '');
        v1Addresses[baseName] = pda.toString();
    }
    if (name.includes('_v3')) {
        const baseName = name.replace('_v3', '');
        v3Addresses[baseName] = pda.toString();
    }
});

Object.keys(v1Addresses).forEach(baseName => {
    if (v3Addresses[baseName]) {
        if (v1Addresses[baseName] === v3Addresses[baseName]) {
            console.log(`❌ COLLISION: ${baseName}_v1 and ${baseName}_v3 have same address: ${v1Addresses[baseName]}`);
        } else {
            console.log(`✅ ${baseName}_v1: ${v1Addresses[baseName]}`);
            console.log(`✅ ${baseName}_v3: ${v3Addresses[baseName]}`);
        }
    }
});
