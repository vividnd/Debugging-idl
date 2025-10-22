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

declare_id!("8h3kubFYaqYjGKg6Lk6baoKxyPpEus6aS8QZaGyY7tZC");

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

#[arcium_program]
pub mod se_qure {
    use super::*;
    
    // ✅ FIXED: Removed manual callback struct imports
    // Arcium automatically generates these types when using #[callback_accounts] macro

    // ✅ REMOVED: Duplicate callback functions - Arcium macros auto-generate these



    // ✅ REVERTED: Use standard Arcium v0.3.0 patterns without problematic macros
    // Note: Computation definitions will be initialized via standard Anchor patterns

    // ✅ FIXED: Initialize Sign PDA Account for cross-program invocations
    pub fn init_sign_pda(ctx: Context<InitSignPda>) -> Result<()> {
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        Ok(())
    }

    // ✅ UPDATED: Computation definition initialization with Supabase circuit URLs
    pub fn init_survey_analytics_comp_def(ctx: Context<InitSurveyAnalyticsCompDef>) -> Result<()> {
        init_comp_def(
            ctx.accounts,
            true,
            0,
            Some(CircuitSource::OffChain(OffChainCircuitSource {
                source: "https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public/arcium-circuits/survey_analytics.arcis".to_string(),
                hash: [0; 32], // Hash verification not enforced yet
            })),
            None,
        )?;
        Ok(())
    }

    pub fn init_quiz_evaluation_comp_def(ctx: Context<InitQuizEvaluationCompDef>) -> Result<()> {
        init_comp_def(
            ctx.accounts,
            true,
            0,
            Some(CircuitSource::OffChain(OffChainCircuitSource {
                source: "https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public/arcium-circuits/quiz_evaluation.arcis".to_string(),
                hash: [0; 32], // Hash verification not enforced yet
            })),
            None,
        )?;
        Ok(())
    }

    pub fn init_analytics_computation_comp_def(ctx: Context<InitAnalyticsComputationCompDef>) -> Result<()> {
        init_comp_def(
            ctx.accounts,
            true,
            0,
            Some(CircuitSource::OffChain(OffChainCircuitSource {
                source: "https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public/arcium-circuits/analytics_computation.arcis".to_string(),
                hash: [0; 32], // Hash verification not enforced yet
            })),
            None,
        )?;
        Ok(())
    }

    pub fn init_quiz_threshold_check_comp_def(ctx: Context<InitQuizThresholdCheckCompDef>) -> Result<()> {
        init_comp_def(
            ctx.accounts,
            true,
            0,
            Some(CircuitSource::OffChain(OffChainCircuitSource {
                source: "https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public/arcium-circuits/quiz_threshold_check.arcis".to_string(),
                hash: [0; 32], // Hash verification not enforced yet
            })),
            None,
        )?;
        Ok(())
    }


