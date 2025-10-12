# MRE: Sign PDA Account Signer Privilege Escalation in Arcium v0.3.0

## Environment Details
- **Program ID**: `Crky8qtmLUVESVNTnTUgLaVnUdgBvCQuFzsGmEFSKmZ2`
- **Network**: Devnet
- **Arcium SDK Version**: 0.3.0
- **Anchor Version**: 0.31.1
- **Solana Version**: 1.18.x

## Issue Description
Despite following Arcium v0.3.0 documentation exactly, the Sign PDA Account is causing "Cross-program invocation with unauthorized signer" errors during `queue_computation` CPI calls. The implementation matches the official migration guide but still fails.

## Error Message
```
Cross-program invocation with unauthorized signer or writable account
"2PVQxTpX3pNpeNrUQoTAMBehNmkNx9gvJTs68sqKVxSe's signer privilege escalated"
```

## Use Case
Survey response submission with encrypted analytics using the `submit_survey_response` instruction.

## Code Implementation

### Account Structure
```rust
#[queue_computation_accounts("survey_analytics", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct SubmitSurveyResponse<'info> {
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
```

### Instruction Implementation
```rust
pub fn submit_survey_response(
    ctx: Context<SubmitSurveyResponse>,
    analytics_computation_offset: u64,
    feedback_computation_offset: u64,
    // ... other parameters
) -> Result<()> {
    // Set bump for Sign PDA Account (required by v0.3.0)
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
    
    // Call queue_computation with proper v0.3.0 signature - this is where the error occurs
    queue_computation(
        ctx.accounts,
        analytics_computation_offset,
        creator_args,
        None,
        vec![SurveyAnalyticsCallback::callback_ix(&[])],
    )?;
    
    Ok(())
}
```

### Account Derivations
```rust
// Sign PDA Account derivation (matches v0.3.0 documentation)
pub const SIGN_PDA_SEED: &[u8] = b"sign_pda";

// All derivations use Arcium's built-in macros
// derive_sign_pda!() - Sign PDA Account
// derive_mxe_pda!() - MXE Account  
// derive_mempool_pda!() - Mempool Account
// derive_execpool_pda!() - Execution Pool Account
// derive_comp_pda!(offset) - Computation Account
// derive_comp_def_pda!(offset) - Computation Definition Account
// derive_cluster_pda!(mxe_account) - Cluster Account
```

### Cargo.toml Configuration
```toml
[dependencies]
anchor-lang = { version = "0.31.1", features = ["init-if-needed"] }
arcium-client = { default-features = false, version = "0.3.0" }
arcium-macros = "0.3.0"
arcium-anchor = "0.3.0"

[patch.crates-io]
proc-macro2 = { git = "https://github.com/arcium-hq/proc-macro2.git" }
```

## Steps to Reproduce
1. Deploy the program to devnet
2. Initialize computation definitions using `arcium deploy`
3. Create a survey with analytics enabled
4. Attempt to submit a survey response
5. Observe the signer privilege escalation error

## Expected vs Actual Behavior

### Expected (Based on Nico's Response)
- Sign PDA Account should be a signer (as confirmed by Nico from Arcium team)
- CPI call to `queue_computation` should succeed with proper Sign PDA setup
- Survey response should be processed with encrypted analytics

### Actual
- Implementation follows v0.3.0 documentation exactly
- Sign PDA Account is properly configured as signer
- CPI call still fails with "unauthorized signer" error
- Survey response submission is blocked despite correct implementation

## Key Questions 

1. **Implementation Question**: Our code matches the v0.3.0 migration guide exactly - what could be causing the "unauthorized signer" error?

2. **Architecture Question**: Nico confirmed Sign PDA should be a signer, but our implementation still fails - is there a missing step?

3. **Use Case Question**: Is our survey response submission with encrypted analytics a supported use case in v0.3.0?

4. **Debugging Question**: What additional debugging information would help identify the root cause?

## Additional Context
- All account derivations match between frontend and backend
- MXE account is properly initialized at `4CaE3mJQXmreAd4SeLUtGHFhM56U7wi85f9n3z1suEpH`
- Code follows Arcium v0.3.0 documentation exactly
- Issue occurs specifically with the `queue_computation` CPI call
- Sign PDA Account bump is properly set before CPI call
- All required accounts are included in the account struct
- Cargo.toml has correct dependencies and patches

## Transaction Details
- **Sign PDA Account**: `2PVQxTpX3pNpeNrUQoTAMBehNmkNx9gvJTs68sqKVxSe`
- **MXE Account**: `4CaE3mJQXmreAd4SeLUtGHFhM56U7wi85f9n3z1suEpH`

## Impact
This issue prevents all encrypted instruction execution in Arcium v0.3.0, blocking the core functionality of confidential smart contracts.

## Request for Nico
Hi Nico, please provide guidance on:
1. What could be causing the "unauthorized signer" error despite correct v0.3.0 implementation?
2. Are there any additional steps or configurations required for Sign PDA Accounts?
3. Is this a known issue with v0.3.0 that has a workaround?
4. What debugging steps should we take to identify the root cause?
