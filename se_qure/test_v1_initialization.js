const { Connection, PublicKey, Keypair, Transaction } = require('@solana/web3.js');
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

// Function to derive PDA for v1 names
function deriveV1PDA(name) {
    const [pda] = PublicKey.findProgramAddressSync(
        [Buffer.from(name)],
        PROGRAM_ID
    );
    return pda;
}

// Test initializing with v1 names
async function testV1Initialization() {
    try {
        console.log('Testing initialization with v1 names...');
        
        // Derive v1 addresses
        const surveyAnalyticsV1 = deriveV1PDA('survey_analytics_v1');
        const quizEvaluationV1 = deriveV1PDA('quiz_evaluation_v1');
        const analyticsComputationV1 = deriveV1PDA('analytics_computation_v1');
        const quizThresholdV1 = deriveV1PDA('quiz_threshold_check_v1');
        
        console.log('V1 addresses:');
        console.log(`survey_analytics_v1: ${surveyAnalyticsV1.toString()}`);
        console.log(`quiz_evaluation_v1: ${quizEvaluationV1.toString()}`);
        console.log(`analytics_computation_v1: ${analyticsComputationV1.toString()}`);
        console.log(`quiz_threshold_v1: ${quizThresholdV1.toString()}`);
        
        // Try to initialize survey_analytics_v1
        console.log('\nTrying to initialize survey_analytics_v1...');
        
        const transaction = new Transaction();
        const instruction = await program.methods
            .initSurveyAnalyticsCompDef()
            .accounts({
                compDefAccount: surveyAnalyticsV1,
                payer: wallet.publicKey,
                systemProgram: PublicKey.default,
            })
            .instruction();
        
        transaction.add(instruction);
        transaction.feePayer = wallet.publicKey;
        
        // Simulate the transaction
        const simulation = await connection.simulateTransaction(transaction);
        console.log('Simulation result:', simulation);
        
        if (simulation.value.err) {
            console.log('❌ Simulation failed:', simulation.value.err);
        } else {
            console.log('✅ Simulation succeeded');
        }
        
    } catch (error) {
        console.error('Error testing v1 initialization:', error.message);
        if (error.logs) {
            console.log('Error logs:', error.logs);
        }
    }
}

testV1Initialization();
