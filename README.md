# Arcium IDL Generation Issue - Debugging Files

This repository contains critical files to diagnose the Arcium IDL generation issue where `arcium build` generates incomplete IDLs that break BorshCoder type registration.

## **Problem Summary**

- ✅ **Arcium CLI/SDK versions**: 0.3.0 (correctly aligned)
- ✅ **Code was working**: Previously worked with 24 arguments
- ❌ **Current IDL issue**: Missing account type definitions
- ❌ **BorshCoder broken**: Cannot register types without complete IDL

## **Critical Files**

### **1. IDL Files (Most Important)**
- **`current_broken_idl.json`** - Current IDL from `sequre-vite/src/se_qure.json`
- **`old_working_idl.json`** - IDL from `se_qure-fresh/target/idl/se_qure.json`

**Issue**: Both IDLs have account discriminators but **missing type definitions**:
```json
{
  "name": "Survey",
  "discriminator": [146, 73, 17, 4, 6, 233, 167, 141]
  // ❌ Missing: "type": { "kind": "struct", "fields": [...] }
}
```

### **2. Configuration Files**
- **`arcium_config.toml`** - Arcium configuration
- **`cargo_lock.txt`** - Exact dependency versions
- **`package_json.txt`** - Frontend dependencies

### **3. Code Samples**
- **`rust_program_sample.rs`** - First 200 lines of main program
- **`useAnchorProgram.ts`** - Frontend hook showing IDL usage
- **`encrypted_ixs_cargo.toml`** - Encrypted instructions configuration

## **Evidence of the Issue**

### **IDL Validation Results**
```bash
$ node check_idl.js
Accounts count: 11
First few accounts:
  - ClockAccount undefined      ← No type information
  - Cluster undefined           ← No type information  
  - ComputationDefinitionAccount undefined ← No type information
```

### **Arcium Version Check**
```bash
$ arcium --version
arcium-cli 0.3.0

# From Cargo.toml:
arcium-client = "0.3.0"
arcium-anchor = "0.3.0"
arcium-macros = "0.3.0"
```

### **Working Code Evidence**
The Rust program shows 24+ argument usage that was working before:
```rust
let instructor_args = vec![
    Argument::ArcisPubkey(student_pub_key),           // 1
    Argument::PlaintextU128(student_nonce),           // 2  
    Argument::EncryptedU32(ciphertext_answer1),       // 3
    // ... continues to 24+ arguments
];
```

## **Root Cause Analysis**

**The Issue**: `arcium build` generates incomplete IDLs that break BorshCoder type registration.

**What's Working**:
- ✅ MPC circuit generation
- ✅ Account discriminators
- ✅ Instruction definitions
- ✅ Type definitions (in types section)

**What's Broken**:
- ❌ Account type definitions (missing from accounts section)
- ❌ BorshCoder type registration
- ❌ Account deserialization

## **Request for Nico**

**Question**: How can we get `arcium build` to generate complete IDLs with account type definitions, or is there a post-processing solution to enrich the IDL?

**Expected Fix**: The accounts section should include:
```json
{
  "name": "Survey",
  "discriminator": [146, 73, 17, 4, 6, 233, 167, 141],
  "type": {
    "kind": "struct",
    "fields": [
      {
        "name": "creator",
        "type": "pubkey"
      },
      {
        "name": "title", 
        "type": "string"
      }
      // ... rest of fields
    ]
  }
}
```

## **Environment Details**

- **OS**: macOS 24.3.0
- **Arcium CLI**: 0.3.0
- **Arcium SDK**: 0.3.0
- **Anchor**: 0.31.1
- **Solana**: 1.18.x
- **Program ID**: `5qfZir789yPZ9vQoSYt6mnWtbapP2FyqHnKogvswfC69`

## **Related Issues**

This repository also contains evidence of related Arcium issues:
- Argument type conversion problems
- Multiple user submission issues
- Sign PDA account signer privilege escalation
- PublicKey parameter wrapping issues

## **Next Steps**

1. Analyze the IDL differences between current and old versions
2. Identify why `arcium build` stopped generating complete account types
3. Provide solution for IDL enrichment or fix Arcium build process
4. Test with BorshCoder type registration
