# MRE: MXE Initialization Failing - Cluster Offset Conflict

**Arcium Version:** v0.3.0 (devnet)  
**Issue:** New program can't initialize its MXE because old MXE (from previous deployment) is still using the same cluster offset

---

## The Problem

When deploying a new Arcium program to the same cluster offset where you previously had another program:

### ❌ MXE Initialization Failing
- **Error:** `"Account already in use"` or `"Allocate: account Address { address: ... } already in use"`
- **Issue:** My new program can't initialize its MXE because the old MXE (from my previous program deployment) is still using the same cluster offset

### ⏱️ Computation Finalization Timing Out
- **Issue:** Because the MXE isn't initialized, my program can't connect to Arcium's MPC cluster
- When I try to initialize computation definitions, they never finalize (timeout after 5 minutes)
- **Root cause:** MXE not initialized for the NEW program
- **Symptom:** CompDef finalization timeout

---

## Why This Happens

### The Root Cause

**Each program needs its OWN MXE account**, even if they share the same cluster offset.

When you run `arcium deploy`, it tries to initialize a **cluster-level MXE** (shared across all programs on that cluster). If you previously deployed a different program to the same cluster offset, that cluster-level MXE already exists, causing the "account already in use" error.

**However**, your new program needs a **program-specific MXE** derived from YOUR program ID, not the cluster-level one.

### The Flow

```
Old Deployment:
- Program A deployed → Cluster MXE created → Program A's MXE created

New Deployment (same cluster offset):
- Program B deployed → ❌ Cluster MXE exists (error "already in use")
                      → ⚠️ Program B's MXE NOT created
                      → ❌ CompDef initialization times out
```

---

## The Solution

### Step 1: Deploy Your Program (Ignore the Error)

```bash
arcium deploy \
  --cluster-offset <YOUR_CLUSTER_OFFSET> \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url <YOUR_RPC_URL> \
  --program-keypair target/deploy/<program_name>-keypair.json \
  --program-name <program_name>
```

**Expected behavior:**
1. ✅ Program deploys successfully
2. ❌ MXE initialization fails with "account already in use" - **THIS IS NORMAL**
3. ⚠️ You must manually initialize your program-specific MXE next

### Step 2: Manually Initialize Program-Specific MXE

```bash
# Get your new program ID
PROGRAM_ID=$(solana-keygen pubkey target/deploy/<program_name>-keypair.json)
echo "Program ID: $PROGRAM_ID"

# Initialize MXE specifically for YOUR program
arcium init-mxe \
  --callback-program $PROGRAM_ID \
  --cluster-offset <YOUR_CLUSTER_OFFSET> \
  --keypair-path ~/.config/solana/id.json \
  --authority $(solana address) \
  --rpc-url <YOUR_RPC_URL>
```

**Important:** Always include `--authority $(solana address)` to set yourself as the MXE authority.

### Step 3: Verify MXE Was Created

```bash
# Derive expected MXE address
node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log('Expected MXE Account:', mxeAccount.toBase58());
"

# Check it exists on-chain
MXE_ACCOUNT=$(node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
console.log(getMXEAccAddress(programId).toBase58());
")

solana account $MXE_ACCOUNT --url <YOUR_RPC_URL>
```

**Expected output:** Account exists with owner `BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6` (Arcium program)

### Step 4: Initialize Computation Definitions

Now that your program-specific MXE exists, you can initialize computation definitions:

```bash
# Run your init script
ANCHOR_PROVIDER_URL=<YOUR_RPC_URL> \
ANCHOR_WALLET=~/.config/solana/id.json \
npx ts-node scripts/init_comp_defs.ts
```

**Expected behavior:** Computation definitions initialize successfully and finalize within 30-60 seconds.

---

## Complete Example

```bash
# === STEP 1: Deploy Program ===
cd my-arcium-program
arcium build

arcium deploy \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url "https://api.devnet.solana.com" \
  --program-keypair target/deploy/my_program-keypair.json \
  --program-name my_program

# Output: ✅ Program deployed, ❌ MXE "already in use" (IGNORE THIS)

# === STEP 2: Initialize Program-Specific MXE ===
PROGRAM_ID=$(solana-keygen pubkey target/deploy/my_program-keypair.json)
echo "New Program ID: $PROGRAM_ID"

arcium init-mxe \
  --callback-program $PROGRAM_ID \
  --cluster-offset 3726127828 \
  --keypair-path ~/.config/solana/id.json \
  --authority $(solana address) \
  --rpc-url "https://api.devnet.solana.com"

# Output: ✅ MXE initialized successfully

# === STEP 3: Verify MXE Exists ===
node -e "
const { PublicKey } = require('@solana/web3.js');
const { getMXEAccAddress } = require('@arcium-hq/client');
const programId = new PublicKey('$PROGRAM_ID');
const mxeAccount = getMXEAccAddress(programId);
console.log('MXE Account:', mxeAccount.toBase58());
"

# === STEP 4: Initialize Computation Definitions ===
ANCHOR_PROVIDER_URL="https://api.devnet.solana.com" \
ANCHOR_WALLET=~/.config/solana/id.json \
npx ts-node scripts/init_comp_defs.ts

# Output: ✅ All computation definitions initialized and finalized
```