    // ✅ FIXED: Store application preferences for survey/quiz computations
    // Note: This is application-level metadata, not actual Arcium MXE configuration
    // MXE configuration happens through Arcium's infrastructure, not custom program accounts
    pub fn set_survey_computation_preferences(
        ctx: Context<SetSurveyComputationPreferences>,
        preferred_encryption_scheme: EncryptionScheme,
        preferred_data_provisioning: DataProvisioningMethod,
        preferred_mpc_protocol: MPCProtocol,
    ) -> Result<()> {
        // Store application preferences for survey/quiz computations
        // These are used to guide computation setup, not to configure Arcium's MXE
        
        // Validate preferences are supported by our application
        require!(
            matches!(preferred_encryption_scheme, EncryptionScheme::Rescue | EncryptionScheme::AES),
            ErrorCode::UnsupportedEncryptionScheme
        );
        
        // Validate data provisioning preference
        require!(
            matches!(preferred_data_provisioning, DataProvisioningMethod::Direct | DataProvisioningMethod::Batch),
            ErrorCode::UnsupportedDataProvisioning
        );
        
        // Validate MPC protocol preference
        require!(
            matches!(preferred_mpc_protocol, MPCProtocol::Cerberus | MPCProtocol::Manticore),
            ErrorCode::UnsupportedMPCProtocol
        );
        
        // Store application preferences (not Arcium MXE configuration)
        let app_preferences = &mut ctx.accounts.app_preferences;
        app_preferences.admin = ctx.accounts.admin.key();
        app_preferences.preferred_encryption_scheme = preferred_encryption_scheme.clone();
        app_preferences.preferred_data_provisioning = preferred_data_provisioning.clone();
        app_preferences.preferred_mpc_protocol = preferred_mpc_protocol.clone();
        app_preferences.configured_at = Clock::get()?.unix_timestamp;
        app_preferences.is_active = true;
        
        emit!(SurveyComputationPreferencesSet {
            preferred_encryption_scheme: format!("{:?}", preferred_encryption_scheme),
            preferred_data_provisioning: format!("{:?}", preferred_data_provisioning),
            preferred_mpc_protocol: format!("{:?}", preferred_mpc_protocol),
            configured_at: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }



    // Create a new survey with timestamp-based PDA (allows multiple surveys per creator)
    // Only stores verification data on blockchain - sensitive data encrypted in Supabase
    pub fn create_survey_with_timestamp(
        ctx: Context<CreateSurveyWithTimestamp>,
        _timestamp: i64,
        slug: String,
        title: String,
        description: String,
        survey_type: SurveyType,
        max_responses: u32,
    ) -> Result<()> {
        let survey = &mut ctx.accounts.survey;
        let clock = Clock::get()?;
        
        // Input validation with security checks
        validate_slug(&slug)?;
        validate_string_length(&title, MAX_TITLE_LENGTH, "title")?;
        validate_string_length(&description, MAX_DESCRIPTION_LENGTH, "description")?;
        validate_max_responses(max_responses)?;
        
        // Security validations
        validate_no_xss(&title)?;
        validate_no_xss(&description)?;
        validate_no_sql_injection(&title)?;
        validate_no_sql_injection(&description)?;
        
        // ✅ FIXED: Validate space requirements before account creation
        let required_space = Survey::calculate_space(&slug, &title);
        require!(
            required_space <= 15000, // Increased limit to accommodate questions storage
            ErrorCode::SurveyTooLarge
        );
        
        // Validate quiz expiration configuration if it's a quiz
        if let SurveyType::Quiz { completion_proof_expiration_value, completion_proof_expiration_unit, .. } = &survey_type {
            validate_expiration_config(*completion_proof_expiration_value, completion_proof_expiration_unit)?;
        }
        
        survey.creator = ctx.accounts.creator.key();
        survey.slug = slug;
        survey.title = title;
        survey.description = description;
        survey.survey_type = survey_type;
        survey.questions = Vec::new();  // ✅ FIXED: Initialize questions as empty vector
        survey.max_responses = max_responses;
        survey.current_responses = 0;  // Initialize response counter
        survey.is_active = true;
        survey.is_publicly_browsable = true;  // Regular surveys are publicly browsable
        survey.modification_count = 0;  // Initialize modification counter
        survey.created_at = clock.unix_timestamp;
        survey.mpc_computation_id = 0;  // ✅ FIXED: Initialize MPC computation reference

        emit!(SurveyCreated {
            survey: survey.key(),
            survey_type: match survey.survey_type {
                SurveyType::Basic => "basic".to_string(),
                SurveyType::Quiz { .. } => "quiz".to_string(),
                SurveyType::Special => "special".to_string(),
            },
            created_at: clock.unix_timestamp,
        });

        Ok(())
    }

    // Add questions to a survey
    pub fn add_questions(
        ctx: Context<AddQuestions>,
        questions: Vec<QuestionData>,
    ) -> Result<()> {
        // Input validation
        validate_questions(&questions)?;
        
        let survey = &mut ctx.accounts.survey;
        
        require!(
            survey.creator == ctx.accounts.creator.key(),
            ErrorCode::Unauthorized
        );

        let question_count = questions.len() as u32;
        
        for question_data in questions {
            survey.questions.push(question_data);
        }

        emit!(QuestionsAdded {
            survey: survey.key(),
            question_count,
        });

        Ok(())
    }

    // Separate instruction for analytics computation
    pub fn submit_survey_analytics(
        ctx: Context<SubmitSurveyAnalytics>,
        analytics_computation_offset: u64,    // For creator analytics
        // answer1: Enc<Shared, u32> (3 args)
        answer1_pub_key: [u8; 32],           // Encryption key for answer1
        answer1_nonce: u128,                 // Nonce for answer1
        ciphertext_answer1: [u8; 32],        // Encrypted answer1
        // answer2: Enc<Shared, u32> (3 args)
        answer2_pub_key: [u8; 32],           // Encryption key for answer2
        answer2_nonce: u128,                 // Nonce for answer2
        ciphertext_answer2: [u8; 32],        // Encrypted answer2
        // question_type1: Enc<Shared, u32> (3 args)
        question_type1_pub_key: [u8; 32],    // Encryption key for question_type1
        question_type1_nonce: u128,          // Nonce for question_type1
        ciphertext_question_type1: [u8; 32], // Encrypted question_type1
        // question_type2: Enc<Shared, u32> (3 args)
        question_type2_pub_key: [u8; 32],    // Encryption key for question_type2
        question_type2_nonce: u128,          // Nonce for question_type2
        ciphertext_question_type2: [u8; 32], // Encrypted question_type2
        // total_responses: Enc<Shared, u32> (3 args)
        total_responses_pub_key: [u8; 32],   // Encryption key for total_responses
        total_responses_nonce: u128,         // Nonce for total_responses
        ciphertext_total_responses: [u8; 32], // Encrypted total_responses
        // completion_rate: Enc<Shared, u32> (3 args)
        completion_rate_pub_key: [u8; 32],   // Encryption key for completion_rate
        completion_rate_nonce: u128,         // Nonce for completion_rate
        ciphertext_completion_rate: [u8; 32], // Encrypted completion_rate
        // survey_creator: Shared (2 args)
        survey_creator_pub_key: [u8; 32],    // survey_creator public key
        survey_creator_nonce: u128,          // survey_creator nonce
        // public_viewer: Shared (2 args)
        public_viewer_pub_key: [u8; 32],     // public_viewer public key
        public_viewer_nonce: u128,           // public_viewer nonce
        // respondent: Shared (2 args)
        respondent_pub_key: [u8; 32],        // respondent public key
        respondent_nonce: u128,              // respondent nonce
    ) -> Result<()> {
        // Set the sign PDA account bump
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        
        // Validate survey is active
        require!(ctx.accounts.survey.is_active, ErrorCode::SurveyInactive);
        
        // Check if survey has reached max responses
        require!(
            ctx.accounts.survey.current_responses < ctx.accounts.survey.max_responses,
            ErrorCode::SurveyFull
        );
        
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // Computation 1: Analytics for survey creator
        // The survey_analytics function expects:
        // 1. 6 Enc<Shared, u32> = 18 args (3 args each: pubkey + nonce + encrypted_data)
        // 2. 3 Shared structs = 6 args (2 args each)
        // Total: 24 arguments
        let creator_args = vec![
            // answer1: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(answer1_pub_key),           // Encryption key for answer1
            Argument::PlaintextU128(answer1_nonce),           // Nonce for answer1
            Argument::EncryptedU32(ciphertext_answer1),       // Encrypted answer1
            // answer2: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(answer2_pub_key),           // Encryption key for answer2
            Argument::PlaintextU128(answer2_nonce),           // Nonce for answer2
            Argument::EncryptedU32(ciphertext_answer2),       // Encrypted answer2
            // question_type1: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(question_type1_pub_key),    // Encryption key for question_type1
            Argument::PlaintextU128(question_type1_nonce),    // Nonce for question_type1
            Argument::EncryptedU32(ciphertext_question_type1), // Encrypted question_type1
            // question_type2: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(question_type2_pub_key),    // Encryption key for question_type2
            Argument::PlaintextU128(question_type2_nonce),    // Nonce for question_type2
            Argument::EncryptedU32(ciphertext_question_type2), // Encrypted question_type2
            // total_responses: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(total_responses_pub_key),   // Encryption key for total_responses
            Argument::PlaintextU128(total_responses_nonce),   // Nonce for total_responses
            Argument::EncryptedU32(ciphertext_total_responses), // Encrypted total_responses
            // completion_rate: Enc<Shared, u32> (3 args)
            Argument::ArcisPubkey(completion_rate_pub_key),   // Encryption key for completion_rate
            Argument::PlaintextU128(completion_rate_nonce),   // Nonce for completion_rate
            Argument::EncryptedU32(ciphertext_completion_rate), // Encrypted completion_rate
            // survey_creator: Shared (2 args)
            Argument::ArcisPubkey(survey_creator_pub_key),    // survey_creator public key
            Argument::PlaintextU128(survey_creator_nonce),    // survey_creator nonce
            // public_viewer: Shared (2 args)
            Argument::ArcisPubkey(public_viewer_pub_key),     // public_viewer public key
            Argument::PlaintextU128(public_viewer_nonce),     // public_viewer nonce
            // respondent: Shared (2 args)
            Argument::ArcisPubkey(respondent_pub_key),        // respondent public key
            Argument::PlaintextU128(respondent_nonce),        // respondent nonce
        ];

        // Single computation for analytics
        queue_computation(
            ctx.accounts,
            analytics_computation_offset,
            creator_args,
            None,
            vec![SurveyAnalyticsCallback::callback_ix(&[
                CallbackAccount {
                    pubkey: ctx.accounts.analytics_storage.key(),
                    is_writable: true,
                },
                CallbackAccount {
                    pubkey: ctx.accounts.survey.key(),
                    is_writable: true,
                },
            ])],
        )?;

        // Increment response counter
        ctx.accounts.survey.current_responses += 1;

        // Emit event for tracking - avoid wallet exposure
        emit!(ResponseSubmitted {
            survey: ctx.accounts.survey.key(),
        });

        Ok(())
    }

    // Separate instruction for feedback computation
    pub fn submit_survey_feedback(
        ctx: Context<SubmitSurveyFeedback>,
        feedback_computation_offset: u64,     // For respondent feedback
        // answer1: Enc<Shared, u32> (3 args)
        answer1_pub_key: [u8; 32],           // Encryption key for answer1
        answer1_nonce: u128,                 // Nonce for answer1
        ciphertext_answer1: [u8; 32],        // Encrypted answer1
        // answer2: Enc<Shared, u32> (3 args)
        answer2_pub_key: [u8; 32],           // Encryption key for answer2
        answer2_nonce: u128,                 // Nonce for answer2
        ciphertext_answer2: [u8; 32],        // Encrypted answer2
        // question_type1: Enc<Shared, u32> (3 args)
        question_type1_pub_key: [u8; 32],    // Encryption key for question_type1
        question_type1_nonce: u128,          // Nonce for question_type1
        ciphertext_question_type1: [u8; 32], // Encrypted question_type1
        // question_type2: Enc<Shared, u32> (3 args)
        question_type2_pub_key: [u8; 32],    // Encryption key for question_type2
        question_type2_nonce: u128,          // Nonce for question_type2
        ciphertext_question_type2: [u8; 32], // Encrypted question_type2
        // total_responses: Enc<Shared, u32> (3 args)
        total_responses_pub_key: [u8; 32],   // Encryption key for total_responses
        total_responses_nonce: u128,         // Nonce for total_responses
        ciphertext_total_responses: [u8; 32], // Encrypted total_responses
        // completion_rate: Enc<Shared, u32> (3 args)
        completion_rate_pub_key: [u8; 32],   // Encryption key for completion_rate
        completion_rate_nonce: u128,         // Nonce for completion_rate
        ciphertext_completion_rate: [u8; 32], // Encrypted completion_rate
        // survey_creator: Shared (2 args)
        survey_creator_pub_key: [u8; 32],    // survey_creator public key
        survey_creator_nonce: u128,          // survey_creator nonce
        // public_viewer: Shared (2 args)
        public_viewer_pub_key: [u8; 32],     // public_viewer public key
        public_viewer_nonce: u128,           // public_viewer nonce
        // respondent: Shared (2 args)
        respondent_pub_key: [u8; 32],        // respondent public key
        respondent_nonce: u128,              // respondent nonce
    ) -> Result<()> {
        // Set sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // Build arguments for survey_analytics circuit (24 arguments total)
        // Using shared encryption: all Enc<Shared, u32> values use the same key/nonce
        let respondent_args = vec![
            // answer1: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(answer1_pub_key),           // shared encryption key
            Argument::PlaintextU128(answer1_nonce),           // shared nonce
            Argument::EncryptedU32(ciphertext_answer1),       // answer1 ciphertext
            
            // answer2: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(answer2_pub_key),           // shared encryption key (same as answer1)
            Argument::PlaintextU128(answer2_nonce),           // shared nonce (same as answer1)
            Argument::EncryptedU32(ciphertext_answer2),       // answer2 ciphertext
            
            // question_type1: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(question_type1_pub_key),    // shared encryption key (same as answer1)
            Argument::PlaintextU128(question_type1_nonce),    // shared nonce (same as answer1)
            Argument::EncryptedU32(ciphertext_question_type1), // question_type1 ciphertext
            
            // question_type2: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(question_type2_pub_key),    // shared encryption key (same as answer1)
            Argument::PlaintextU128(question_type2_nonce),    // shared nonce (same as answer1)
            Argument::EncryptedU32(ciphertext_question_type2), // question_type2 ciphertext
            
            // total_responses: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(total_responses_pub_key),   // shared encryption key (same as answer1)
            Argument::PlaintextU128(total_responses_nonce),   // shared nonce (same as answer1)
            Argument::EncryptedU32(ciphertext_total_responses), // total_responses ciphertext
            
            // completion_rate: Enc<Shared, u32> (3 args) - using shared key/nonce
            Argument::ArcisPubkey(completion_rate_pub_key),   // shared encryption key (same as answer1)
            Argument::PlaintextU128(completion_rate_nonce),   // shared nonce (same as answer1)
            Argument::EncryptedU32(ciphertext_completion_rate), // completion_rate ciphertext
            
            // survey_creator: Shared (2 args)
            Argument::ArcisPubkey(survey_creator_pub_key),    // survey_creator public key
            Argument::PlaintextU128(survey_creator_nonce),    // survey_creator nonce
            
            // public_viewer: Shared (2 args)
            Argument::ArcisPubkey(public_viewer_pub_key),     // public_viewer public key
            Argument::PlaintextU128(public_viewer_nonce),     // public_viewer nonce
            
            // respondent: Shared (2 args)
            Argument::ArcisPubkey(respondent_pub_key),        // respondent public key
            Argument::PlaintextU128(respondent_nonce),        // respondent nonce
        ];

        // Single computation for feedback
        queue_computation(
            ctx.accounts,
            feedback_computation_offset,
            respondent_args,
            None,
            vec![SurveyAnalyticsCallback::callback_ix(&[])],
        )?;

        Ok(())
    }

    // Separate instruction for student quiz computation
    pub fn submit_quiz_student(
        ctx: Context<SubmitQuizStudent>,
        student_computation_offset: u64,     // For student results
        ciphertext_answer1: [u8; 32],
        ciphertext_answer2: [u8; 32],
        ciphertext_correct_answer1: [u8; 32],
        ciphertext_correct_answer2: [u8; 32],
        ciphertext_points1: [u8; 32],
        ciphertext_points2: [u8; 32],
        ciphertext_threshold: [u8; 32],  // Encrypted passing threshold
        ciphertext_class_total_students: [u8; 32],  // Encrypted class total students
        ciphertext_class_total_score: [u8; 32],     // Encrypted class total score
        ciphertext_class_completion_count: [u8; 32], // Encrypted class completion count
        ciphertext_passing_requirement: [u8; 32],   // Encrypted passing requirement
        student_pub_key: [u8; 32],       // Student's public key
        student_nonce: u128,             // Student's nonce
        instructor_pub_key: [u8; 32],    // Instructor's public key
        instructor_nonce: u128,          // Instructor's nonce
        creator_pub_key: [u8; 32],       // Creator's public key
        creator_nonce: u128,             // Creator's nonce
    ) -> Result<()> {
        // Validate quiz is active
        require!(ctx.accounts.quiz.is_active, ErrorCode::SurveyInactive);
        
        // Check if quiz has reached max responses
        require!(
            ctx.accounts.quiz.current_responses < ctx.accounts.quiz.max_responses,
            ErrorCode::SurveyFull
        );
        
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // Single computation for student results
        let student_args = vec![
            // user_answer1: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer1
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer1
            Argument::EncryptedU32(ciphertext_answer1),       // Encrypted user_answer1
            // user_answer2: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer2
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer2
            Argument::EncryptedU32(ciphertext_answer2),       // Encrypted user_answer2
            // correct_answer1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer1), // Encrypted correct_answer1
            // correct_answer2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer2), // Encrypted correct_answer2
            // points1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points1),       // Encrypted points1
            // points2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points2),       // Encrypted points2
            // passing_threshold: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_threshold),     // Encrypted passing_threshold
            // class_total_students: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_students), // Encrypted class_total_students
            // class_total_score: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_score), // Encrypted class_total_score
            // class_completion_count: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_completion_count), // Encrypted class_completion_count
            // student: Shared - 2 args
            Argument::ArcisPubkey(student_pub_key),           // student public key
            Argument::PlaintextU128(student_nonce),           // student nonce
            // instructor: Shared - 2 args
            Argument::ArcisPubkey(instructor_pub_key),        // instructor public key
            Argument::PlaintextU128(instructor_nonce),        // instructor nonce
            // quiz_creator: Shared - 2 args
            Argument::ArcisPubkey(creator_pub_key),           // quiz_creator public key
            Argument::PlaintextU128(creator_nonce),           // quiz_creator nonce
        ];

        queue_computation(
            ctx.accounts,
            student_computation_offset,
            student_args,
            None,
            vec![QuizEvaluationCallback::callback_ix(&[])],
        )?;

        // Increment response counter
        ctx.accounts.quiz.current_responses += 1;

        // Emit event for tracking - avoid wallet exposure
        emit!(QuizResponseSubmitted {
            quiz: ctx.accounts.quiz.key(),
        });

        Ok(())
    }

    // Separate instruction for instructor quiz computation
    pub fn submit_quiz_instructor(
        ctx: Context<SubmitQuizInstructor>,
        instructor_computation_offset: u64,  // For instructor analytics
        ciphertext_answer1: [u8; 32],
        ciphertext_answer2: [u8; 32],
        ciphertext_correct_answer1: [u8; 32],
        ciphertext_correct_answer2: [u8; 32],
        ciphertext_points1: [u8; 32],
        ciphertext_points2: [u8; 32],
        ciphertext_threshold: [u8; 32],  // Encrypted passing threshold
        ciphertext_class_total_students: [u8; 32],  // Encrypted class total students
        ciphertext_class_total_score: [u8; 32],     // Encrypted class total score
        ciphertext_class_completion_count: [u8; 32], // Encrypted class completion count
        ciphertext_passing_requirement: [u8; 32],   // Encrypted passing requirement
        student_pub_key: [u8; 32],       // Student's public key
        student_nonce: u128,             // Student's nonce
        instructor_pub_key: [u8; 32],    // Instructor's public key
        instructor_nonce: u128,          // Instructor's nonce
        creator_pub_key: [u8; 32],       // Creator's public key
        creator_nonce: u128,             // Creator's nonce
    ) -> Result<()> {
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // Single computation for instructor analytics
        let instructor_args = vec![
            // user_answer1: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer1
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer1
            Argument::EncryptedU32(ciphertext_answer1),       // Encrypted user_answer1
            // user_answer2: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer2
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer2
            Argument::EncryptedU32(ciphertext_answer2),       // Encrypted user_answer2
            // correct_answer1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer1), // Encrypted correct_answer1
            // correct_answer2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer2), // Encrypted correct_answer2
            // points1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points1),       // Encrypted points1
            // points2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points2),       // Encrypted points2
            // passing_threshold: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_threshold),     // Encrypted passing_threshold
            // class_total_students: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_students), // Encrypted class_total_students
            // class_total_score: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_score), // Encrypted class_total_score
            // class_completion_count: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_completion_count), // Encrypted class_completion_count
            // student: Shared - 2 args
            Argument::ArcisPubkey(student_pub_key),           // student public key
            Argument::PlaintextU128(student_nonce),           // student nonce
            // instructor: Shared - 2 args
            Argument::ArcisPubkey(instructor_pub_key),        // instructor public key
            Argument::PlaintextU128(instructor_nonce),        // instructor nonce
            // quiz_creator: Shared - 2 args
            Argument::ArcisPubkey(creator_pub_key),           // quiz_creator public key
            Argument::PlaintextU128(creator_nonce),           // quiz_creator nonce
        ];

        queue_computation(
            ctx.accounts,
            instructor_computation_offset,
            instructor_args,
            None,
            vec![QuizEvaluationCallback::callback_ix(&[])],
        )?;

        Ok(())
    }

    // Separate instruction for creator quiz computation
    pub fn submit_quiz_creator(
        ctx: Context<SubmitQuizCreator>,
        creator_computation_offset: u64,     // For quiz creator analytics
        ciphertext_answer1: [u8; 32],
        ciphertext_answer2: [u8; 32],
        ciphertext_correct_answer1: [u8; 32],
        ciphertext_correct_answer2: [u8; 32],
        ciphertext_points1: [u8; 32],
        ciphertext_points2: [u8; 32],
        ciphertext_threshold: [u8; 32],  // Encrypted passing threshold
        ciphertext_class_total_students: [u8; 32],  // Encrypted class total students
        ciphertext_class_total_score: [u8; 32],     // Encrypted class total score
        ciphertext_class_completion_count: [u8; 32], // Encrypted class completion count
        ciphertext_passing_requirement: [u8; 32],   // Encrypted passing requirement
        student_pub_key: [u8; 32],       // Student's public key
        student_nonce: u128,             // Student's nonce
        instructor_pub_key: [u8; 32],    // Instructor's public key
        instructor_nonce: u128,          // Instructor's nonce
        creator_pub_key: [u8; 32],       // Creator's public key
        creator_nonce: u128,             // Creator's nonce
    ) -> Result<()> {
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // Single computation for creator analytics
        let creator_args = vec![
            // user_answer1: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer1
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer1
            Argument::EncryptedU32(ciphertext_answer1),       // Encrypted user_answer1
            // user_answer2: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for user_answer2
            Argument::PlaintextU128(student_nonce),           // Nonce for user_answer2
            Argument::EncryptedU32(ciphertext_answer2),       // Encrypted user_answer2
            // correct_answer1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer1), // Encrypted correct_answer1
            // correct_answer2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_correct_answer2), // Encrypted correct_answer2
            // points1: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points1),       // Encrypted points1
            // points2: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_points2),       // Encrypted points2
            // passing_threshold: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_threshold),     // Encrypted passing_threshold
            // class_total_students: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_students), // Encrypted class_total_students
            // class_total_score: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_total_score), // Encrypted class_total_score
            // class_completion_count: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(ciphertext_class_completion_count), // Encrypted class_completion_count
            // student: Shared - 2 args
            Argument::ArcisPubkey(student_pub_key),           // student public key
            Argument::PlaintextU128(student_nonce),           // student nonce
            // instructor: Shared - 2 args
            Argument::ArcisPubkey(instructor_pub_key),        // instructor public key
            Argument::PlaintextU128(instructor_nonce),        // instructor nonce
            // quiz_creator: Shared - 2 args
            Argument::ArcisPubkey(creator_pub_key),           // quiz_creator public key
            Argument::PlaintextU128(creator_nonce),           // quiz_creator nonce
        ];

        queue_computation(
            ctx.accounts,
            creator_computation_offset,
            creator_args,
            None,
            vec![QuizEvaluationCallback::callback_ix(&[])],
        )?;

        Ok(())
    }

    // Create special survey for high scorers
    pub fn create_special_survey(
        ctx: Context<CreateSpecialSurvey>,
        title: String,
        description: String,
        max_responses: u32,
    ) -> Result<()> {
        // Input validation
        validate_string_length(&title, MAX_TITLE_LENGTH, "title")?;
        validate_string_length(&description, MAX_DESCRIPTION_LENGTH, "description")?;
        validate_max_responses(max_responses)?;
        
        let special_survey = &mut ctx.accounts.special_survey;
        let quiz = &mut ctx.accounts.quiz;
        let clock = Clock::get()?;

        require!(
            quiz.creator == ctx.accounts.creator.key(),
            ErrorCode::Unauthorized
        );

        // Only allow special survey if enabled on the quiz
        require!(
            matches!(quiz.survey_type, SurveyType::Quiz { special_survey_enabled: true, .. }),
            ErrorCode::InvalidSurveyType
        );

        special_survey.creator = ctx.accounts.creator.key();
        special_survey.title = title;
        special_survey.description = description;
        special_survey.survey_type = SurveyType::Special;
        special_survey.max_responses = max_responses;
        special_survey.current_responses = 0;  // Initialize response counter
        special_survey.is_active = true;
        special_survey.is_publicly_browsable = false;  // Special surveys are NOT publicly browsable
        special_survey.created_at = clock.unix_timestamp;

        emit!(SpecialSurveyCreated {
            quiz: quiz.key(),
            special_survey: special_survey.key(),
        });

        Ok(())
    }

    /// ✅ NEW: Verify quiz threshold to grant special survey access
    /// 
    /// This instruction queues the `quiz_threshold_check` computation with correct argument count (11 args).
    /// The arguments passed here MUST match the circuit signature:
    /// 
    /// Circuit parameters → Instruction arguments mapping:
    /// - `encrypted_score: Enc<Shared, u32>` → 3 args: ArcisPubkey + PlaintextU128 + EncryptedU32
    /// - `threshold: Enc<Mxe, u8>` → 1 arg: EncryptedU8
    /// - `passing_requirement: Enc<Mxe, u32>` → 1 arg: EncryptedU32
    /// - `student: Shared` → 2 args: ArcisPubkey + PlaintextU128
    /// - `access_controller: Shared` → 2 args: ArcisPubkey + PlaintextU128
    /// - `auditor: Shared` → 2 args: ArcisPubkey + PlaintextU128
    /// 
    /// # Flow
    /// 1. Student completes quiz and gets encrypted score
    /// 2. This instruction verifies the score meets the threshold
    /// 3. If verification passes, student gains access to special surveys
    /// 4. Results are re-encrypted for student, access controller, and auditor
    // Separate instruction for student quiz verification
    pub fn verify_quiz_student(
        ctx: Context<VerifyQuizStudent>,
        quiz: Pubkey,
        student_computation_offset: u64,     // For student verification result
        encrypted_score: [u8; 32], // This will be processed by MPC
        encrypted_threshold: [u8; 32], // Encrypted threshold
        encrypted_requirement: [u8; 32], // Encrypted additional requirement
        student_pub_key: [u8; 32], // Student's public key
        student_nonce: u128,       // Student's nonce
        access_controller_pub_key: [u8; 32], // Access controller's public key
        access_controller_nonce: u128,       // Access controller's nonce
        auditor_pub_key: [u8; 32], // Auditor's public key
        auditor_nonce: u128,       // Auditor's nonce
    ) -> Result<()> {
        // Set the sign PDA account bump
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

        // Computation 1: Verification result for student
        // Circuit signature: encrypted_score: Enc<Shared, u32>, threshold: Enc<Mxe, u8>, passing_requirement: Enc<Mxe, u32>, student: Shared, access_controller: Shared, auditor: Shared
        let student_args = vec![
            // encrypted_score: Enc<Shared, u32> - 3 args
            Argument::ArcisPubkey(student_pub_key),           // Encryption key for encrypted_score
            Argument::PlaintextU128(student_nonce),           // Nonce for encrypted_score
            Argument::EncryptedU32(encrypted_score),          // Encrypted score
            // threshold: Enc<Mxe, u8> - 1 arg
            Argument::EncryptedU8(encrypted_threshold),       // Encrypted threshold
            // passing_requirement: Enc<Mxe, u32> - 1 arg
            Argument::EncryptedU32(encrypted_requirement),    // Encrypted passing requirement
            // student: Shared - 2 args
            Argument::ArcisPubkey(student_pub_key),           // student public key
            Argument::PlaintextU128(student_nonce),           // student nonce
            // access_controller: Shared - 2 args
            Argument::ArcisPubkey(access_controller_pub_key), // access_controller public key
            Argument::PlaintextU128(access_controller_nonce), // access_controller nonce
            // auditor: Shared - 2 args
            Argument::ArcisPubkey(auditor_pub_key),           // auditor public key
            Argument::PlaintextU128(auditor_nonce),           // auditor nonce
        ];

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
        
        // ✅ FIXED: Store completion proof with proper data handling
        let completion_proof = &mut ctx.accounts.completion_proof;
        
        // Check if this is a new completion proof or updating an existing one
        if completion_proof.verified {
            // If already verified, check if it's expired
            let clock = Clock::get()?;
            if completion_proof.expires_at > 0 && clock.unix_timestamp > completion_proof.expires_at {
                // Expired - allow re-verification
                completion_proof.verified = false;
                completion_proof.verified_at = 0;
            } else {
                // Still valid - prevent re-verification
                return Err(ErrorCode::AlreadyVerified.into());
            }
        }
        
        // Initialize or update completion proof data
        completion_proof.quiz = quiz;
        completion_proof.user = ctx.accounts.payer.key();
        completion_proof.encrypted_score = encrypted_score;  // Store as bytes
        completion_proof.threshold = 0;  // Threshold is now encrypted, not stored as plaintext
        completion_proof.verified = false; // Will be set to true by MPC callback
        completion_proof.verified_at = 0; // Will be set by MPC callback
        completion_proof.mpc_computation_id = student_computation_offset; // Use first computation ID
        
        // Set expiration timestamp based on quiz configuration
        let expiration_timestamp = match &ctx.accounts.quiz.survey_type {
            SurveyType::Quiz { completion_proof_expiration_value, completion_proof_expiration_unit, .. } => {
                calculate_expiration_timestamp(
                    clock.unix_timestamp,
                    *completion_proof_expiration_value,
                    &completion_proof_expiration_unit
                )?
            },
            _ => clock.unix_timestamp + (7 * 24 * 60 * 60), // Default 7 days fallback
        };
        
        completion_proof.expires_at = expiration_timestamp;
        
        // ✅ FIXED: Don't emit event here - wait for MPC callback to complete verification
        // The QuizCompletionVerified event will be emitted by the callback with the actual result
        
        Ok(())
    }



    // Submit special survey response with access control
    pub fn submit_special_survey_response(
        ctx: Context<SubmitSpecialSurveyResponse>,
        computation_offset: u64,
        _recipient_pubkey: [u8; 32],
        ciphertext_answer1: [u8; 32],
        ciphertext_answer2: [u8; 32],
        ciphertext_question_type1: [u8; 32],
        ciphertext_question_type2: [u8; 32],
        pub_key: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        // Set the sign PDA account bump
        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        
        // Access control: Verify user has valid quiz completion proof
        require!(ctx.accounts.completion_proof.verified, ErrorCode::Unauthorized);
        require!(ctx.accounts.completion_proof.user == ctx.accounts.payer.key(), ErrorCode::Unauthorized);
        require!(ctx.accounts.completion_proof.quiz == ctx.accounts.quiz.key(), ErrorCode::Unauthorized);
        
        // Check if completion proof has expired
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp <= ctx.accounts.completion_proof.expires_at,
            ErrorCode::ProofExpired
        );
        
        // Verify special survey is active
        require!(ctx.accounts.special_survey.is_active, ErrorCode::SurveyInactive);
        
        // Check if special survey has reached max responses
        require!(
            ctx.accounts.special_survey.current_responses < ctx.accounts.special_survey.max_responses,
            ErrorCode::SurveyFull
        );
        
        // Queue computation for special survey analytics
        // Following Arcium v0.3.0 pattern: Enc<Shared, T> requires pubkey + nonce before ciphertexts
        let args = vec![
            Argument::ArcisPubkey(pub_key),
            Argument::PlaintextU128(nonce),
            Argument::EncryptedU32(ciphertext_answer1),
            Argument::EncryptedU32(ciphertext_answer2),
            Argument::EncryptedU32(ciphertext_question_type1),
            Argument::EncryptedU32(ciphertext_question_type2),
        ];

        // Set the sign PDA account bump
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![SurveyAnalyticsCallback::callback_ix(&[])],
        )?;

        // Increment response counter
        ctx.accounts.special_survey.current_responses += 1;

        emit!(SpecialSurveyResponseSubmitted {
            special_survey: ctx.accounts.special_survey.key(),
            quiz: ctx.accounts.quiz.key(),
        });

        Ok(())
    }

