const { Connection, PublicKey } = require('@solana/web3.js');

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

// Test what the program expects by comparing v1 vs v3 addresses
async function compareV1VsV3() {
    try {
        console.log('Comparing v1 vs v3 addresses...');
        
        // Derive v1 addresses
        const surveyAnalyticsV1 = derivePDA('survey_analytics_v1');
        const quizEvaluationV1 = derivePDA('quiz_evaluation_v1');
        const analyticsComputationV1 = derivePDA('analytics_computation_v1');
        const quizThresholdV1 = derivePDA('quiz_threshold_check_v1');
        
        // Derive v3 addresses
        const surveyAnalyticsV3 = derivePDA('survey_analytics_v3');
        const quizEvaluationV3 = derivePDA('quiz_evaluation_v3');
        const analyticsComputationV3 = derivePDA('analytics_computation_v3');
        const quizThresholdV3 = derivePDA('quiz_threshold_check_v3');
        
        console.log('\nV1 addresses:');
        console.log(`survey_analytics_v1: ${surveyAnalyticsV1.toString()}`);
        console.log(`quiz_evaluation_v1: ${quizEvaluationV1.toString()}`);
        console.log(`analytics_computation_v1: ${analyticsComputationV1.toString()}`);
        console.log(`quiz_threshold_v1: ${quizThresholdV1.toString()}`);
        
        console.log('\nV3 addresses:');
        console.log(`survey_analytics_v3: ${surveyAnalyticsV3.toString()}`);
        console.log(`quiz_evaluation_v3: ${quizEvaluationV3.toString()}`);
        console.log(`analytics_computation_v3: ${analyticsComputationV3.toString()}`);
        console.log(`quiz_threshold_v3: ${quizThresholdV3.toString()}`);
        
        // Check if any of these accounts exist
        console.log('\nChecking account existence...');
        const allAddresses = [
            surveyAnalyticsV1, quizEvaluationV1, analyticsComputationV1, quizThresholdV1,
            surveyAnalyticsV3, quizEvaluationV3, analyticsComputationV3, quizThresholdV3
        ];
        
        const accounts = await connection.getMultipleAccountsInfo(allAddresses);
        const names = [
            'survey_analytics_v1', 'quiz_evaluation_v1', 'analytics_computation_v1', 'quiz_threshold_v1',
            'survey_analytics_v3', 'quiz_evaluation_v3', 'analytics_computation_v3', 'quiz_threshold_v3'
        ];
        
        accounts.forEach((account, index) => {
            if (account) {
                console.log(`✅ ${names[index]}: EXISTS (${account.data.length} bytes)`);
            } else {
                console.log(`❌ ${names[index]}: NOT FOUND`);
            }
        });
        
        // The key question: if the program is using v3 names in the Rust code,
        // but we're getting ConstraintSeeds errors, it might mean:
        // 1. The program is still using v1 names internally
        // 2. There's a mismatch between what we're providing and what it expects
        
        console.log('\nAnalysis:');
        console.log('- Rust code uses v3 names');
        console.log('- Program was rebuilt after v3 changes');
        console.log('- But we get ConstraintSeeds errors when trying to initialize v3');
        console.log('- This suggests the program might still be expecting v1 addresses');
        
    } catch (error) {
        console.error('Error comparing v1 vs v3:', error.message);
    }
}

compareV1VsV3();
