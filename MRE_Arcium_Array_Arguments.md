# MRE: Arcium `InvalidArguments` Error with `Enc<Mxe, [u32; N]>` Arrays

**Arcium Version:** v0.3.0 (devnet)  
**Issue:** Getting "Invalid argument ArcisPubkey for parameter PlaintextU128" when passing MXE-encrypted array arguments

---

## Context

I'm building a quiz grading system where students submit encrypted answers and the MPC network grades them. The quiz grading data (correct answers, points, threshold, class stats) is encrypted with MXE so only the MPC network can see it.

I consolidated 8 individual `Enc<Mxe, u32>` parameters into ONE `Enc<Mxe, [u32; 8]>` to reduce argument count since I was hitting memory issues with 26+ arguments (remember your advice from before about using array types for shared encryption).

---

## The Error

```
Program log: Invalid argument ArcisPubkey([140, 70, 193, 175, ...]) 
             for parameter PlaintextU128

AnchorError: InvalidArguments (Error 6301)
Message: Arguments supplied are invalid.
```

So the Arcium program is expecting a `PlaintextU128` (nonce) but I'm passing an `ArcisPubkey` (public key). This suggests my argument order or format is wrong.

---

## Circuit Signature

```rust
// se_qure/encrypted-ixs/src/quiz_evaluation.rs

#[instruction]
pub fn quiz_evaluation(
    user_answers: Enc<Shared, [u32; 2]>,      // [answer1, answer2]
    quiz_data: Enc<Mxe, [u32; 8]>,            // [correct1, correct2, points1, points2, 
                                               //  threshold, stat1, stat2, stat3]
    student: Shared,                          // Re-encrypt for student
    instructor: Shared,                       // Re-encrypt for instructor
    quiz_creator: Shared                      // Re-encrypt for quiz creator
) -> (
    Enc<Shared, QuizResult>,
    Enc<Shared, InstructorAnalytics>,
    Enc<Shared, StudentFeedback>
) {
    // Decrypt inputs
    let user_ans = user_answers.to_arcis();
    let quiz_dat = quiz_data.to_arcis();
    
    // Grading logic...
    let score = if user_ans[0] == quiz_dat[0] { quiz_dat[2] } else { 0 }
              + if user_ans[1] == quiz_dat[1] { quiz_dat[3] } else { 0 };
    
    // Return encrypted results...
}
```

---

## How I'm Passing Arguments (Rust Program)

```rust
// se_qure/programs/se_qure/src/lib.rs

pub fn compute_quiz_grade(
    ctx: Context<ComputeQuizGrade>,
    computation_offset: u64,
    student_pub_key: [u8; 32],
    student_nonce: u128,
    threshold: u8,
) -> Result<()> {
    let answers_storage = &ctx.accounts.answers_storage;
    let quiz = &ctx.accounts.quiz;
    let mxe_pubkey = ctx.accounts.mxe_account.key().to_bytes();
    
    let grading_args = vec![
        // user_answers: Enc<Shared, [u32; 2]>
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        Argument::EncryptedU32(answers_storage.encrypted_answers[0..32].try_into().unwrap()),
        Argument::EncryptedU32(answers_storage.encrypted_answers[32..64].try_into().unwrap()),
        
        // quiz_data: Enc<Mxe, [u32; 8]>
        Argument::ArcisPubkey(mxe_pubkey),
        Argument::PlaintextU128(quiz.quiz_data_nonce),
        Argument::EncryptedU32(quiz.encrypted_quiz_data[0..32].try_into().unwrap()),    // correct1
        Argument::EncryptedU32(quiz.encrypted_quiz_data[32..64].try_into().unwrap()),   // correct2
        Argument::EncryptedU32(quiz.encrypted_quiz_data[64..96].try_into().unwrap()),   // points1
        Argument::EncryptedU32(quiz.encrypted_quiz_data[96..128].try_into().unwrap()),  // points2
        Argument::EncryptedU32(quiz.encrypted_quiz_data[128..160].try_into().unwrap()), // threshold
        Argument::EncryptedU32(quiz.encrypted_quiz_data[160..192].try_into().unwrap()), // stat1
        Argument::EncryptedU32(quiz.encrypted_quiz_data[192..224].try_into().unwrap()), // stat2
        Argument::EncryptedU32(quiz.encrypted_quiz_data[224..256].try_into().unwrap()), // stat3
        
        // student: Shared
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        
        // instructor: Shared
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        
        // quiz_creator: Shared
        Argument::ArcisPubkey(quiz.creator.to_bytes()),
        Argument::PlaintextU128(student_nonce),
    ];
    
    queue_computation(
        ctx.accounts.arcium_program.to_account_info(),
        ctx.accounts.sign_pda_account.to_account_info(),
        // ... other accounts
        computation_offset,
        grading_args,
    )?;
    
    Ok(())
}
```