    // Delete survey/quiz
    pub fn delete_survey(ctx: Context<DeleteSurvey>) -> Result<()> {
        let survey = &mut ctx.accounts.survey;
        let clock = Clock::get()?;
        
        require!(
            survey.creator == ctx.accounts.creator.key(),
            ErrorCode::Unauthorized
        );

        survey.is_active = false;

        emit!(SurveyDeleted {
            survey: survey.key(),
            creator: survey.creator,
            deleted_at: clock.unix_timestamp,
        });

        Ok(())
    }

    // ✅ REVERTED: Use standard Arcium v0.3.0 callback pattern
    // Note: Callbacks will be handled via standard Anchor event patterns

    // ✅ REVERTED: Use standard Arcium v0.3.0 callback pattern

    // ✅ FIXED: Implement callbacks using standard Arcium v0.3.0 patterns
    #[arcium_callback(encrypted_ix = "survey_analytics")]
    pub fn survey_analytics_callback(
        ctx: Context<SurveyAnalyticsCallback>,
        output: ComputationOutputs<SurveyAnalyticsOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(data) => data,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        // ✅ FIX: Store analytics data in the analytics storage account
        let analytics_storage = &mut ctx.accounts.analytics_storage;
        let clock = Clock::get()?;
        
        // Store the encrypted analytics data
        // The MPC output structure has field_0, field_1, field_2 - we need to access the correct fields
        analytics_storage.survey = ctx.accounts.survey.key();
        analytics_storage.creator_analytics = result.field_0.field_0.ciphertexts[0];  // Extract ciphertext from first field
        analytics_storage.public_summary = result.field_0.field_1.ciphertexts[0];     // Extract ciphertext from second field
        analytics_storage.respondent_feedback = result.field_0.field_2.ciphertexts[0]; // Extract ciphertext from third field
        analytics_storage.nonce = 0; // TODO: Extract nonce from MPC output if available
        analytics_storage.created_at = clock.unix_timestamp;
        analytics_storage.updated_at = clock.unix_timestamp;

        // Emit event for survey analytics completion
        emit!(SurveyAnalyticsComplete {
            analytics_data_ct: result.field_0,    // Analytics data for stakeholders
        });

        Ok(())
    }

