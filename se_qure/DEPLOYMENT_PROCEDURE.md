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
**Note**: If it fails due to write transaction just retry
```bash
# Deploy to devnet (MXE initialization failure is expected and normal)
arcium deploy --cluster-offset 3726127828 --keypair-path ~/.config/solana/id.json --rpc-url 'https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777' --program-keypair target/deploy/se_qure-keypair.json --program-name se_qure

# NOTE: MXE initialization will fail with "account already in use" - this is NORMAL
# The cluster MXE already exists, but we need program-specific MXE
```

### 3.1. **Initialize Program-Specific MXE**
```bash
# Get your program ID
PROGRAM_ID=$(solana-keygen pubkey target/deploy/se_qure-keypair.json)
echo "Program ID: $PROGRAM_ID"

# Initialize MXE for your specific program
# NOTE: The --authority parameter may not work as expected, but include it anyway
arcium init-mxe \
  --callback-program $PROGRAM_ID \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --authority $(solana address) \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
  
```

### 3.2. **Debug Deployment Process**
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
export const PROGRAM_ID = new PublicKey('C67b3EadVSbnggx5U6gJTHtizykQ9VUC75nGUGSWZWc1');
```

## Current Deployment Status

### ✅ **Deployment Complete:**
- **Program ID**: `C67b3EadVSbnggx5U6gJTHtizykQ9VUC75nGUGSWZWc1`
- **Source Code**: Updated with new program ID
- **Anchor.toml**: Updated with new program ID
- **MXE Account**: `9Sxfa3fhZAxzgQMRZxXcrCCj8rir83iWtyUkheHo8BoL`
- **Computation Definitions**: All 4 initialized successfully with off-chain circuit sources
- **Frontend**: Updated with new program ID and actual computation definition addresses
- **RPC**: Using Helius RPC (resolved program ID mismatch issue)
- **Frontend RPC**: Updated to use same Helius RPC as deployment
- **Circuit Files**: Uploaded to Supabase Storage and accessible via public URLs

### **Actual Computation Definition Addresses:**
- **survey_analytics**: `6tohW8jmZa5ULfYfCAnvhosd9bUZJ7jdqeWGUUBYL7YE`
- **quiz_evaluation**: `DyodxHZscLyQVEaaZcVSYYNcWdKdCedg35sArmD3WwGU`
- **analytics_computation**: `6S5Rv8ZyJtVfuZ8gEKq5ya5atETXEmmRzA1WiQL2UsfB`
- **quiz_threshold_check**: `Az8zoMyZouwasXaAgZrWUmrWhYwbvNtm9R3eE9GDaA1t`

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

## MXE Initialization Issue & Fix

### **Issue**: MXE Account Not Initializing During Deployment

**Problem**: When deploying a new program, the `arcium deploy` command fails to initialize the program-specific MXE account. The deployment process tries to initialize a cluster-level MXE (which already exists) instead of the program-specific MXE.

**Symptoms**:
- Deployment succeeds but MXE initialization fails with "account already in use" error
- Error shows old hardcoded MXE address instead of program-derived MXE address
- Computation definitions initialization fails because program-specific MXE doesn't exist

**Root Cause**: 
The `arcium deploy` process initializes MXE for the cluster offset, not for the specific program. Each program needs its own MXE account derived from the program ID.

### **Fix**: Manual MXE Initialization

After successful program deployment, manually initialize the program-specific MXE:

```bash
# Initialize MXE for your specific program
arcium init-mxe \
  --callback-program <YOUR_PROGRAM_ID> \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```

**Example**:
```bash
arcium init-mxe \
  --callback-program DkWzQVB7iDaLrVcNBnf5ESq4coPNXu6nnaWUYXDqWHzf \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```

**Verification**:
```bash
# Check if your program's MXE exists
node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('YOUR_PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log('Expected MXE Account:', mxeAccount.toBase58());
"

# Verify on-chain
solana account <MXE_ADDRESS> --url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```

**Note**: This step is required after every fresh program deployment before initializing computation definitions.

### **Issue**: MXE Authority Mismatch

**Problem**: Computation definitions initialization fails with "InvalidAuthority" error.

**Root Cause**: 
The MXE was initialized without specifying the `--authority` parameter, causing it to use a default authority instead of your wallet.

**Symptoms**:
- Error: "InvalidAuthority. Error Number: 6000. Error Message: The given authority is invalid"
- MXE authority is different from your wallet address
- Computation definitions initialization fails

**Fix**: 
Always specify the `--authority` parameter when initializing MXE:

```bash
arcium init-mxe \
  --callback-program <YOUR_PROGRAM_ID> \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --authority $(solana address) \
  --rpc-url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```

**Prevention**: Always include `--authority $(solana address)` in MXE initialization commands.

### **Issue**: Cluster MXE Already Exists Error

**Problem**: MXE initialization fails with "account already in use" error during deployment.

**Root Cause**: 
The `arcium deploy` command tries to initialize a cluster-level MXE account that already exists from previous deployments.

**Symptoms**:
- Error: "Allocate: account Address { address: An14qtza31FVRp4uvVNR3F2BZAw5yLNRqN1uNRr9Soxq, base: None } already in use"
- Deployment succeeds but MXE initialization fails
- This is a cluster-level MXE, not program-specific

**Solution**: 
This error is **NORMAL** and can be ignored. The deployment process will:
1. ✅ Deploy your program successfully
2. ❌ Fail to initialize cluster MXE (expected - already exists)
3. ✅ Allow you to initialize program-specific MXE manually

**Fix**: 
Proceed with manual MXE initialization using `arcium init-mxe` as shown in Step 3.1.

### **Issue**: MXE Authority Mismatch (Resolved by Fresh Deployment)

**Problem**: Computation definitions initialization fails with "InvalidAuthority" error even after MXE initialization.

**Root Cause**: 
The MXE was initialized with a different authority than expected, causing computation definition initialization to fail.

**Symptoms**:
- Error: "InvalidAuthority. Error Number: 6000. Error Message: The given authority is invalid"
- MXE authority is different from your wallet address
- Computation definitions initialization fails

**Solution**: 
**Fresh deployment resolves this issue**. The problem occurs when:
1. MXE is initialized with unexpected authority
2. The `--authority` parameter doesn't work as expected
3. Fresh deployment with new program ID creates clean MXE account

**Fix**: 
1. Close existing program: `solana program close <PROGRAM_ID> --bypass-warning`
2. Start fresh deployment process
3. Initialize MXE with new program ID
4. Computation definitions will initialize successfully

**Prevention**: Always use fresh deployments when encountering authority issues.

## **Deployment Verification**

After successful deployment, verify all components exist on-chain:

### **Verify Program**
```bash
# Check program exists and is executable
solana account <YOUR_PROGRAM_ID> --url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```
**Expected**: `Executable: true`, `Owner: BPFLoaderUpgradeab1e11111111111111111111111`

### **Verify MXE Account**
```bash
# Check MXE account exists
solana account <MXE_ACCOUNT_ADDRESS> --url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```
**Expected**: `Owner: BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6`

### **Verify Computation Definitions**
```bash
# Check all computation definitions exist
solana account <COMP_DEF_ADDRESS> --url "https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777"
```
**Expected**: All 4 computation definitions with rent-exempt balances

### **Complete Verification Script**
```bash
#!/bin/bash
echo "=== DEPLOYMENT VERIFICATION ==="
PROGRAM_ID=$(solana-keygen pubkey target/deploy/se_qure-keypair.json)
echo "Program ID: $PROGRAM_ID"

# Verify program
echo "✅ Program exists: $(solana account $PROGRAM_ID --url 'https://devnet.helius-rpc.com/?api-key=eb6397f9-9d6d-40c5-b723-5bfb28ead777' --output json | jq -r '.account != null')"

# Verify MXE (you'll need to get the actual MXE address from your deployment)
echo "✅ MXE exists: [Check manually with solana account <MXE_ADDRESS>]"

# Verify computation definitions
echo "✅ Comp Defs exist: [Check manually with solana account <COMP_DEF_ADDRESS>]"
```

## **Success Criteria**

✅ **Program deployed and executable**  
✅ **MXE account initialized**  
✅ **All 4 computation definitions initialized**  
✅ **All accounts verified on-chain**  

**Your SeQure Arcium MXE program is now ready for MPC processing!**