**Total arguments: 20**
- `user_answers`: 4 args (pubkey, nonce, 2 ciphertexts)
- `quiz_data`: 10 args (mxe_pubkey, nonce, 8 ciphertexts)
- `student`: 2 args
- `instructor`: 2 args
- `quiz_creator`: 2 args

---

## Frontend Encryption (TypeScript)

```typescript
// How I encrypt the quiz data with MXE on quiz creation
async setQuizGradingData(
  quizPda: PublicKey,
  correctAnswers: number[],  // [1, 0] for 2 questions
  points: number[],          // [1, 1] points per question
  threshold: number,         // 70 (passing threshold)
  classStats: number[]       // [0, 0, 0] initial stats
): Promise<{ success: boolean; signature?: string }> {
  const mxePublicKey = await this.getMXEPublicKeyWithRetry();
  
  // Generate random client keypair for ECDH
  const clientPrivateKey = x25519.utils.randomSecretKey();
  const clientPublicKey = x25519.getPublicKey(clientPrivateKey);
  const sharedSecret = x25519.getSharedSecret(clientPrivateKey, mxePublicKey);
  
  const cipher = new RescueCipher(sharedSecret);
  
  // Consolidate all 8 values into ONE array
  const allDataBigInt = [
    ...correctAnswers,  // [1, 0]
    ...points,          // [1, 1]
    threshold,          // 70
    ...classStats       // [0, 0, 0]
  ].map(n => BigInt(n)); // [1n, 0n, 1n, 1n, 70n, 0n, 0n, 0n]
  
  // Generate ONE nonce for the entire array
  const quizDataNonce = randomBytes(16);
  
  // Encrypt all 8 values with the SAME key/nonce
  const encryptedRaw = cipher.encrypt(allDataBigInt, quizDataNonce, mxePublicKey);
  
  // Concatenate all 8 ciphertexts: [cipher1, cipher2, ..., cipher8]
  const encryptedQuizData = new Uint8Array(256); // 8 * 32 bytes
  encryptedRaw.forEach((cipher, idx) => {
    encryptedQuizData.set(new Uint8Array(Array.from(cipher)), idx * 32);
  });
  
  // Convert nonce to u128 for Rust
  const quizDataNonceU128 = new BN(
    BigInt('0x' + Buffer.from(quizDataNonce).toString('hex')).toString()
  );
  
  // Send to Solana
  await this.program.methods
    .setQuizGradingData(
      Array.from(encryptedQuizData),        // 256 bytes
      Array.from(clientPublicKey),          // 32 bytes (for ECDH)
      quizDataNonceU128                     // u128
    )
    .accounts({ quiz: quizPda, creator: this.wallet.publicKey })
    .rpc();
}
```

---

## Questions

### 1. Is my argument format for `Enc<Mxe, [u32; 8]>` correct?
   - Should it be: `(mxe_pubkey, nonce, cipher1, cipher2, ..., cipher8)` = 10 args?
   - Or is there a special format for MXE-encrypted arrays?
   - Does the order matter? Should nonce come before or after pubkey?

### 2. How do I update computation definitions after changing circuit signatures?
   After modifying a circuit signature and running `arcium build`:
   - Do I need to delete the old comp def account and recreate it?
   - Or is re-running initialization with the new `.arcis` file sufficient?
   - I uploaded new circuit bytecode to Supabase and reinitialized, but the error persists.

### 3. Any debugging tools?
   - Is there a way to inspect what argument types the comp def expects vs what I'm passing?
   - Can I get more detailed error messages than "InvalidArguments"?

---

## Environment

- **Program ID:** `FatytETyGVXdDKH4xQXNaE6BTkmXe2QfieZSPQc9Au7T`
- **Comp Def (quiz_evaluation):** `9z1xD2xzzCSfKivWh8MFKgBaNwNsCxxK88jEx4z38Fu`
- **Cluster:** Devnet (cluster offset: 3726127828)
- **Circuit Files:** Uploaded to Supabase, initialized via `arcium init-mxe` + custom init script

---

## What I've Tried

1. Verified encryption produces correct number of ciphertexts (8)
2. Confirmed nonce is stored as u128 and converted to BN properly
3. Rebuilt circuits with `arcium build` after signature changes
4. Uploaded new `.arcis` files to Supabase
5. Reinitialized comp def accounts
6. Deployed fresh program with new program ID
7. Still getting "Invalid argument ArcisPubkey for parameter PlaintextU128"

The error suggests the Arcium runtime expects arguments in a different order or format than I'm providing.