    #[arcium_callback(encrypted_ix = "quiz_evaluation")]
    pub fn quiz_evaluation_callback(
        ctx: Context<QuizEvaluationCallback>,
        output: ComputationOutputs<QuizEvaluationOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(data) => data,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        // Emit event for quiz evaluation completion
        emit!(QuizEvaluationComplete {
            evaluation_data_ct: result.field_0,        // Quiz evaluation data
        });

        Ok(())
    }

    #[arcium_callback(encrypted_ix = "analytics_computation")]
    pub fn analytics_computation_callback(
        ctx: Context<AnalyticsComputationCallback>,
        output: ComputationOutputs<AnalyticsComputationOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(data) => data,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        // Emit event for analytics computation completion
        emit!(AnalyticsComputed {
            computation_data_ct: result.field_0,     // Analytics computation data
        });

        Ok(())
    }

    #[arcium_callback(encrypted_ix = "quiz_threshold_check")]
    pub fn quiz_threshold_check_callback(
        ctx: Context<QuizThresholdCheckCallback>,
        output: ComputationOutputs<QuizThresholdCheckOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(data) => data,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let clock = Clock::get()?;
        
        // Update completion proof account with MPC result
        let completion_proof = &mut ctx.accounts.completion_proof;
        completion_proof.verified = true; // MPC has verified the threshold check
        completion_proof.verified_at = clock.unix_timestamp;
        // Extract the actual encrypted verification result from MPC output
        // result.field_0 = ThresholdVerification (for student)
        // result.field_1 = AccessControl (for access controller)
        // result.field_2 = AuditRecord (for auditor)
        completion_proof.encrypted_verification_result = result.field_0.field_0.ciphertexts[0];
        
        // TODO: Update quiz aggregation account here
        // This would require adding the quiz_aggregation account to the callback struct
        // and implementing the aggregation logic
        
        // Emit event for quiz completion verification
        emit!(QuizCompletionVerified {
            verified_at: clock.unix_timestamp,
            verification_data_ct: result.field_0,   // Verification data
        });

        Ok(())
    }



    // ✅ DASHBOARD: Create user account for personalized dashboard
    pub fn create_user_account(
        ctx: Context<CreateUserAccount>,
        username: String,
        email: String,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let clock = Clock::get()?;
        
        // Input validation
        require!(username.len() <= 50, ErrorCode::UsernameTooLong);
        require!(email.len() <= 100, ErrorCode::EmailTooLong);
        require!(username.len() >= 3, ErrorCode::UsernameTooShort);
        
        // Security validations
        validate_no_xss(&username)?;
        validate_no_sql_injection(&username)?;
        validate_no_xss(&email)?;
        validate_no_sql_injection(&email)?;
        
        user_account.creator = ctx.accounts.creator.key();
        user_account.username = username;
        user_account.email = email;
        user_account.created_at = clock.unix_timestamp;
        user_account.is_active = true;
        user_account.last_login = clock.unix_timestamp;
        
        emit!(UserAccountCreated {
            user: ctx.accounts.creator.key(),
            username: user_account.username.clone(),
            created_at: clock.unix_timestamp,
        });
        
        Ok(())
    }

