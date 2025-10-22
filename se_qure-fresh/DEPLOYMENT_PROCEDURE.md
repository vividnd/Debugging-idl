# SeQure Arcium MXE Deployment Procedure

## Complete Step-by-Step Deployment Process

### 1. **Clean Previous Deployment**
```bash
# Delete old IDL to force regeneration
rm -f target/idl/se_qure.json

# Clear all build artifacts
cargo clean

# Close previous program (if exists)
solana program close <OLD_PROGRAM_ID> --url devnet --bypass-warning
```

### 2. **Sync Arcium Keys and Rebuild**
```bash
# Sync Arcium keys (updates program ID if needed)
arcium keys sync

# Rebuild with arcium (regenerates IDL with synced program ID)
arcium build
```

### 3. **Deploy**
```bash
# Deploy to devnet with debugging
arcium deploy --cluster-offset 3726127828 --keypair-path ~/.config/solana/id.json --rpc-url https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777 --program-keypair target/deploy/se_qure-keypair.json --program-name se_qure --verbose
```

### 3.1. **Debug Deployment Process**
```bash
# Check deployment wallet balance before deployment
echo "=== DEPLOYMENT WALLET STATUS ==="
solana balance
echo "Wallet address: $(solana address)"

# Check program keypair exists
echo "=== PROGRAM KEYPAIR CHECK ==="
ls -la target/deploy/se_qure-keypair.json
echo "Program ID: $(solana-keygen pubkey target/deploy/se_qure-keypair.json)"

# Check build artifacts
echo "=== BUILD ARTIFACTS CHECK ==="
ls -la target/deploy/se_qure.so
ls -la target/idl/se_qure.json

# Deploy with verbose logging
echo "=== STARTING DEPLOYMENT ==="
arcium deploy --cluster-offset 3726127828 --keypair-path ~/.config/solana/id.json --rpc-url https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777 --program-keypair target/deploy/se_qure-keypair.json --program-name se_qure --verbose

# Check deployment results
echo "=== DEPLOYMENT RESULTS ==="
echo "Program ID: $(solana-keygen pubkey target/deploy/se_qure-keypair.json)"
echo "Program exists on-chain: $(solana account $(solana-keygen pubkey target/deploy/se_qure-keypair.json) --output json | jq -r '.account != null')"
```

### 4. **Initialize Computation Definitions**
```bash
# Run initialization script with debugging
ANCHOR_PROVIDER_URL=https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777 ANCHOR_WALLET=~/.config/solana/id.json npx ts-node scripts/init_comp_defs.ts
```

### 4.1. **Debug Computation Definition Initialization**
```bash
# Check MXE account exists before initialization
echo "=== MXE ACCOUNT CHECK ==="
PROGRAM_ID=$(solana-keygen pubkey target/deploy/se_qure-keypair.json)
echo "Program ID: $PROGRAM_ID"

# Derive expected MXE account
node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log('Expected MXE Account:', mxeAccount.toBase58());
"

# Check if MXE account exists on-chain
echo "=== CHECKING MXE ACCOUNT ON-CHAIN ==="
MXE_ACCOUNT=$(node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log(mxeAccount.toBase58());
")
solana account $MXE_ACCOUNT --output json | jq -r '.account != null' && echo "MXE Account EXISTS" || echo "MXE Account MISSING"

# Run initialization with detailed logging
echo "=== INITIALIZING COMPUTATION DEFINITIONS ==="
ANCHOR_PROVIDER_URL=https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777 ANCHOR_WALLET=~/.config/solana/id.json npx ts-node scripts/init_comp_defs.ts

# Verify computation definitions were created
echo "=== VERIFYING COMPUTATION DEFINITIONS ==="
node -e "
const { Connection, PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress, getCompDefAccAddress } = require('@arcium-hq/client');
const connection = new Connection('https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777', 'confirmed');

async function checkCompDefs() {
  const programId = new PublicKey('$PROGRAM_ID');
  const mxeAccount = getMXEAccAddress(programId);
  
  const compDefNames = ['survey_analytics', 'quiz_evaluation', 'analytics_computation', 'quiz_threshold_check'];
  
  for (const name of compDefNames) {
    const compDefAccount = getCompDefAccAddress(mxeAccount, name);
    const accountInfo = await connection.getAccountInfo(compDefAccount);
    console.log(\`\${name}: \${accountInfo ? 'EXISTS' : 'MISSING'} (\${compDefAccount.toBase58()})\`);
  }
}

checkCompDefs().catch(console.error);
"
```

### 5. **Update Frontend Configuration**
Update both frontend projects:
- `sequre-vite/src/config/constants.ts`
- `sequre-frontend/src/config/constants.ts`

Change:
```typescript
export const PROGRAM_ID = new PublicKey('7STBh2auA5RxtGtcDHfk3nhQQWENa8rFTA7wLACaxkWh');
```

## Current Deployment Status

### ✅ **Deployment Complete:**
- **Program ID**: `7STBh2auA5RxtGtcDHfk3nhQQWENa8rFTA7wLACaxkWh`
- **Source Code**: Updated with new program ID
- **Anchor.toml**: Updated with new program ID
- **MXE Account**: `CyWDyG263xR7AiEYtcDH6copJ6PB4z6CPp2QMAt4AYpA`
- **Computation Definitions**: All 4 initialized successfully with off-chain circuit sources
- **Frontend**: Updated with new program ID and actual computation definition addresses
- **RPC**: Using Helius RPC (resolved program ID mismatch issue)
- **Frontend RPC**: Updated to use same Helius RPC as deployment
- **Circuit Files**: Uploaded to Supabase Storage and accessible via public URLs

### **Actual Computation Definition Addresses:**
- **survey_analytics**: `FEvN88mmYug5vqXLbKyzraLFBiw28vDLxvN4QKkcynp3`
- **quiz_evaluation**: `HN9s4eMoKXxcQyADMtefpv9cXb9Q2xSLHnW7nTToXqtJ`
- **analytics_computation**: `23VzoWcsK6QWMU52tD9UdEhtXBYMLArHzcYeeptND5Hb`
- **quiz_threshold_check**: `3xrABWb3wyDn4LF2EwKjfE7M4S4YYELZG1YCwrCFc7of`

**Note**: These addresses are stored in `COMPUTATION_DEFINITION_ADDRESSES` in `constants.ts` to avoid SDK derivation mismatch.

### ✅ **Deployment Complete:**
1. **✅ Rebuilt**: `arcium build`
2. **✅ Deployed**: `arcium deploy` command
3. **✅ Initialized**: Run `init_comp_defs.ts`
4. **✅ Updated Frontend**: Updated program ID in constants

## Important Notes

- **Always delete IDL first** to force regeneration with synced program ID
- **Use `arcium keys sync`** instead of generating new keypairs
- **Use reliable RPC** (`https://api.devnet.solana.com`) instead of `-u d`
- **Cluster offset**: `3726127828` (second cluster)
- **MXE address**: Will be fetched automatically by frontend
- **Computation definitions**: Must be initialized after deployment

## Troubleshooting

- **IDL mismatch**: Delete IDL and rebuild
- **Program ID mismatch**: Run `arcium keys sync` to sync program ID
- **Deployment fails**: Check SOL balance and RPC endpoint
- **Computation defs fail**: Ensure cluster account is correct











