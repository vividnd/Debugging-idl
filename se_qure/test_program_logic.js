const { Connection, PublicKey, Keypair, Transaction } = require('@solana/web3.js');
const { Program, AnchorProvider } = require('@coral-xyz/anchor');
const fs = require('fs');
const path = require('path');

// Load the IDL
const idl = JSON.parse(fs.readFileSync(path.join(__dirname, 'target/idl/se_qure.json'), 'utf8'));

// Connection
const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777');

// Program ID
const PROGRAM_ID = new PublicKey('5V48eqzYSLNhAKPmFeR3483npbMNgB12RUhKhC4Nhvtu');

// Function to derive PDA
function derivePDA(name) {
    const [pda] = PublicKey.findProgramAddressSync(
        [Buffer.from(name)],
        PROGRAM_ID
    );
    return pda;
}

// Test what the program expects by trying to initialize with v1 names
async function testV1Initialization() {
    try {
        console.log('Testing if program expects v1 names...');
        
        // Derive v1 addresses
        const surveyAnalyticsV1 = derivePDA('survey_analytics_v1');
        const quizEvaluationV1 = derivePDA('quiz_evaluation_v1');
        const analyticsComputationV1 = derivePDA('analytics_computation_v1');
        const quizThresholdV1 = derivePDA('quiz_threshold_check_v1');
        
        console.log('V1 addresses:');
        console.log(`survey_analytics_v1: ${surveyAnalyticsV1.toString()}`);
        console.log(`quiz_evaluation_v1: ${quizEvaluationV1.toString()}`);
        console.log(`analytics_computation_v1: ${analyticsComputationV1.toString()}`);
        console.log(`quiz_threshold_v1: ${quizThresholdV1.toString()}`);
        
        // Check if these accounts exist
        const accounts = await connection.getMultipleAccountsInfo([
            surveyAnalyticsV1,
            quizEvaluationV1,
            analyticsComputationV1,
            quizThresholdV1
        ]);
        
        console.log('\nV1 account status:');
        accounts.forEach((account, index) => {
            const names = ['survey_analytics_v1', 'quiz_evaluation_v1', 'analytics_computation_v1', 'quiz_threshold_v1'];
            if (account) {
                console.log(`✅ ${names[index]}: EXISTS (${account.data.length} bytes)`);
            } else {
                console.log(`❌ ${names[index]}: NOT FOUND`);
            }
        });
        
    } catch (error) {
        console.error('Error testing v1 initialization:', error.message);
    }
}

// Test what the program expects by trying to initialize with v3 names
async function testV3Initialization() {
    try {
        console.log('\nTesting if program expects v3 names...');
        
        // Derive v3 addresses
        const surveyAnalyticsV3 = derivePDA('survey_analytics_v3');
        const quizEvaluationV3 = derivePDA('quiz_evaluation_v3');
        const analyticsComputationV3 = derivePDA('analytics_computation_v3');
        const quizThresholdV3 = derivePDA('quiz_threshold_check_v3');
        
        console.log('V3 addresses:');
        console.log(`survey_analytics_v3: ${surveyAnalyticsV3.toString()}`);
        console.log(`quiz_evaluation_v3: ${quizEvaluationV3.toString()}`);
        console.log(`analytics_computation_v3: ${analyticsComputationV3.toString()}`);
        console.log(`quiz_threshold_v3: ${quizThresholdV3.toString()}`);
        
        // Check if these accounts exist
        const accounts = await connection.getMultipleAccountsInfo([
            surveyAnalyticsV3,
            quizEvaluationV3,
            analyticsComputationV3,
            quizThresholdV3
        ]);
        
        console.log('\nV3 account status:');
        accounts.forEach((account, index) => {
            const names = ['survey_analytics_v3', 'quiz_evaluation_v3', 'analytics_computation_v3', 'quiz_threshold_v3'];
            if (account) {
                console.log(`✅ ${names[index]}: EXISTS (${account.data.length} bytes)`);
            } else {
                console.log(`❌ ${names[index]}: NOT FOUND`);
            }
        });
        
    } catch (error) {
        console.error('Error testing v3 initialization:', error.message);
    }
}

async function main() {
    await testV1Initialization();
    await testV3Initialization();
}

main();