    // ✅ DASHBOARD: Get user's personalized dashboard data
    pub fn get_user_dashboard(
        ctx: Context<GetUserDashboard>,
    ) -> Result<UserDashboard> {
        let user = ctx.accounts.user.key();
        let clock = Clock::get()?;
        
        // Update last login
        let user_account = &mut ctx.accounts.user_account;
        user_account.last_login = clock.unix_timestamp;
        
        // Build dashboard data (this would typically query from events/logs)
        let dashboard = UserDashboard {
            surveys_created: vec![], // Would be populated from survey creation events
            responses_submitted: vec![], // Would be populated from response events
            quiz_results: vec![], // Would be populated from quiz completion events
            special_survey_access: vec![], // Would be populated from special survey events
        };
        
        emit!(UserDashboardAccessed {
            user,
            accessed_at: clock.unix_timestamp,
        });
        
        Ok(dashboard)
    }


}


// Account structures for our survey/quiz DApp

#[derive(Accounts)]
#[instruction(timestamp: i64)]
pub struct CreateSurveyWithTimestamp<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Survey::INIT_SPACE + 2000, // ✅ FIXED: Extra space for longer strings
        seeds = [b"survey", creator.key().as_ref(), &timestamp.to_le_bytes()],
        bump
    )]
    pub survey: Account<'info, Survey>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddQuestions<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    pub creator: Signer<'info>,
}

#[queue_computation_accounts("survey_analytics", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct SubmitSurveyResponse<'info> {
    // Removed response account - following Arcium docs: no individual response storage
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SURVEY_ANALYTICS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for analytics computation
#[queue_computation_accounts("survey_analytics", payer)]
#[derive(Accounts)]
#[instruction(analytics_computation_offset: u64)]
pub struct SubmitSurveyAnalytics<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(analytics_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SURVEY_ANALYTICS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    
    // ✅ FIX: Add analytics storage account for callback
    #[account(
        init_if_needed,
        payer = payer,
        space = SurveyAnalyticsStorage::INIT_SPACE,
        seeds = [b"analytics", survey.key().as_ref()],
        bump
    )]
    pub analytics_storage: Account<'info, SurveyAnalyticsStorage>,
    
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for feedback computation
#[queue_computation_accounts("survey_analytics", payer)]
#[derive(Accounts)]
#[instruction(feedback_computation_offset: u64)]
pub struct SubmitSurveyFeedback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(feedback_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SURVEY_ANALYTICS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[queue_computation_accounts("quiz_evaluation", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct SubmitQuizResponse<'info> {
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    // Removed response account - following Arcium docs: no individual response storage
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_EVALUATION)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for student quiz computation
#[queue_computation_accounts("quiz_evaluation", payer)]
#[derive(Accounts)]
#[instruction(student_computation_offset: u64)]
pub struct SubmitQuizStudent<'info> {
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(student_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_EVALUATION)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for instructor quiz computation
#[queue_computation_accounts("quiz_evaluation", payer)]
#[derive(Accounts)]
#[instruction(instructor_computation_offset: u64)]
pub struct SubmitQuizInstructor<'info> {
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(instructor_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_EVALUATION)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for creator quiz computation
#[queue_computation_accounts("quiz_evaluation", payer)]
#[derive(Accounts)]
#[instruction(creator_computation_offset: u64)]
pub struct SubmitQuizCreator<'info> {
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(creator_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_EVALUATION)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct CreateSpecialSurvey<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(
        init,
        payer = creator,
        space = 8 + Survey::INIT_SPACE,
        seeds = [b"special_survey", creator.key().as_ref(), quiz.key().as_ref()],
        bump
    )]
    pub special_survey: Account<'info, Survey>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("quiz_threshold_check", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct VerifyQuizCompletion<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + QuizCompletionProof::INIT_SPACE,
        seeds = [b"quiz_completion", payer.key().as_ref(), quiz.key().as_ref()],
        bump
    )]
    pub completion_proof: Account<'info, QuizCompletionProof>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_THRESHOLD)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for student quiz verification
#[queue_computation_accounts("quiz_threshold_check", payer)]
#[derive(Accounts)]
#[instruction(student_computation_offset: u64)]
pub struct VerifyQuizStudent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + QuizCompletionProof::INIT_SPACE,
        seeds = [b"quiz_completion", payer.key().as_ref(), quiz.key().as_ref()],
        bump
    )]
    pub completion_proof: Account<'info, QuizCompletionProof>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(student_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_THRESHOLD)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for access controller quiz verification
#[queue_computation_accounts("quiz_threshold_check", payer)]
#[derive(Accounts)]
#[instruction(access_controller_computation_offset: u64)]
pub struct VerifyQuizAccessController<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + QuizCompletionProof::INIT_SPACE,
        seeds = [b"quiz_completion", payer.key().as_ref(), quiz.key().as_ref()],
        bump
    )]
    pub completion_proof: Account<'info, QuizCompletionProof>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(access_controller_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_THRESHOLD)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

// Separate instruction for auditor quiz verification
#[queue_computation_accounts("quiz_threshold_check", payer)]
#[derive(Accounts)]
#[instruction(auditor_computation_offset: u64)]
pub struct VerifyQuizAuditor<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + QuizCompletionProof::INIT_SPACE,
        seeds = [b"quiz_completion", payer.key().as_ref(), quiz.key().as_ref()],
        bump
    )]
    pub completion_proof: Account<'info, QuizCompletionProof>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(auditor_computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_QUIZ_THRESHOLD)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[queue_computation_accounts("survey_analytics", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct SubmitSpecialSurveyResponse<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub special_survey: Account<'info, Survey>,
    #[account(mut)]
    pub quiz: Account<'info, Survey>,
    #[account(mut)]
    pub completion_proof: Account<'info, QuizCompletionProof>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 9,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SURVEY_ANALYTICS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct DeleteSurvey<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    pub creator: Signer<'info>,
}

// ✅ FIXED: Account structure for initializing Sign PDA Account
#[derive(Accounts)]
pub struct InitSignPda<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    pub system_program: Program<'info, System>,
}

// ✅ ADDED: Account structures for new multi-recipient computations








// ✅ REVERTED: Use proper Arcium macro-generated account structures
// These will be automatically generated by the  macro

// ✅ REVERTED: Using proper Arcium macro-based computation definition initialization

