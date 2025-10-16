// Removed deprecated feature suppression - using current Arcium APIs
use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::{CallbackAccount, CircuitSource, OffChainCircuitSource};
// MXEAccount is available from arcium_anchor::prelude::*

// Legacy functions moved to separate file for reference
mod legacy_functions;

// Computation definition offsets for our survey/quiz DApp
const COMP_DEF_OFFSET_SURVEY_ANALYTICS: u32 = comp_def_offset("survey_analytics");
const COMP_DEF_OFFSET_QUIZ_EVALUATION: u32 = comp_def_offset("quiz_evaluation");
const COMP_DEF_OFFSET_ANALYTICS: u32 = comp_def_offset("analytics_computation");
const COMP_DEF_OFFSET_QUIZ_THRESHOLD: u32 = comp_def_offset("quiz_threshold_check");

// ✅ FIXED: Using SIGN_PDA_SEED from arcium_anchor::prelude::* instead of manual definition

// Arcium program ID for owner constraints
pub const ARCIUM_PROGRAM_ID: Pubkey = pubkey!("BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6");

// ✅ FIXED: Arcium v0.3.0 constants (following SDK patterns)
// These constants should be available from arcium_anchor::prelude::*
// If not available, they will be defined by the SDK during compilation

declare_id!("5qfZir789yPZ9vQoSYt6mnWtbapP2FyqHnKogvswfC69");

// ✅ ARCIUM MPC: Callback account structures MUST be defined BEFORE the program module
// ✅ FIXED: Moved callback structs outside the program module to fix visibility issues
// ✅ FIXED: Reordered callback structs to fix macro processing order issue

// ✅ RESTORED: Manual callback account struct definitions (required by Arcium v0.3.0)
// These must be defined manually - Arcium does not auto-generate them

#[callback_accounts("survey_analytics")]
#[derive(Accounts)]
pub struct SurveyAnalyticsCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_SURVEY_ANALYTICS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    /// CHECK: instructions_sysvar, checked by the account constraint
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
    
    // ✅ FIX: Add callback accounts that are passed from queue_computation
    #[account(mut)]
    pub analytics_storage: Account<'info, SurveyAnalyticsStorage>,
    
    #[account(mut)]
    pub survey: Account<'info, Survey>,
}

#[callback_accounts("quiz_evaluation")]
#[derive(Accounts)]
pub struct QuizEvaluationCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_EVALUATION))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    /// CHECK: instructions_sysvar, checked by the account constraint
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}

#[callback_accounts("analytics_computation")]
#[derive(Accounts)]
pub struct AnalyticsComputationCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_ANALYTICS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    /// CHECK: instructions_sysvar, checked by the account constraint
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}

#[callback_accounts("quiz_threshold_check")]
#[derive(Accounts)]
pub struct QuizThresholdCheckCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_THRESHOLD))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    /// CHECK: instructions_sysvar, checked by the account constraint
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub completion_proof: Account<'info, QuizCompletionProof>,
}

// ✅ RESTORED: Manual computation definition initialization functions (from backup)
#[init_computation_definition_accounts("survey_analytics", payer)]
#[derive(Accounts)]
pub struct InitSurveyAnalyticsCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by the arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[init_computation_definition_accounts("quiz_evaluation", payer)]
#[derive(Accounts)]
pub struct InitQuizEvaluationCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by the arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[init_computation_definition_accounts("analytics_computation", payer)]
#[derive(Accounts)]
pub struct InitAnalyticsComputationCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by the arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[init_computation_definition_accounts("quiz_threshold_check", payer)]
#[derive(Accounts)]
pub struct InitQuizThresholdCheckCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by the arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

// Helper function to calculate expiration timestamp
fn calculate_expiration_timestamp(
    current_timestamp: i64,
    value: u32,
    unit: &ExpirationUnit,
) -> Result<i64> {
    let seconds_to_add = match unit {
        ExpirationUnit::Minutes => {
            let minutes = value as i64;
            minutes.checked_mul(60).ok_or::<anchor_lang::error::Error>(ErrorCode::IntegerOverflow.into())?
        },
        ExpirationUnit::Hours => {
            let hours = value as i64;
            hours.checked_mul(60 * 60).ok_or::<anchor_lang::error::Error>(ErrorCode::IntegerOverflow.into())?
        },
        ExpirationUnit::Days => {
            let days = value as i64;
            days.checked_mul(24 * 60 * 60).ok_or::<anchor_lang::error::Error>(ErrorCode::IntegerOverflow.into())?
        },
    };
    
    current_timestamp.checked_add(seconds_to_add).ok_or::<anchor_lang::error::Error>(ErrorCode::IntegerOverflow.into())
}

// ✅ REMOVED: Fake hash function - using direct user references instead

// Helper function to validate expiration configuration
fn validate_expiration_config(
    value: u32,
    unit: &ExpirationUnit,
) -> Result<()> {
    match unit {
        ExpirationUnit::Minutes => {
            require!(value > 0 && value <= 525600, ErrorCode::InvalidExpirationValue); // Max 1 year in minutes
        },
        ExpirationUnit::Hours => {
            require!(value > 0 && value <= 8760, ErrorCode::InvalidExpirationValue); // Max 1 year in hours
        },
        ExpirationUnit::Days => {
            require!(value > 0 && value <= 365, ErrorCode::InvalidExpirationValue); // Max 1 year in days
        },
    }
    Ok(())
}

