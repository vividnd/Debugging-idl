// LEGACY FUNCTIONS - MOVED FROM lib.rs
// These are the OLD functions that had incorrect argument counts
// They are kept here for reference only

use anchor_lang::prelude::*;
use crate::{ErrorCode, *};

// MOVED: This was the OLD submit_quiz_response function with incorrect argument count
// It was moved here because it had the wrong signature for the circuit
pub fn submit_quiz_response_old(
    ctx: Context<SubmitQuizResponse>,
    student_computation_offset: u64,     // For student results
    instructor_computation_offset: u64,  // For instructor analytics
    creator_computation_offset: u64,     // For quiz creator analytics
    ciphertext_answer1: [u8; 32],
    ciphertext_answer2: [u8; 32],
    ciphertext_correct_answer1: [u8; 32],
    ciphertext_correct_answer2: [u8; 32],
    ciphertext_points1: [u8; 32],
    ciphertext_points2: [u8; 32],
    ciphertext_threshold: [u8; 32],  // Encrypted passing threshold
    student_pub_key: [u8; 32],       // Student's public key
    student_nonce: u128,             // Student's nonce
    instructor_pub_key: [u8; 32],    // Instructor's public key
    instructor_nonce: u128,          // Instructor's nonce
    creator_pub_key: [u8; 32],       // Creator's public key
    creator_nonce: u128,             // Creator's nonce
) -> Result<()> {
    // This was the OLD function that only had 9 arguments
    // The quiz_evaluation circuit expects 19 arguments
    // This function was moved here and replaced with a new one in lib.rs
    
    msg!("⚠️  This is the OLD function that was moved to legacy_functions.rs");
    msg!("⚠️  It had incorrect argument count for the circuit");
    
    // Set the sign PDA account bump
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
    
    // Validate quiz is active
    require!(ctx.accounts.quiz.is_active, ErrorCode::SurveyInactive);
    
    // Check if quiz has reached max responses
    require!(
        ctx.accounts.quiz.current_responses < ctx.accounts.quiz.max_responses,
        ErrorCode::SurveyFull
    );
    
    // Set the sign PDA account bump
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

    // OLD queuing with only 9 arguments (WRONG - circuit expects 19)
    let student_args = vec![
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        Argument::EncryptedU32(ciphertext_answer1),
        Argument::EncryptedU32(ciphertext_answer2),
        Argument::EncryptedU32(ciphertext_correct_answer1),
        Argument::EncryptedU32(ciphertext_correct_answer2),
        Argument::EncryptedU32(ciphertext_points1),
        Argument::EncryptedU32(ciphertext_points2),
        Argument::EncryptedU8(ciphertext_threshold),
    ];

    queue_computation(
        ctx.accounts,
        student_computation_offset,
        student_args,
        None,
        vec![QuizEvaluationCallback::callback_ix(&[])],
    )?;

    // Similar old queuing for instructor and creator...
    
    Ok(())
}

// LEGACY: verify_quiz_completion with incorrect argument count (5 args instead of 11)
// This function will cause InvalidArguments errors with the current circuit signatures
#[deprecated(note = "Use verify_quiz_completion_v2 instead - this has incorrect argument count")]
pub fn verify_quiz_completion_legacy(
    ctx: Context<VerifyQuizCompletion>,
    quiz: Pubkey,
    student_computation_offset: u64,     // For student verification result
    access_controller_computation_offset: u64, // For access controller result
    auditor_computation_offset: u64,     // For auditor result
    encrypted_score: [u8; 32], // This will be processed by MPC
    encrypted_threshold: [u8; 32], // Encrypted threshold
    encrypted_requirement: [u8; 32], // Encrypted additional requirement
    student_pub_key: [u8; 32], // Student's public key
    student_nonce: u128,       // Student's nonce
) -> Result<()> {
    // This function has the OLD signature with only 5 arguments
    // The quiz_threshold_check circuit now expects 11 arguments
    // This will cause InvalidArguments errors
    
    msg!("⚠️  WARNING: Using legacy function with incorrect argument count!");
    msg!("⚠️  quiz_threshold_check circuit expects 11 arguments, but this provides only 5");
    msg!("⚠️  Use verify_quiz_completion_v2 instead");
    
    // Set the sign PDA account bump
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
    
    let clock = Clock::get()?;
    
    // Verify the quiz exists and is active
    require!(ctx.accounts.quiz.is_active, ErrorCode::SurveyInactive);
    
    // Verify the quiz has special survey enabled
    require!(
        matches!(ctx.accounts.quiz.survey_type, SurveyType::Quiz { special_survey_enabled: true, .. }),
        ErrorCode::InvalidSurveyType
    );
    
    // Set the sign PDA account bump
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

    // LEGACY: This queuing will FAIL because it only provides 5 arguments
    // but quiz_threshold_check circuit expects 11 arguments
    let student_args = vec![
        Argument::ArcisPubkey(student_pub_key),
        Argument::PlaintextU128(student_nonce),
        Argument::EncryptedU32(encrypted_score),
        Argument::EncryptedU8(encrypted_threshold),
        Argument::EncryptedU32(encrypted_requirement),
    ];

    // This will fail with InvalidArguments error
    queue_computation(
        ctx.accounts,
        student_computation_offset,
        student_args,
        None,
        vec![QuizThresholdCheckCallback::callback_ix(&[
            CallbackAccount {
                pubkey: ctx.accounts.completion_proof.key(),
                is_writable: true,
            }
        ])],
    )?;
    
    Ok(())
}

/*
LEGACY FUNCTION REFERENCE:

These functions were moved here because they have incorrect argument counts
that don't match the current circuit signatures:

1. submit_quiz_response_legacy:
   - Provides: 9 arguments
   - Circuit expects: 19 arguments
   - Missing: class_total_students, class_total_score, class_completion_count, passing_requirement
   - Plus missing Shared struct arguments for instructor and quiz_creator

2. verify_quiz_completion_legacy:
   - Provides: 5 arguments  
   - Circuit expects: 11 arguments
   - Missing: access_controller and auditor Shared struct arguments

The new functions in lib.rs have the correct argument counts and will work
properly with the Arcium MPC circuits.
*/
