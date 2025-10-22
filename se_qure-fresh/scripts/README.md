# SeQure Initialization Scripts

This directory contains scripts for initializing the SeQure program's computation definitions on devnet.

## Files

- `init_comp_defs.ts` - Main initialization script for all 4 computation definitions
- `README.md` - This documentation file

## Prerequisites

1. **Program Deployed**: Your SeQure program must be deployed to devnet
2. **MXE Initialized**: The Arcium MXE account must be initialized on devnet
3. **Wallet Setup**: Your Solana wallet at `~/.config/solana/id.json` must have sufficient SOL
4. **Dependencies**: All npm dependencies must be installed (`npm install`)

### MXE Initialization

**Important**: The MXE (Multi-party eXecution Environment) account must be initialized before running this script. If you get an error about MXE not being initialized, you need to:

1. **Use Arcium CLI** (recommended):
   ```bash
   arcium init-mxe --cluster-offset 1078779259 --keypair-path ~/.config/solana/id.json --rpc-url https://api.devnet.solana.com
   ```

2. **Or use Arcium Dashboard**: Visit the Arcium dashboard and initialize MXE through the web interface

3. **Or call MXE initialization directly**: Use the Arcium program's MXE initialization instruction

The MXE account address should be: `3rmBdxKarab9cghhGbs9Tb8D8GutxgRn1FaBhjsjNqxG`

## Usage

### Method 1: Using npm script (Recommended)

```bash
cd se_qure
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com ANCHOR_WALLET=~/.config/solana/id.json npm run init-comp-defs
```

### Method 2: Direct execution

```bash
cd se_qure
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com ANCHOR_WALLET=~/.config/solana/id.json npx ts-node scripts/init_comp_defs.ts
```

### Method 3: Using existing devnet script

```bash
cd se_qure
npm run devnet:init
```

## What the Script Does

The initialization script will:

1. **Connect to devnet** using the specified RPC endpoint
2. **Load your wallet** from `~/.config/solana/id.json`
3. **Verify MXE account** exists on devnet
4. **Initialize 4 computation definitions**:
   - `survey_analytics` - For survey response analytics
   - `quiz_evaluation` - For quiz grading and evaluation
   - `analytics_computation` - For advanced analytics processing
   - `quiz_threshold_check` - For quiz completion verification

## Expected Output

```
🚀 SeQure Computation Definitions Initialization
==================================================

📡 Setting up connection...
   ✅ Connected to Solana 1.18.4 on devnet

🔑 Loading wallet...
   ✅ Wallet loaded: YOUR_WALLET_ADDRESS
   💰 Wallet balance: X.XXXX SOL

🔧 Setting up Anchor program...
   ✅ Program loaded: YOUR_PROGRAM_ID

🔍 Verifying MXE account...
   ✅ MXE account verified: MXE_ACCOUNT_ADDRESS

🎯 Initializing computation definitions...

🔄 Initializing survey_analytics computation definition...
   MXE Account: MXE_ACCOUNT_ADDRESS
   Comp Def Account: COMP_DEF_ADDRESS
   ✅ survey_analytics computation definition initialized successfully!
   📝 Transaction signature: TRANSACTION_SIGNATURE

🔄 Initializing quiz_evaluation computation definition...
   ✅ quiz_evaluation computation definition initialized successfully!

🔄 Initializing analytics_computation computation definition...
   ✅ analytics_computation computation definition initialized successfully!

🔄 Initializing quiz_threshold_check computation definition...
   ✅ quiz_threshold_check computation definition initialized successfully!

📊 Initialization Summary
==============================
   survey_analytics: ✅ SUCCESS
   quiz_evaluation: ✅ SUCCESS
   analytics_computation: ✅ SUCCESS
   quiz_threshold_check: ✅ SUCCESS

🎉 Initialization complete! 4/4 computation definitions ready.

🚀 All computation definitions are now ready for use!
   You can now create surveys, quizzes, and submit responses with MPC processing.

✅ Script completed successfully!
```

## Troubleshooting

### Common Issues

1. **"MXE account not found"**
   - Ensure MXE is initialized on devnet
   - Check that you're using the correct Arcium program ID

2. **"Insufficient SOL balance"**
   - Fund your wallet with SOL from a devnet faucet
   - Each initialization requires ~0.01-0.02 SOL for transaction fees

3. **"Program not found"**
   - Ensure your program is deployed to devnet
   - Check that the program ID in your code matches the deployed program

4. **"Transaction failed"**
   - Check your RPC endpoint is accessible
   - Ensure your wallet has sufficient SOL
   - Verify the program is properly deployed

### Re-running the Script

The script is **idempotent** - it can be run multiple times safely. If a computation definition is already initialized, it will skip that initialization and show:

```
✅ survey_analytics computation definition already initialized
```

## Configuration

The script uses these default configurations:

- **RPC Endpoint**: `https://api.devnet.solana.com`
- **Arcium Program ID**: `BKck65TgoKRokMjQM3datB9oRwJ8rAj2jxPXvHXUvcL6`
- **Wallet Path**: `~/.config/solana/id.json`

To use different configurations, modify the constants at the top of `init_comp_defs.ts`.

## Next Steps

After successful initialization:

1. **Test survey creation** with MPC processing
2. **Test quiz submission** with encrypted evaluation
3. **Verify MPC results** in your database
4. **Monitor computation completion** events

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Verify all prerequisites are met
3. Check the transaction signatures in Solana Explorer
4. Review the error messages for specific guidance