// Data structures
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum SurveyType {
    Basic,
    Quiz {
        time_per_question: Option<u64>,
        total_time_limit: Option<u64>,
        passing_threshold: u8,
        special_survey_enabled: bool,
        completion_proof_expiration_value: u32,  // Expiration value
        completion_proof_expiration_unit: ExpirationUnit,  // Expiration unit (minutes, hours, days)
    },
    Special,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ExpirationUnit {
    Minutes,
    Hours,
    Days,
}

// ✅ SECURE: MXE Configuration enums for Arcium compatibility
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum EncryptionScheme {
    Rescue,  // Arcium's preferred encryption scheme
    AES,      // Alternative encryption scheme
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum DataProvisioningMethod {
    Direct,   // Direct data provisioning for real-time computations
    Batch,    // Batch data provisioning for efficient processing
}

// ✅ FIXED: Add MPC protocol enum for Arcium compatibility
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum MPCProtocol {
    Cerberus,    // Arcium's Cerberus MPC protocol
    Manticore,   // Arcium's Manticore MPC protocol
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum QuestionType {
    MultipleChoice { options: Vec<String> },
    TrueFalse,
    TextInput,
    Rating { min: u8, max: u8 },
    TimeLimit { duration: u64 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuestionData {
    pub id: u32,
    pub question_text: String,
    pub question_type: QuestionType,
    pub required: bool,
    pub points: u8,
}

// Account definitions
#[account]
pub struct Survey {
    pub creator: Pubkey,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub survey_type: SurveyType,
    pub questions: Vec<QuestionData>,  // ✅ FIXED: Questions as plaintext (public for browsing)
    pub max_responses: u32,
    pub current_responses: u32,  // Track current number of responses
    pub is_active: bool,
    pub is_publicly_browsable: bool,  // Hide special surveys from public browsing
    pub modification_count: u8,  // Track modifications (max 1)
    pub created_at: i64,
    pub mpc_computation_id: u64,  // ✅ FIXED: Reference to MPC computation
}

#[account]
pub struct QuizCompletionProof {
    pub quiz: Pubkey,
    pub user: Pubkey,  // ✅ FIXED: Direct user reference (simpler and more secure)
    pub encrypted_score: [u8; 32],  // ✅ FIXED: Encrypted score as bytes
    pub threshold: u8,  // ✅ FIXED: Threshold as plaintext (public parameter)
    pub verified: bool,
    pub verified_at: i64,
    pub expires_at: i64,  // Expiration timestamp for access control
    pub mpc_computation_id: u64,  // ✅ FIXED: Reference to MPC computation
    pub encrypted_verification_result: [u8; 32],  // ✅ FIXED: Store MPC verification result
}

// ✅ DASHBOARD: User account for personalized dashboard
#[account]
pub struct UserAccount {
    pub creator: Pubkey,
    pub username: String,
    pub email: String,
    pub created_at: i64,
    pub is_active: bool,
    pub last_login: i64,
}

// ✅ CALLBACK ACCOUNTS: Survey analytics result storage
#[account]
#[derive(InitSpace)]
pub struct SurveyAnalyticsStorage {
    pub survey: Pubkey,
    pub creator_analytics: [u8; 32],    // Encrypted full analytics for survey creator
    pub public_summary: [u8; 32],       // Encrypted limited summary for public viewers
    pub respondent_feedback: [u8; 32],  // Encrypted personal feedback for respondent
    pub nonce: u128,                    // Nonce for decryption
    pub created_at: i64,
    pub updated_at: i64,
}

// ✅ CALLBACK ACCOUNTS: Quiz evaluation result storage
#[account]
#[derive(InitSpace)]
pub struct QuizEvaluationStorage {
    pub quiz: Pubkey,
    pub student_result: [u8; 32],        // Encrypted quiz result for student
    pub instructor_analytics: [u8; 32],  // Encrypted analytics for instructor
    pub creator_feedback: [u8; 32],      // Encrypted feedback for quiz creator
    pub nonce: u128,                     // Nonce for decryption
    pub created_at: i64,
    pub updated_at: i64,
}

// ✅ CALLBACK ACCOUNTS: Analytics computation result storage
#[account]
#[derive(InitSpace)]
pub struct AnalyticsComputationStorage {
    pub computation_id: u64,
    pub analyst_analytics: [u8; 32],      // Encrypted comprehensive analytics for data analyst
    pub business_metrics: [u8; 32],       // Encrypted business metrics for stakeholders
    pub research_insights: [u8; 32],      // Encrypted research insights for researchers
    pub nonce: u128,                      // Nonce for decryption
    pub created_at: i64,
    pub updated_at: i64,
}

// ✅ CALLBACK ACCOUNTS: Quiz threshold verification result storage
#[account]
#[derive(InitSpace)]
pub struct QuizThresholdStorage {
    pub quiz: Pubkey,
    pub user: Pubkey,
    pub threshold_verification: [u8; 32],  // Encrypted threshold verification result
    pub access_control: [u8; 32],          // Encrypted access control result
    pub audit_record: [u8; 32],            // Encrypted audit record
    pub nonce: u128,                       // Nonce for decryption
    pub created_at: i64,
    pub updated_at: i64,
}

// ✅ MULTI-USER: Quiz-level aggregation for multiple completion proofs
#[account]
#[derive(InitSpace)]
pub struct QuizAggregation {
    pub quiz: Pubkey,
    pub total_attempts: u32,               // Total number of quiz attempts
    pub successful_completions: u32,       // Number of successful completions
    pub average_score: u8,                 // Average score (0-100)
    pub last_updated: i64,                 // Last update timestamp
    pub created_at: i64,                   // Creation timestamp
}

// ✅ FIXED: Application preferences for survey/quiz computations
// Note: This is application-level metadata, not Arcium MXE configuration
#[account]
pub struct SurveyComputationPreferences {
    pub admin: Pubkey,
    pub preferred_encryption_scheme: EncryptionScheme,
    pub preferred_data_provisioning: DataProvisioningMethod,
    pub preferred_mpc_protocol: MPCProtocol,
    pub configured_at: i64,
    pub is_active: bool,
}

impl QuizCompletionProof {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 1 + 1 + 8 + 8 + 8 + 32; // quiz + user + encrypted_score + threshold + verified + verified_at + expires_at + mpc_computation_id + encrypted_verification_result
}

impl UserAccount {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // creator
        + STRING_LENGTH_PREFIX + 50 // username (max length)
        + STRING_LENGTH_PREFIX + 100 // email (max length)
        + 8 // created_at
        + 1 // is_active
        + 8; // last_login
}

impl SurveyComputationPreferences {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // admin
        + 1 // preferred_encryption_scheme (enum)
        + 1 // preferred_data_provisioning (enum)
        + 1 // preferred_mpc_protocol (enum)
        + 8 // configured_at
        + 1; // is_active
}

impl QuizAggregation {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // quiz
        + 4 // total_attempts
        + 4 // successful_completions
        + 1 // average_score
        + 8 // last_updated
        + 8; // created_at
}

// ✅ CALLBACK ACCOUNTS: INIT_SPACE implementations for result storage accounts
impl SurveyAnalyticsStorage {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // survey
        + 32 // creator_analytics
        + 32 // public_summary
        + 32 // respondent_feedback
        + 16 // nonce
        + 8 // created_at
        + 8; // updated_at
}

impl QuizEvaluationStorage {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // quiz
        + 32 // student_result
        + 32 // instructor_analytics
        + 32 // creator_feedback
        + 16 // nonce
        + 8 // created_at
        + 8; // updated_at
}

impl AnalyticsComputationStorage {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + 8 // computation_id
        + 32 // analyst_analytics
        + 32 // business_metrics
        + 32 // research_insights
        + 16 // nonce
        + 8 // created_at
        + 8; // updated_at
}

impl QuizThresholdStorage {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // quiz
        + PUBKEY_LENGTH // user
        + 32 // threshold_verification
        + 32 // access_control
        + 32 // audit_record
        + 16 // nonce
        + 8 // created_at
        + 8; // updated_at
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Question {
    pub id: u32,
    pub question_text: String,
    pub question_type: QuestionType,
    pub required: bool,
    pub points: u8,
}

// Removed Response struct - following Arcium docs: no individual response storage

// Event structures
#[event]
pub struct SurveyCreated {
    pub survey: Pubkey,
    // Removed creator, slug, title to prevent privacy leaks
    pub survey_type: String, // "basic", "quiz", or "special"
    pub created_at: i64,
}

#[event]
pub struct QuestionsAdded {
    pub survey: Pubkey,
    pub question_count: u32,
}

#[event]
pub struct ResponseSubmitted {
    pub survey: Pubkey,
}

#[event]
pub struct QuizResponseSubmitted {
    pub quiz: Pubkey,
}

#[event]
pub struct SpecialSurveyCreated {
    pub quiz: Pubkey,
    pub special_survey: Pubkey,
}

#[event]
pub struct QuizCompletionVerified {
    // Removed quiz, user and threshold to prevent privacy leaks
    pub verified_at: i64,
    pub verification_data_ct: QuizThresholdCheckOutputStruct0,      // Verification data
}

#[event]
pub struct QuizCompletionFailed {
    pub quiz: Pubkey,
    pub failed_at: i64,
}

// ✅ DASHBOARD: User account creation event
#[event]
pub struct UserAccountCreated {
    pub user: Pubkey,
    pub username: String,
    pub created_at: i64,
}

// ✅ DASHBOARD: Dashboard access event
#[event]
pub struct UserDashboardAccessed {
    pub user: Pubkey,
    pub accessed_at: i64,
}

// ✅ DASHBOARD: Dashboard data structures
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserDashboard {
    pub surveys_created: Vec<UserSurvey>,
    pub responses_submitted: Vec<UserResponse>,
    pub quiz_results: Vec<UserQuizResult>,
    pub special_survey_access: Vec<UserSpecialAccess>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserSurvey {
    pub survey: Pubkey,
    pub title: String,
    pub description: String,
    pub response_count: u32,
    pub is_active: bool,
    pub created_at: i64,
    pub analytics_available: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserResponse {
    pub survey: Pubkey,
    pub survey_title: String,
    pub submitted_at: i64,
    pub response_summary: String,  // Encrypted summary
    pub results_received: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserQuizResult {
    pub quiz: Pubkey,
    pub quiz_title: String,
    pub score: Option<u32>,  // If user wants to see it
    pub passed: bool,
    pub special_survey_access: bool,
    pub taken_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserSpecialAccess {
    pub quiz: Pubkey,
    pub special_survey: Pubkey,
    pub access_granted: bool,
    pub access_expires: i64,
    pub accessed_at: Option<i64>,
}

#[event]
pub struct SpecialSurveyResponseSubmitted {
    pub special_survey: Pubkey,
    pub quiz: Pubkey,
}

#[event]
pub struct SpecialSurveyAnalyticsComplete {
    pub special_survey: Pubkey,
    pub quiz: Pubkey,
    pub analytics1_ct: SharedEncryptedStruct<1>,  // Full struct
    pub analytics2_ct: SharedEncryptedStruct<1>,  // Full struct
}


#[event]
pub struct SurveyAnalyticsComplete {
    pub analytics_data_ct: SurveyAnalyticsOutputStruct0,    // Analytics data for stakeholders
}

#[event]
pub struct QuizEvaluationComplete {
    pub evaluation_data_ct: QuizEvaluationOutputStruct0,        // Quiz evaluation data
}

#[event]
pub struct QuizPassed {
    pub quiz: Pubkey,
    // Removed respondent and score to prevent privacy leaks
    pub special_survey_available: bool,
    pub passed_at: i64,
}

#[event]
pub struct AnalyticsComputed {
    pub computation_data_ct: AnalyticsComputationOutputStruct0,      // Analytics computation data
}


// Admin control events for audit logging
#[event]
pub struct SurveySuspended {
    pub survey: Pubkey,
    pub creator: Pubkey,
    pub suspended_at: i64,
}

#[event]
pub struct SurveyUnsuspended {
    pub survey: Pubkey,
    pub creator: Pubkey,
    pub unsuspended_at: i64,
}

#[event]
pub struct SurveyDeleted {
    pub survey: Pubkey,
    pub creator: Pubkey,
    pub deleted_at: i64,
}

#[event]
pub struct SurveyModified {
    pub survey: Pubkey,
    pub creator: Pubkey,
    pub modified_at: i64,
    pub modification_count: u8,
}

// ✅ FIXED: Application preferences event
#[event]
pub struct SurveyComputationPreferencesSet {
    pub preferred_encryption_scheme: String,
    pub preferred_data_provisioning: String,
    pub preferred_mpc_protocol: String,
    pub configured_at: i64,
}

// ✅ ADD: Missing event structures for comprehensive error tracking
#[event]
pub struct ComputationAborted {
    pub computation_type: String,
    pub aborted_at: i64,
}

#[event]
pub struct ComputationFailed {
    pub computation_type: String,
    pub failed_at: i64,
}

#[event]
pub struct ComputationTimeout {
    pub computation_type: String,
    pub timed_out_at: i64,
}

#[event]
pub struct ComputationResultInvalid {
    pub computation_type: String,
    pub invalid_at: i64,
    pub reason: String,
}

// Constants
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;

// Input validation constants
const MAX_TITLE_LENGTH: usize = 200;
const MAX_DESCRIPTION_LENGTH: usize = 1000;
const MAX_SLUG_LENGTH: usize = 50;
const MIN_SLUG_LENGTH: usize = 3;
const MAX_QUESTIONS: usize = 50;
const MAX_QUESTION_TEXT_LENGTH: usize = 500;
const MAX_OPTIONS_PER_QUESTION: usize = 10;
const MAX_OPTION_TEXT_LENGTH: usize = 100;
const MIN_MAX_RESPONSES: u32 = 1;
const MAX_MAX_RESPONSES: u32 = 1000000;

// Reserved words that cannot be used as slugs
const RESERVED_SLUGS: &[&str] = &[
    "admin", "api", "login", "logout", "register", "signup", "signin", "signout",
    "dashboard", "profile", "settings", "account", "user", "users", "survey", "surveys",
    "quiz", "quizzes", "test", "demo", "example", "sample", "root", "home", "index",
    "www", "mail", "email", "ftp", "news", "blog", "forum", "help", "support",
    "about", "contact", "privacy", "terms", "legal", "faq", "search", "search",
    "create", "edit", "delete", "update", "new", "old", "back", "next", "prev",
    "submit", "cancel", "save", "load", "upload", "download", "file", "files",
    "image", "images", "img", "css", "js", "javascript", "html", "xml", "json",
    "rss", "atom", "feed", "sitemap", "robots", "favicon", "apple-touch-icon",
    "system", "config", "configuration", "setup", "install", "uninstall",
    "upgrade", "update", "version", "changelog", "release", "beta", "alpha",
    "dev", "development", "staging", "production", "prod", "test", "testing",
    "debug", "error", "errors", "exception", "exceptions", "log", "logs",
    "monitor", "monitoring", "stats", "statistics", "analytics", "metrics",
    "report", "reports", "export", "import", "backup", "restore", "migrate",
    "migration", "schema", "database", "db", "sql", "query", "queries",
    "cache", "session", "sessions", "cookie", "cookies", "token", "tokens",
    "auth", "authentication", "authorization", "permission", "permissions",
    "role", "roles", "group", "groups", "team", "teams", "organization",
    "org", "company", "business", "enterprise", "corporate", "public",
    "private", "internal", "external", "guest", "visitor", "member", "members",
    "staff", "employee", "employees", "manager", "managers", "admin", "admins",
    "superuser", "root", "owner", "owners", "creator", "creators", "author",
    "authors", "editor", "editors", "moderator", "moderators", "reviewer",
    "reviewers", "approver", "approvers", "validator", "validators", "auditor",
    "auditors", "inspector", "inspectors", "supervisor", "supervisors",
    "director", "directors", "executive", "executives", "ceo", "cto", "cfo",
    "cmo", "coo", "vp", "vice", "president", "presidents", "chairman",
    "chairwoman", "chair", "board", "committee", "council", "senate",
    "parliament", "government", "official", "officials", "representative",
    "representatives", "delegate", "delegates", "ambassador", "ambassadors",
    "minister", "ministers", "secretary", "secretaries", "commissioner",
    "commissioners", "governor", "governors", "mayor", "mayors", "sheriff",
    "sheriffs", "judge", "judges", "justice", "justices", "lawyer", "lawyers",
    "attorney", "attorneys", "counsel", "counsels", "advocate", "advocates",
    "defender", "defenders", "prosecutor", "prosecutors", "plaintiff",
    "plaintiffs", "defendant", "defendants", "witness", "witnesses",
    "jury", "juries", "court", "courts", "trial", "trials", "case", "cases",
    "lawsuit", "lawsuits", "litigation", "arbitration", "mediation",
    "negotiation", "settlement", "settlements", "agreement", "agreements",
    "contract", "contracts", "treaty", "treaties", "pact", "pacts",
    "alliance", "alliances", "partnership", "partnerships", "collaboration",
    "collaborations", "cooperation", "cooperations", "coordination",
    "coordinations", "integration", "integrations", "merger", "mergers",
    "acquisition", "acquisitions", "takeover", "takeovers", "buyout",
    "buyouts", "investment", "investments", "funding", "fundings",
    "financing", "financings", "capital", "capitals", "equity", "equities",
    "debt", "debts", "loan", "loans", "credit", "credits", "mortgage",
    "mortgages", "lease", "leases", "rental", "rentals", "subscription",
    "subscriptions", "membership", "memberships", "premium", "premiums",
    "basic", "basics", "standard", "standards", "professional", "professionals",
    "enterprise", "enterprises", "custom", "customs", "personal", "personals",
    "individual", "individuals", "family", "families", "household", "households",
    "residential", "residentials", "commercial", "commercials", "industrial",
    "industrials", "retail", "retails", "wholesale", "wholesales", "b2b",
    "b2c", "b2g", "g2b", "g2c", "c2c", "c2b", "p2p", "peer", "peers",
    "network", "networks", "community", "communities", "society", "societies",
    "association", "associations", "federation", "federations", "union",
    "unions", "coalition", "coalitions", "alliance", "alliances", "partnership",
    "partnerships", "collaboration", "collaborations", "cooperation",
    "cooperations", "coordination", "coordinations", "integration",
    "integrations", "merger", "mergers", "acquisition", "acquisitions",
    "takeover", "takeovers", "buyout", "buyouts", "investment", "investments",
    "funding", "fundings", "financing", "financings", "capital", "capitals",
    "equity", "equities", "debt", "debts", "loan", "loans", "credit", "credits",
    "mortgage", "mortgages", "lease", "leases", "rental", "rentals",
    "subscription", "subscriptions", "membership", "memberships", "premium",
    "premiums", "basic", "basics", "standard", "standards", "professional",
    "professionals", "enterprise", "enterprises", "custom", "customs",
    "personal", "personals", "individual", "individuals", "family", "families",
    "household", "households", "residential", "residentials", "commercial",
    "commercials", "industrial", "industrials", "retail", "retails",
    "wholesale", "wholesales", "b2b", "b2c", "b2g", "g2b", "g2c", "c2c",
    "c2b", "p2p", "peer", "peers", "network", "networks", "community",
    "communities", "society", "societies", "association", "associations",
    "federation", "federations", "union", "unions", "coalition", "coalitions"
];

// Input validation helper functions
fn validate_string_length(s: &str, max_length: usize, _field_name: &str) -> Result<()> {
    require!(
        s.len() <= max_length,
        ErrorCode::StringTooLong
    );
    require!(
        !s.is_empty(),
        ErrorCode::StringEmpty
    );
    Ok(())
}

fn validate_max_responses(max_responses: u32) -> Result<()> {
    require!(
        max_responses >= MIN_MAX_RESPONSES && max_responses <= MAX_MAX_RESPONSES,
        ErrorCode::InvalidMaxResponses
    );
    Ok(())
}

fn validate_questions(questions: &Vec<QuestionData>) -> Result<()> {
    require!(
        questions.len() <= MAX_QUESTIONS,
        ErrorCode::TooManyQuestions
    );
    
    for question in questions {
        validate_string_length(&question.question_text, MAX_QUESTION_TEXT_LENGTH, "question_text")?;
        
        if let QuestionType::MultipleChoice { options } = &question.question_type {
            require!(
                options.len() <= MAX_OPTIONS_PER_QUESTION,
                ErrorCode::TooManyOptions
            );
            
            for option in options {
                validate_string_length(option, MAX_OPTION_TEXT_LENGTH, "option")?;
            }
        }
    }
    
    Ok(())
}

// Security validation functions
fn validate_no_xss(input: &str) -> Result<()> {
    // Check for common XSS patterns
    let dangerous_patterns = [
        "<script", "</script", "javascript:", "onload=", "onerror=", "onclick=",
        "onmouseover=", "onfocus=", "onblur=", "onchange=", "onsubmit=",
        "onreset=", "onselect=", "onkeydown=", "onkeyup=", "onkeypress=",
        "onmousedown=", "onmouseup=", "onmousemove=", "onmouseout=",
        "ondblclick=", "oncontextmenu=", "onresize=", "onscroll=",
        "onunload=", "onbeforeunload=", "onhashchange=", "onpopstate=",
        "onstorage=", "onmessage=", "onabort=", "onloadstart=", "onprogress=",
        "onloadend=", "ontimeout=", "onreadystatechange=", "onopen=", "onclose="
    ];
    
    let input_lower = input.to_lowercase();
    for pattern in &dangerous_patterns {
        if input_lower.contains(pattern) {
            return Err(ErrorCode::SuspiciousInput.into());
        }
    }
    
    Ok(())
}

fn validate_no_sql_injection(input: &str) -> Result<()> {
    // Check for common SQL injection patterns
    let dangerous_patterns = [
        "'", "\"", ";", "--", "/*", "*/", "xp_", "sp_", "exec", "execute",
        "union", "select", "insert", "update", "delete", "drop", "create",
        "alter", "grant", "revoke", "truncate", "declare", "cast", "convert"
    ];
    
    let input_lower = input.to_lowercase();
    for pattern in &dangerous_patterns {
        if input_lower.contains(pattern) {
            return Err(ErrorCode::SuspiciousInput.into());
        }
    }
    
    Ok(())
}

fn validate_slug(slug: &str) -> Result<()> {
    // Check length
    require!(
        slug.len() >= MIN_SLUG_LENGTH && slug.len() <= MAX_SLUG_LENGTH,
        ErrorCode::InvalidSlugLength
    );
    
    // Check if empty
    require!(!slug.is_empty(), ErrorCode::StringEmpty);
    
    // Check format - only alphanumeric and hyphens
    require!(
        slug.chars().all(|c| c.is_alphanumeric() || c == '-'),
        ErrorCode::InvalidSlugFormat
    );
    
    // Cannot start or end with hyphen
    require!(
        !slug.starts_with('-') && !slug.ends_with('-'),
        ErrorCode::InvalidSlugFormat
    );
    
    // Cannot have consecutive hyphens
    require!(
        !slug.contains("--"),
        ErrorCode::InvalidSlugFormat
    );
    
    // Check for reserved words (case-insensitive)
    let slug_lower = slug.to_lowercase();
    require!(
        !RESERVED_SLUGS.iter().any(|&reserved| reserved == slug_lower),
        ErrorCode::ReservedSlug
    );
    
    // Check for suspicious patterns
    require!(
        !slug_lower.contains("javascript") && 
        !slug_lower.contains("script") && 
        !slug_lower.contains("alert") &&
        !slug_lower.contains("onclick") &&
        !slug_lower.contains("onload") &&
        !slug_lower.contains("onerror") &&
        !slug_lower.contains("onmouseover") &&
        !slug_lower.contains("onfocus") &&
        !slug_lower.contains("onblur") &&
        !slug_lower.contains("onchange") &&
        !slug_lower.contains("onsubmit") &&
        !slug_lower.contains("onreset") &&
        !slug_lower.contains("onselect") &&
        !slug_lower.contains("onkeydown") &&
        !slug_lower.contains("onkeyup") &&
        !slug_lower.contains("onkeypress") &&
        !slug_lower.contains("onmousedown") &&
        !slug_lower.contains("onmouseup") &&
        !slug_lower.contains("onmousemove") &&
        !slug_lower.contains("onmouseout") &&
        !slug_lower.contains("ondblclick") &&
        !slug_lower.contains("oncontextmenu") &&
        !slug_lower.contains("onresize") &&
        !slug_lower.contains("onscroll") &&
        !slug_lower.contains("onunload") &&
        !slug_lower.contains("onbeforeunload") &&
        !slug_lower.contains("onhashchange") &&
        !slug_lower.contains("onpopstate") &&
        !slug_lower.contains("onstorage") &&
        !slug_lower.contains("onmessage") &&
        !slug_lower.contains("onerror") &&
        !slug_lower.contains("onabort") &&
        !slug_lower.contains("onloadstart") &&
        !slug_lower.contains("onprogress") &&
        !slug_lower.contains("onloadend") &&
        !slug_lower.contains("ontimeout") &&
        !slug_lower.contains("onreadystatechange") &&
        !slug_lower.contains("onopen") &&
        !slug_lower.contains("onclose") &&
        !slug_lower.contains("onmessage") &&
        !slug_lower.contains("onerror") &&
        !slug_lower.contains("onabort") &&
        !slug_lower.contains("onloadstart") &&
        !slug_lower.contains("onprogress") &&
        !slug_lower.contains("onloadend") &&
        !slug_lower.contains("ontimeout") &&
        !slug_lower.contains("onreadystatechange") &&
        !slug_lower.contains("onopen") &&
        !slug_lower.contains("onclose"),
        ErrorCode::SuspiciousSlug
    );
    
    // Check for numeric-only slugs (prevent enumeration)
    require!(
        !slug.chars().all(|c| c.is_numeric()),
        ErrorCode::NumericSlug
    );
    
    // Check for common attack patterns
    require!(
        !slug_lower.contains("..") && 
        !slug_lower.contains("//") && 
        !slug_lower.contains("\\\\") &&
        !slug_lower.contains("..") &&
        !slug_lower.contains("~") &&
        !slug_lower.contains("$") &&
        !slug_lower.contains("(") &&
        !slug_lower.contains(")") &&
        !slug_lower.contains("[") &&
        !slug_lower.contains("]") &&
        !slug_lower.contains("{") &&
        !slug_lower.contains("}") &&
        !slug_lower.contains("|") &&
        !slug_lower.contains("\\") &&
        !slug_lower.contains("^") &&
        !slug_lower.contains("`") &&
        !slug_lower.contains("'") &&
        !slug_lower.contains("\"") &&
        !slug_lower.contains(";") &&
        !slug_lower.contains(":") &&
        !slug_lower.contains(",") &&
        !slug_lower.contains(".") &&
        !slug_lower.contains("?") &&
        !slug_lower.contains("!") &&
        !slug_lower.contains("@") &&
        !slug_lower.contains("#") &&
        !slug_lower.contains("%") &&
        !slug_lower.contains("&") &&
        !slug_lower.contains("*") &&
        !slug_lower.contains("+") &&
        !slug_lower.contains("=") &&
        !slug_lower.contains("<") &&
        !slug_lower.contains(">") &&
        !slug_lower.contains(" ") &&
        !slug_lower.contains("\t") &&
        !slug_lower.contains("\n") &&
        !slug_lower.contains("\r"),
        ErrorCode::InvalidSlugFormat
    );
    
    Ok(())
}

impl Survey {
    pub const INIT_SPACE: usize = DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // creator
        + STRING_LENGTH_PREFIX + 40 // slug (max length)
        + STRING_LENGTH_PREFIX + 100 // title (max length)
        + STRING_LENGTH_PREFIX + 200 // description (max length)
        + 4 // survey_type
        + 4 + (10 * 200) // questions (Vec<QuestionData> with max 10 questions)
        + 4 // max_responses
        + 4 // current_responses
        + 1 // is_active
        + 1 // is_publicly_browsable
        + 1 // modification_count
        + 8 // created_at
        + 8; // mpc_computation_id
    
    // Dynamic space calculation for actual string lengths
    pub fn calculate_space(slug: &str, title: &str) -> usize {
        DISCRIMINATOR_LENGTH
        + PUBKEY_LENGTH // creator
        + STRING_LENGTH_PREFIX + slug.len() // dynamic slug size
        + STRING_LENGTH_PREFIX + title.len() // dynamic title size
        + STRING_LENGTH_PREFIX + 200 // description (max length) - FIXED: was missing
        + 4 // survey_type - FIXED: was missing
        + 4 + (10 * 200) // questions (Vec<QuestionData> with max 10 questions) - FIXED: was missing
        + 4 // max_responses
        + 4 // current_responses
        + 1 // is_active
        + 1 // is_publicly_browsable
        + 1 // modification_count
        + 8 // created_at
        + 8 // mpc_computation_id - FIXED: was missing
    }
}

// Removed Response::INIT_SPACE - no longer storing individual responses

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Survey is not active")]
    SurveyInactive,
    #[msg("Survey has reached maximum responses")]
    SurveyFull,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid survey type")]
    InvalidSurveyType,
    #[msg("Operation failed")]
    QuestionNotFound,
    #[msg("Invalid input")]
    InvalidAnswerFormat,
    #[msg("Operation failed")]
    AbortedComputation,
    #[msg("Operation failed")]
    ClusterNotSet,
    #[msg("Operation failed")]
    ScoreBelowThreshold,
    #[msg("Invalid input")]
    InvalidThreshold,
    #[msg("Operation failed")]
    InvalidScoreProof,
    #[msg("Operation failed")]
    ProofExpired,
    #[msg("Invalid input")]
    InvalidExpirationValue,
    #[msg("Operation failed")]
    StringTooLong,
    #[msg("Operation failed")]
    StringEmpty,
    #[msg("Operation failed")]
    InvalidMaxResponses,
    #[msg("Operation failed")]
    TooManyQuestions,
    #[msg("Operation failed")]
    TooManyOptions,
    #[msg("Invalid input")]
    InvalidSlugLength,
    #[msg("Invalid input")]
    InvalidSlugFormat,
    #[msg("Operation failed")]
    ReservedSlug,
    #[msg("Invalid input")]
    SuspiciousSlug,
    #[msg("Invalid input")]
    NumericSlug,
    #[msg("Operation failed")]
    AlreadyModified,
    #[msg("Operation failed")]
    SurveyAlreadyActive,
    #[msg("Quiz completion already verified")]
    AlreadyVerified,
    #[msg("Invalid input")]
    SuspiciousInput,
    SurveyTooLarge,
    UsernameTooLong,
    EmailTooLong,
    UsernameTooShort,
    #[msg("Integer overflow detected")]
    IntegerOverflow,
    #[msg("Unsupported encryption scheme")]
    UnsupportedEncryptionScheme,
    #[msg("Unsupported data provisioning method")]
    UnsupportedDataProvisioning,
    #[msg("Unsupported MPC protocol")]
    UnsupportedMPCProtocol,
    #[msg("Invalid MXE configuration")]
    InvalidMXEConfiguration,
    #[msg("Cluster not available")]
    ClusterNotAvailable,
    #[msg("MPC computation failed")]
    MPCComputationFailed,
    #[msg("Computation failed")]
    ComputationFailed,
    #[msg("Computation timed out")]
    ComputationTimeout,
}

// Account structures for admin controls
#[derive(Accounts)]
pub struct SuspendSurvey<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnsuspendSurvey<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub creator: Signer<'info>,
}


#[derive(Accounts)]
pub struct ModifySurvey<'info> {
    #[account(mut)]
    pub survey: Account<'info, Survey>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

// ✅ DASHBOARD: Account structures for user account creation
#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user_account", creator.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

// ✅ DASHBOARD: Account structures for dashboard access
#[derive(Accounts)]
pub struct GetUserDashboard<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
}

// ✅ FIXED: Account structure for application preferences
// Note: This is application-level metadata, not Arcium MXE configuration
#[derive(Accounts)]
pub struct SetSurveyComputationPreferences<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + SurveyComputationPreferences::INIT_SPACE,
        seeds = [b"survey_computation_preferences"],
        bump
    )]
    pub app_preferences: Account<'info, SurveyComputationPreferences>,
    pub system_program: Program<'info, System>,
}

// Admin control functions for creators
pub fn suspend_my_survey(ctx: Context<SuspendSurvey>) -> Result<()> {
    let survey = &mut ctx.accounts.survey;
    let clock = Clock::get()?;
    
    require!(
        survey.creator == ctx.accounts.creator.key(),
        ErrorCode::Unauthorized
    );
    require!(survey.is_active, ErrorCode::SurveyInactive);
    
    survey.is_active = false;
    
    emit!(SurveySuspended {
        survey: survey.key(),
        creator: survey.creator,
        suspended_at: clock.unix_timestamp,
    });
    
    Ok(())
}

pub fn unsuspend_my_survey(ctx: Context<UnsuspendSurvey>) -> Result<()> {
    let survey = &mut ctx.accounts.survey;
    let clock = Clock::get()?;
    
    require!(
        survey.creator == ctx.accounts.creator.key(),
        ErrorCode::Unauthorized
    );
    require!(!survey.is_active, ErrorCode::SurveyAlreadyActive);
    
    survey.is_active = true;
    
    emit!(SurveyUnsuspended {
        survey: survey.key(),
        creator: survey.creator,
        unsuspended_at: clock.unix_timestamp,
    });
    
    Ok(())
}

pub fn delete_my_survey(ctx: Context<DeleteSurvey>) -> Result<()> {
    let survey = &mut ctx.accounts.survey;
    let clock = Clock::get()?;
    
    require!(
        survey.creator == ctx.accounts.creator.key(),
        ErrorCode::Unauthorized
    );
    
    // Permanently delete survey
    // Note: In Solana, we can't actually delete accounts, but we can mark them as deleted
    survey.is_active = false;
    survey.title = "DELETED".to_string();
    survey.slug = "deleted".to_string();
    
    emit!(SurveyDeleted {
        survey: survey.key(),
        creator: survey.creator,
        deleted_at: clock.unix_timestamp,
    });
    
    Ok(())
}

pub fn modify_my_survey(
    ctx: Context<ModifySurvey>,
    new_title: String,
    new_description: String,
) -> Result<()> {
    let survey = &mut ctx.accounts.survey;
    let clock = Clock::get()?;
    
    require!(
        survey.creator == ctx.accounts.creator.key(),
        ErrorCode::Unauthorized
    );
    require!(
        survey.modification_count == 0,
        ErrorCode::AlreadyModified
    );
    
    // Input validation
    validate_string_length(&new_title, MAX_TITLE_LENGTH, "title")?;
    validate_string_length(&new_description, MAX_DESCRIPTION_LENGTH, "description")?;
    
    survey.title = new_title;
    survey.description = new_description;
    survey.modification_count = 1;
    
    emit!(SurveyModified {
        survey: survey.key(),
        creator: survey.creator,
        modified_at: clock.unix_timestamp,
        modification_count: survey.modification_count,
    });
    
    Ok(())
}

