# 🔧 Quiz Transaction Size Fix - Implementation Plan

## Problem
Current quiz submission uses 3 separate transactions with 18+ arguments each, totaling ~4,890 bytes across 3 TXs.
This exceeds Solana's 1,232-byte per-transaction limit.

## Solution: Two-Step Quiz Submission

### Step 1: Submit Answers (Lightweight)
Student submits encrypted answers in a SMALL transaction (~300 bytes)

### Step 2: Auto-Grade (Background) 
Backend triggers MPC auto-grading separately (2-5 seconds later)

---

## Rust Program Changes

### 1. New Account Type: `QuizAnswersStorage`
```rust
#[account]
#[derive(InitSpace)]
pub struct QuizAnswersStorage {
    pub quiz: Pubkey,
    pub student: Pubkey,
    pub encrypted_answers: [u8; 128],  // Encrypted answer data
    pub answers_hash: [u8; 32],  // Hash for verification
    pub submission_timestamp: i64,
    pub grading_status: GradingStatus,  // pending, completed, failed
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum GradingStatus {
    Pending,
    Computing,
    Completed,
    Failed,
}
```

### 2. New Instruction: `submit_quiz_answers` (SMALL!)
```rust
pub fn submit_quiz_answers(
    ctx: Context<SubmitQuizAnswers>,
    encrypted_answers: [u8; 128],  // Just encrypted blob
    answers_hash: [u8; 32],  // Hash for integrity
) -> Result<()>
```

### 3. New Instruction: `compute_quiz_grade` (Separate TX)
```rust
pub fn compute_quiz_grade(
    ctx: Context<ComputeQuizGrade>,
    computation_offset: u64,
    // Minimal MPC args - references stored answers
) -> Result<()>
```

---

## Frontend Changes

### 1. RealArciumService
- Add `submitQuizAnswers()` method
- Add `computeQuizGrade()` method
- Add polling for grading results

### 2. PublicSurveys/Dashboard
- Update quiz submission UI
- Add grading progress indicator
- Poll for results

---

## Benefits
✅ Small submission transaction (~300 bytes vs 1,630 bytes)
✅ No transaction size limits
✅ Can handle unlimited quiz questions
✅ 2-5 second grading acceptable UX
✅ Maintains full Arcium MPC privacy



