const { Connection, PublicKey, Keypair } = require('@solana/web3.js');
const { Program, AnchorProvider } = require('@coral-xyz/anchor');
const fs = require('fs');
const path = require('path');

// Load the IDL
const idl = JSON.parse(fs.readFileSync(path.join(__dirname, 'target/idl/se_qure.json'), 'utf8'));

// Connection and provider
const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777');
const wallet = Keypair.fromSecretKey(require(require('os').homedir() + '/.config/solana/id.json'));
const provider = new AnchorProvider(connection, { publicKey: wallet.publicKey, signTransaction: async (tx) => tx, signAllTransactions: async (txs) => txs }, {});
const program = new Program(idl, provider);

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
    'quiz_evaluation_v3'
];

console.log('Testing PDA derivation for different naming schemes:');
console.log('='.repeat(80));

schemes.forEach(name => {
    const pda = derivePDA(name);
    console.log(`${name.padEnd(25)} -> ${pda.toString()}`);
});

// Test if we can call the initialization method
async function testInitialization() {
    try {
        console.log('\nTesting initialization method availability...');
        
        // Check if the method exists
        if (program.methods.initSurveyAnalyticsCompDef) {
            console.log('✅ initSurveyAnalyticsCompDef method exists');
        } else {
            console.log('❌ initSurveyAnalyticsCompDef method not found');
        }
        
        // Try to get the method signature
        const method = program.methods.initSurveyAnalyticsCompDef;
        console.log('Method signature:', method.toString());
        
    } catch (error) {
        console.error('Error testing initialization:', error.message);
    }
}

testInitialization();
