# MRE: Transaction Size Limits & Double Signing Issue

**Arcium Version:** v0.3.0 (devnet)  
**Issues:** 
1. Transaction size limits with encrypted data
2. Users having to sign twice for quiz submissions

---

## Context

I'm building a quiz system with Arcium MPC where students submit encrypted answers and the MPC network grades them. I split it into two steps:
- **Step 1:** Student submits encrypted answers to blockchain (lightweight transaction)
- **Step 2:** Trigger MPC computation to grade the quiz (heavier transaction with many arguments)

I originally tried to do both in ONE transaction but hit Solana's transaction size limit.

---

## Issue 1: Transaction Size Limits

### Original Approach (FAILED)
I tried to submit answers + trigger grading in a single transaction:

```rust
pub fn submit_quiz_and_grade(
    ctx: Context<SubmitQuizAndGrade>,
    encrypted_answers: [u8; 128],
    answer_hash: [u8; 32],
    timestamp: i64,
    computation_offset: u64,
    student_pub_key: [u8; 32],
    student_nonce: u128,
    threshold: u8,
) -> Result<()> {
    // Store answers
    let answers_storage = &mut ctx.accounts.answers_storage;
    answers_storage.encrypted_answers = encrypted_answers;
    answers_storage.answer_hash = answer_hash;
    answers_storage.submission_timestamp = timestamp;
    
    // Build 20 arguments for MPC grading
    let grading_args = vec![
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        Argument::EncryptedU32(/* ... */),
        // ... 17 more arguments
    ];
    
    // Queue computation
    queue_computation(
        ctx.accounts.arcium_program.to_account_info(),
        // ... many account infos
        computation_offset,
        grading_args,
    )?;
    
    Ok(())
}
```

**Result:** Transaction too large (exceeds 1232 byte limit)

---

### Current Approach (2-STEP FLOW)

So I had to split it into two separate transactions because the combined transaction was exceeding the 1232 byte limit. The `queue_computation` call alone with all the Arcium accounts and 20 arguments takes up ~900 bytes, and adding the answer submission data on top pushes it over.

**Step 1 - Submit Answers (Lightweight):**
```rust
pub fn submit_quiz_answers(
    ctx: Context<SubmitQuizAnswers>,
    encrypted_answers: [u8; 128],
    answer_hash: [u8; 32],
    timestamp: i64,
) -> Result<()> {
    let answers_storage = &mut ctx.accounts.answers_storage;
    answers_storage.quiz = ctx.accounts.quiz.key();
    answers_storage.student = ctx.accounts.student.key();
    answers_storage.encrypted_answers = encrypted_answers;
    answers_storage.answer_hash = answer_hash;
    answers_storage.submission_timestamp = timestamp;
    Ok(())
}
```

**Step 2 - Trigger Grading (Heavy):**
```rust
pub fn compute_quiz_grade(
    ctx: Context<ComputeQuizGrade>,
    computation_offset: u64,
    student_pub_key: [u8; 32],
    student_nonce: u128,
    threshold: u8,
) -> Result<()> {
    let grading_args = vec![
        // 20 arguments for MPC computation...
    ];
    
    queue_computation(
        ctx.accounts.arcium_program.to_account_info(),
        // ... many account infos
        computation_offset,
        grading_args,
    )?;
    
    Ok(())
}
```

**This works, but requires TWO wallet signatures!**

---

## Issue 2: Double Signing (User Friction)

### Current User Experience:
1. User fills out quiz
2. Clicks "Submit"
3. **Wallet popup 1:** "Sign transaction to submit answers" ✅ User signs
4. **Wallet popup 2:** "Sign transaction to compute grade" ✅ User signs again

**Users ask:** "Why do I have to sign twice?"

---

### Why I Can't Combine Them:

The issue is that each Arcium `queue_computation` call requires a ton of accounts (12 total including MXE, sign PDA, mempool, computation account, etc.) and the argument vector itself. Even after consolidating the MXE-encrypted data into arrays to reduce from 26 to 20 arguments, the transaction is still too large.

From my frontend:
```typescript
async submitQuizAnswers(quizPda, encryptedData, answerHash, timestamp) {
  const tx1 = await this.program.methods
    .submitQuizAnswers(
      Array.from(encryptedData),
      Array.from(answerHash),
      new BN(timestamp)
    )
    .accounts({
      quiz: quizPda,
      answersStorage: answersStoragePda,
      student: this.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  
  return tx1; // ~400 bytes
}

async computeQuizGrade(quizPda, studentPubKey, studentNonce, threshold) {
  const arciumAccounts = await this.deriveArciumAccounts(quizPda);
  
  const tx2 = await this.program.methods
    .computeQuizGrade(
      arciumAccounts.computationOffset,
      Array.from(studentPubKey),
      studentNonce,
      threshold
    )
    .accounts({
      quiz: quizPda,
      answersStorage: answersStoragePda,
      student: this.wallet.publicKey,
      mxeAccount: arciumAccounts.mxeAccount,
      signPdaAccount: arciumAccounts.signPdaAccount,
      mempoolAccount: arciumAccounts.mempoolAccount,
      executingPool: arciumAccounts.executingPool,
      computationAccount: arciumAccounts.computationAccount,
      compDefAccount: arciumAccounts.compDefAccount,
      clusterAccount: arciumAccounts.clusterAccount,
      poolAccount: arciumAccounts.poolAccount,
      clockAccount: SYSVAR_CLOCK_PUBKEY,
      arciumProgram: ARCIUM_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  
  return tx2; // ~900 bytes
}
```

**Combined transaction size:** 400 + 900 = 1300 bytes > 1232 byte limit ❌

---

## Questions

### 1. Is there a way to combine these into ONE transaction?
   - Can I reduce the number of accounts needed for `queue_computation`?
   - Are there any Arcium SDK helpers for reducing transaction size?

### 2. Can I avoid the second signature?
   - Is there a way to make Step 2 "automatic" after Step 1?

### 3. Is the 2-step flow the recommended approach?
   - Is this how Arcium MPC is typically used?
   - Are there examples of single-transaction MPC workflows?

---

## Transaction Size Breakdown

**Step 1 Transaction (~400 bytes):**
- Instruction: `submit_quiz_answers`
- Accounts: 4 (quiz, answers_storage, student, system_program)
- Data: 168 bytes (128 encrypted + 32 hash + 8 timestamp)

**Step 2 Transaction (~900 bytes):**
- Instruction: `compute_quiz_grade`
- Accounts: 12 (quiz, answers_storage, student, 9 Arcium accounts)
- Data: ~200 bytes (offset, pubkey, nonce, threshold, + CPI args)
- CPI to Arcium: `queue_computation` with 20 arguments (~400 bytes)

**Combined: ~1300 bytes > 1232 limit** ❌

---

## What I've Tried

1. Reduced argument count from 26 to 20 using array types (consolidated `Enc<Mxe, u32>` params into `Enc<Mxe, [u32; 8]>`)
2. Used Solana versioned transactions (still too large - ~1300 bytes vs 1232 limit)
3. Tried combining both instructions into one transaction (exceeds size limit)
4. Tried using transaction batching with wallet adapter (not supported)
5. Looked into Address Lookup Tables to reduce account size (but Arcium's `queue_computation` requires all those accounts inline)
6. Current 2-step flow works but requires double signing - users are confused why they need to sign twice

