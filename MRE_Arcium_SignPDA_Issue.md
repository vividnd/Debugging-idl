# MRE: Sign PDA Account Signer Privilege Escalation in Arcium v0.3.0

## Environment Details
- **Program ID**: `Crky8qtmLUVESVNTnTUgLaVnUdgBvCQuFzsGmEFSKmZ2`
- **Network**: Devnet
- **Arcium SDK Version**: 0.3.0
- **Anchor Version**: 0.31.1
- **Solana Version**: 1.18.x

## Issue Description
The `#[queue_computation_accounts]` macro incorrectly treats the Sign PDA Account as a signer during CPI calls, causing "Cross-program invocation with unauthorized signer" errors.

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
        address = derive_computation_pda!(computation_offset)
    )]
    pub computation_account: Account<'info, ComputationAccount>,
    #[account(
        mut,
        address = derive_survey_pda!(survey_id)
    )]
    pub survey_account: Account<'info, SurveyAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, ArciumProgram>,
}
```

### Instruction Implementation
```rust
pub fn submit_survey_response(
    ctx: Context<SubmitSurveyResponse>,
    computation_offset: u64,
    args: SurveyAnalyticsArgs,
) -> Result<()> {
    // Set bump for Sign PDA Account
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
    
    // Call queue_computation - this is where the error occurs
    queue_computation(ctx.accounts, computation_offset, args, None, callbacks)?;
    
    Ok(())
}
```

### Account Derivations
```rust
// Sign PDA Account derivation
pub const SIGN_PDA_SEED: &[u8] = b"sign_pda";

// MXE Account derivation (using our program ID)
pub fn derive_mxe_pda() -> Pubkey {
    get_mxe_acc_address(&crate::ID)
}

// Computation Account derivation
pub fn derive_computation_pda(offset: u64) -> Pubkey {
    get_computation_acc_address(&crate::ID, &offset)
}
```

## Steps to Reproduce
1. Deploy the program to devnet
2. Initialize computation definitions using `arcium deploy`
3. Create a survey with analytics enabled
4. Attempt to submit a survey response
5. Observe the signer privilege escalation error

## Expected vs Actual Behavior

### Expected
- Sign PDA Account should be writable but not a signer
- CPI call to `queue_computation` should succeed
- Survey response should be processed with encrypted analytics

### Actual
- Sign PDA Account is escalated to signer privileges
- CPI call fails with "unauthorized signer" error
- Survey response submission is blocked

## Key Questions 

1. **Architecture Question**: How can a PDA be a signer in CPI calls when Solana's security model prevents PDAs from being signers?

2. **Implementation Question**: What's the correct pattern for implementing Sign PDA Accounts in the `#[queue_computation_accounts]` macro?

3. **Use Case Question**: Is our survey response submission with encrypted analytics a supported use case?

4. **Documentation Question**: Can you provide a working example of the correct implementation?

## Additional Context
- All account derivations match between frontend and backend
- MXE account is properly initialized at `4CaE3mJQXmreAd4SeLUtGHFhM56U7wi85f9n3z1suEpH`
- Code follows Arcium v0.3.0 documentation exactly
- Issue occurs specifically with the `queue_computation` CPI call

## Transaction Details
- **Sign PDA Account**: `2PVQxTpX3pNpeNrUQoTAMBehNmkNx9gvJTs68sqKVxSe`
- **MXE Account**: `4CaE3mJQXmreAd4SeLUtGHFhM56U7wi85f9n3z1suEpH`

## Impact
This issue prevents all encrypted instruction execution in Arcium v0.3.0, blocking the core functionality of confidential smart contracts.

## Request
Please provide guidance on:
1. The correct implementation pattern for Sign PDA Accounts
2. Whether this is a known issue with a workaround
3. Expected timeline for resolution if this is a bug
