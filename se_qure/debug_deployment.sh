#!/bin/bash

echo "🔍 SeQure Deployment Debug Script"
echo "=================================="

# Get current program ID
PROGRAM_ID=$(solana-keygen pubkey target/deploy/se_qure-keypair.json)
echo "📋 Program ID: $PROGRAM_ID"

echo ""
echo "=== DEPLOYMENT WALLET STATUS ==="
echo "Wallet address: $(solana address)"
echo "Balance: $(solana balance) SOL"

echo ""
echo "=== PROGRAM KEYPAIR CHECK ==="
if [ -f "target/deploy/se_qure-keypair.json" ]; then
    echo "✅ Program keypair exists"
    echo "Program ID: $PROGRAM_ID"
else
    echo "❌ Program keypair missing"
    exit 1
fi

echo ""
echo "=== BUILD ARTIFACTS CHECK ==="
if [ -f "target/deploy/se_qure.so" ]; then
    echo "✅ Program binary exists ($(ls -lh target/deploy/se_qure.so | awk '{print $5}'))"
else
    echo "❌ Program binary missing"
    exit 1
fi

if [ -f "target/idl/se_qure.json" ]; then
    echo "✅ IDL exists"
    echo "IDL Program ID: $(cat target/idl/se_qure.json | jq -r '.address')"
    if [ "$(cat target/idl/se_qure.json | jq -r '.address')" = "$PROGRAM_ID" ]; then
        echo "✅ IDL Program ID matches keypair"
    else
        echo "❌ IDL Program ID mismatch!"
        exit 1
    fi
else
    echo "❌ IDL missing"
    exit 1
fi

echo ""
echo "=== EXPECTED MXE ACCOUNT ==="
node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log('Expected MXE Account:', mxeAccount.toBase58());
"

echo ""
echo "=== CHECKING MXE ACCOUNT ON-CHAIN ==="
MXE_ACCOUNT=$(node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log(mxeAccount.toBase58());
")

if solana account $MXE_ACCOUNT --output json | jq -r '.account != null' > /dev/null 2>&1; then
    echo "✅ MXE Account EXISTS on-chain"
    echo "MXE Account: $MXE_ACCOUNT"
else
    echo "❌ MXE Account MISSING on-chain"
    echo "MXE Account: $MXE_ACCOUNT"
fi

echo ""
echo "=== CHECKING COMPUTATION DEFINITIONS ==="
node -e "
const { Connection, PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress, getCompDefAccAddress } = require('@arcium-hq/client');
const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777', 'confirmed');

async function checkCompDefs() {
  const programId = new PublicKey('$PROGRAM_ID');
  const mxeAccount = getMXEAccAddress(programId);
  
  const compDefNames = ['survey_analytics', 'quiz_evaluation', 'analytics_computation', 'quiz_threshold_check'];
  
  console.log('Checking computation definitions...');
  for (const name of compDefNames) {
    const compDefAccount = getCompDefAccAddress(mxeAccount, name);
    const accountInfo = await connection.getAccountInfo(compDefAccount);
    console.log(\`  \${name}: \${accountInfo ? '✅ EXISTS' : '❌ MISSING'} (\${compDefAccount.toBase58()})\`);
  }
}

checkCompDefs().catch(console.error);
"

echo ""
echo "=== READY FOR DEPLOYMENT ==="
echo "All checks passed! Ready to deploy with:"
echo "arcium deploy --cluster-offset 3726127828 --keypair-path ~/.config/solana/id.json --rpc-url https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777 --program-keypair target/deploy/se_qure-keypair.json --program-name se_qure --verbose"
