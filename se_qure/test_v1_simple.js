const { Connection, PublicKey } = require('@solana/web3.js');

// Connection
const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777');

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

// Test what addresses the program expects by checking the deployed bytecode
async function checkProgramExpectations() {
    try {
        console.log('Checking what the deployed program expects...');
        
        // Get the program account info
        const programAccount = await connection.getAccountInfo(PROGRAM_ID);
        if (programAccount) {
            console.log(`✅ Program exists: ${programAccount.data.length} bytes`);
            console.log(`Owner: ${programAccount.owner.toString()}`);
            console.log(`Executable: ${programAccount.executable}`);
        } else {
            console.log('❌ Program not found');
            return;
        }
        
        // Check if we can find any clues about what addresses it expects
        // by looking at the program's data
        console.log('\nProgram data (first 100 bytes):');
        const data = programAccount.data.slice(0, 100);
        console.log(data.toString('hex'));
        
        // Check if there are any hardcoded addresses in the program
        console.log('\nLooking for hardcoded addresses in program data...');
        const fullData = programAccount.data;
        
        // Look for patterns that might be addresses (32-byte sequences)
        const addresses = [];
        for (let i = 0; i < fullData.length - 32; i++) {
            const chunk = fullData.slice(i, i + 32);
            // Check if this looks like a valid public key
            if (chunk.every(byte => byte >= 0 && byte <= 255)) {
                try {
                    const pubkey = new PublicKey(chunk);
                    addresses.push(pubkey.toString());
                } catch (e) {
                    // Not a valid public key
                }
            }
        }
        
        console.log(`Found ${addresses.length} potential addresses in program data`);
        if (addresses.length > 0) {
            console.log('First 10 addresses:');
            addresses.slice(0, 10).forEach((addr, i) => {
                console.log(`${i + 1}: ${addr}`);
            });
        }
        
    } catch (error) {
        console.error('Error checking program expectations:', error.message);
    }
}

checkProgramExpectations();