---

## Troubleshooting

### Issue: Still getting "MXE not found" after initialization

**Solution:** Make sure you're deriving the MXE address from your **new program ID**, not the old one:

```javascript
// ❌ WRONG - using old/wrong program ID
const mxeAccount = getMXEAccAddress(new PublicKey('<OLD_PROGRAM_ID>'));

// ✅ CORRECT - using current program ID
const mxeAccount = getMXEAccAddress(program.programId);
```

### Issue: CompDefs still timing out

**Check:**
1. MXE exists: `solana account <MXE_ADDRESS>`
2. MXE authority is correct: Should be your wallet address
3. Using correct cluster offset in both MXE initialization and CompDef initialization
4. Circuit files are uploaded and accessible (if using off-chain circuits)

**Debug commands:**
```bash
# Check MXE account data
solana account <MXE_ADDRESS> --output json | jq

# Check if CompDef account was created (even if not finalized)
solana account <COMP_DEF_ADDRESS> --output json | jq
```

### Issue: "InvalidAuthority" error during CompDef initialization

**Solution:** You forgot to include `--authority $(solana address)` when initializing MXE.

**Fix:**
1. Close the current MXE account (or deploy a fresh program with new ID)
2. Re-initialize MXE with `--authority $(solana address)`
3. Try CompDef initialization again

---

## Key Takeaways

1. **"Account already in use" during deployment is NORMAL** if you previously deployed to that cluster offset
2. **You must manually initialize program-specific MXE** after deployment with `arcium init-mxe --callback-program <YOUR_PROGRAM_ID>`
3. **Each program needs its own MXE**, even on the same cluster offset
4. **Always include `--authority $(solana address)`** when initializing MXE
5. **CompDefs won't finalize** if the program-specific MXE doesn't exist

---

## Prevention

To avoid this issue in the future:

### Option 1: Use Different Cluster Offsets
```bash
# First program
arcium deploy --cluster-offset 3726127828 ...

# Second program - use different offset
arcium deploy --cluster-offset 1078779259 ...
```

### Option 2: Always Manually Initialize MXE
Make manual MXE initialization part of your deployment script:

```bash
#!/bin/bash
# deploy.sh

# Deploy program
arcium deploy --cluster-offset $CLUSTER_OFFSET ...

# Get program ID
PROGRAM_ID=$(solana-keygen pubkey target/deploy/my_program-keypair.json)

# Initialize program-specific MXE
arcium init-mxe \
  --callback-program $PROGRAM_ID \
  --cluster-offset $CLUSTER_OFFSET \
  --keypair-path ~/.config/solana/id.json \
  --authority $(solana address) \
  --rpc-url $RPC_URL

# Initialize computation definitions
npx ts-node scripts/init_comp_defs.ts
```

### Option 3: Close Old Program First
```bash
# Before deploying new program, close the old one
solana program close <OLD_PROGRAM_ID> --url devnet --bypass-warning

# This releases the old MXE account
# Then deploy new program normally
```

---

## Related Issues

- **MXE Authority Mismatch:** Covered in `/se_qure/DEPLOYMENT_PROCEDURE.md` lines 247-271
- **CompDef Initialization Failures:** Check that MXE exists first
- **Cluster Offset Selection:** Each cluster can have multiple programs, each with their own MXE

---

## Environment Details

- **Arcium Version:** v0.3.0
- **Network:** Devnet
- **Cluster Offsets Used:** 3726127828, 1078779259
- **RPC Endpoints:** 
  - `https://api.devnet.solana.com`
  - `https://devnet.helius-rpc.com/?api-key=<YOUR_KEY>`

---

## Additional Resources

- Full deployment procedure: `/se_qure/DEPLOYMENT_PROCEDURE.md`
- Arcium documentation: [Arcium Docs](https://docs.arcium.com)
- MXE account derivation: `@arcium-hq/client` → `getMXEAccAddress(programId)`

