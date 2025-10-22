# 🌐 Supabase Edge Functions for SeQure Analytics

## 📁 Project Structure (After Setup)

```
sequre-vite/
├── supabase/
│   ├── functions/
│   │   ├── compute-analytics/          # Our function
│   │   │   ├── index.ts               # Main logic
│   │   │   └── .env                   # Secrets (Solana key, etc)
│   │   └── _shared/                   # Reusable code
│   │       ├── supabaseClient.ts
│   │       └── solanaClient.ts
│   └── config.toml                     # Supabase config
```

---

## 🔧 **How It Works (Step-by-Step)**

### **Flow Diagram:**

```
┌──────────────────────────────────────────────────────────────┐
│  STEP 1: Respondent Submits Survey                           │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  React App (Frontend)                                        │
│  ✅ Encrypt answers with Arcium                              │
│  ✅ Store in Supabase: survey_responses table                │
│  ✅ Store on Solana (proof)                                  │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  STEP 2: Database Trigger Fires (Automatic!)                │
│                                                              │
│  CREATE TRIGGER on_new_response                              │
│  AFTER INSERT ON survey_responses                            │
│  FOR EACH ROW                                                │
│  EXECUTE FUNCTION mark_survey_for_computation();             │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  surveys table updated:                                      │
│  needs_analytics_computation = TRUE                          │
│  last_response_at = NOW()                                    │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  STEP 3: Edge Function Runs (Scheduled via Cron)            │
│                                                              │
│  Trigger: Every 5 minutes (or on-demand)                     │
│  Location: Runs in Supabase cloud (NOT user's browser)      │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  Edge Function Logic:                                        │
│                                                              │
│  1. Query database for surveys needing computation           │
│  2. Fetch encrypted responses for each survey                │
│  3. Call Arcium MPC to compute analytics                     │
│  4. Store results in survey_analytics table                  │
│  5. Mark survey as computed                                  │
└───────────────────────┬──────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────────┐
│  STEP 4: Creator Views Dashboard                            │
│                                                              │
│  React App queries survey_analytics table                    │
│  ✅ Analytics already computed!                              │
│  ✅ Show charts instantly (no waiting)                       │
└──────────────────────────────────────────────────────────────┘
```

---

## 💻 **Code Example:**

### **1. Edge Function Code** (`supabase/functions/compute-analytics/index.ts`)

```typescript
import { serve } from "https://deno.land/std@0.168.0/http/server.ts"
import { createClient } from 'https://esm.sh/@supabase/supabase-js@2'
import { Connection, Keypair, PublicKey } from 'https://esm.sh/@solana/web3.js@1.78.0'
import { AnchorProvider, Program } from 'https://esm.sh/@coral-xyz/anchor@0.28.0'

serve(async (req) => {
  try {
    console.log('🚀 Edge Function: compute-analytics started')
    
    // 1. Initialize Supabase client
    const supabase = createClient(
      Deno.env.get('SUPABASE_URL')!,
      Deno.env.get('SUPABASE_SERVICE_ROLE_KEY')! // Backend-only key!
    )
    
    // 2. Find surveys that need analytics
    const { data: surveys, error } = await supabase
      .from('surveys')
      .select('*')
      .eq('needs_analytics_computation', true)
      .lt('last_response_at', new Date(Date.now() - 5 * 60 * 1000).toISOString()) // 5 min debounce
    
    if (error) throw error
    
    console.log(`📊 Found ${surveys.length} surveys needing computation`)
    
    // 3. Process each survey
    for (const survey of surveys) {
      console.log(`📝 Processing survey: ${survey.id}`)
      
      // Get all responses since last computation
      const { data: responses } = await supabase
        .from('survey_responses')
        .select('*')
        .eq('survey_id', survey.id)
        .gt('created_at', survey.last_computation_at || '1970-01-01')
      
      if (!responses || responses.length === 0) {
        console.log(`⏭️ No new responses for survey ${survey.id}`)
        continue
      }
      
      console.log(`🔐 Computing analytics for ${responses.length} responses`)
      
      // 4. Initialize Solana/Arcium (using service wallet)
      const connection = new Connection(Deno.env.get('SOLANA_RPC_URL')!)
      const serviceKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(Deno.env.get('SERVICE_WALLET_SECRET')!))
      )
      
      // 5. Call Arcium MPC to compute analytics
      const analyticsResult = await computeAnalyticsWithArcium(
        connection,
        serviceKeypair,
        survey.id,
        responses
      )
      
      // 6. Store analytics result
      await supabase
        .from('survey_analytics')
        .upsert({
          survey_id: survey.id,
          analytics_data: analyticsResult,
          computation_tx: analyticsResult.txSignature,
          computed_at: new Date().toISOString()
        })
      
      // 7. Mark survey as processed
      await supabase
        .from('surveys')
        .update({
          needs_analytics_computation: false,
          last_computation_at: new Date().toISOString()
        })
        .eq('id', survey.id)
      
      console.log(`✅ Analytics computed for survey ${survey.id}`)
    }
    
    return new Response(
      JSON.stringify({ 
        success: true, 
        processed: surveys.length 
      }),
      { headers: { 'Content-Type': 'application/json' } }
    )
    
  } catch (error) {
    console.error('❌ Edge Function error:', error)
    return new Response(
      JSON.stringify({ error: error.message }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    )
  }
})

// Helper function to call Arcium MPC
async function computeAnalyticsWithArcium(
  connection: Connection,
  payer: Keypair,
  surveyId: string,
  responses: any[]
): Promise<any> {
  // This would use your existing RealArciumService logic
  // but running in the backend instead of browser
  
  const provider = new AnchorProvider(connection, {
    publicKey: payer.publicKey,
    signTransaction: async (tx) => {
      tx.partialSign(payer)
      return tx
    },
    signAllTransactions: async (txs) => {
      txs.forEach(tx => tx.partialSign(payer))
      return txs
    }
  }, {})
  
  // Initialize Arcium service
  const arciumService = new RealArciumService(provider, PROGRAM_ID)
  
  // Compute analytics
  const result = await arciumService.computeSurveyAnalytics({
    surveyId,
    responses: responses.map(r => r.encrypted_response)
  })
  
  return result
}
```

---

## 🕐 **Triggering the Edge Function**

### **Option 1: Scheduled (Cron Job)**

Use an external cron service to call the function every 5 minutes:

```bash
# Using cron-job.org (free service)
# Create a job that hits this URL every 5 minutes:
https://YOUR_PROJECT.supabase.co/functions/v1/compute-analytics

# Headers:
Authorization: Bearer YOUR_SUPABASE_ANON_KEY
```

**Cron Expression:** `*/5 * * * *` (every 5 minutes)

---

### **Option 2: Database Trigger (Advanced)**

Call Edge Function directly from database:

```sql
-- Install pg_cron extension
CREATE EXTENSION IF NOT EXISTS pg_cron;

-- Schedule function to run every 5 minutes
SELECT cron.schedule(
  'compute-analytics-job',
  '*/5 * * * *',
  $$ 
  SELECT net.http_post(
    url := 'https://YOUR_PROJECT.supabase.co/functions/v1/compute-analytics',
    headers := '{"Authorization": "Bearer YOUR_SERVICE_KEY"}'::jsonb
  ) 
  $$
);
```

---

### **Option 3: Manual Trigger (Testing)**

Call from your React app or terminal:

```typescript
// From React app
const triggerAnalyticsComputation = async () => {
  const response = await fetch(
    'https://YOUR_PROJECT.supabase.co/functions/v1/compute-analytics',
    {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        'Content-Type': 'application/json'
      }
    }
  )
  
  const result = await response.json()
  console.log('Analytics computed:', result)
}
```

---

## 🔐 **Security & Environment Variables**

### **Secrets Stored in Edge Function:**

```bash
# .env file (stored securely in Supabase)
SUPABASE_URL=https://yourproject.supabase.co
SUPABASE_SERVICE_ROLE_KEY=your_secret_service_key  # Backend-only!
SOLANA_RPC_URL=https://api.devnet.solana.com
SERVICE_WALLET_SECRET=[1,2,3...]  # Keypair for paying gas
ARCIUM_MPC_ENDPOINT=https://arcium.mpc.endpoint
```

**Important:**
- ✅ These secrets NEVER exposed to frontend
- ✅ Service wallet only used for analytics (not user data)
- ✅ All encryption happens client-side first

---

## 💰 **Cost Breakdown**

### **Supabase Edge Functions Pricing:**

| Plan | Invocations/month | Cost |
|------|------------------|------|
| **Free Tier** | 500,000 | $0 |
| **Pro** | 2 million | $25/month |
| **Additional** | +1 million | $2 |

### **Your Usage:**

```
Cron runs every 5 minutes = 12/hour × 24 hours × 30 days = 8,640 invocations/month
✅ Stays under FREE tier (500k limit)!
```

### **Solana Costs:**

```
Each MPC computation = ~0.001 SOL ($0.10 at $100/SOL)
If 1,000 surveys/month each get 10 responses = 1,000 computations
Total: ~1 SOL/month ($100) for analytics
```

**Who pays?**
- Option A: Service wallet (you pay for all analytics)
- Option B: Charge survey creators in SOL (they pay per computation)

---

## 🚀 **Setup Steps (Step-by-Step)**

### **1. Install Supabase CLI:**

```bash
# Install via npm
npm install -g supabase

# Or via Homebrew (macOS)
brew install supabase/tap/supabase

# Login
supabase login
```

---

### **2. Initialize Supabase in Your Project:**

```bash
cd /Users/progzzz/Desktop/SeQure\ copy\ 3/sequre-vite

# Initialize (creates supabase/ folder)
supabase init

# Link to your project
supabase link --project-ref YOUR_PROJECT_ID
```

---

### **3. Create the Edge Function:**

```bash
# Create function
supabase functions new compute-analytics

# This creates:
# supabase/functions/compute-analytics/index.ts
```

---

### **4. Set Environment Variables:**

```bash
# Set secrets (stored securely in Supabase)
supabase secrets set SUPABASE_URL=https://yourproject.supabase.co
supabase secrets set SUPABASE_SERVICE_ROLE_KEY=your_service_key
supabase secrets set SOLANA_RPC_URL=https://api.devnet.solana.com
supabase secrets set SERVICE_WALLET_SECRET='[1,2,3,...]'
```

---

### **5. Deploy the Function:**

```bash
# Deploy to Supabase
supabase functions deploy compute-analytics

# Test it
supabase functions invoke compute-analytics
```

---

### **6. Set Up Cron Job:**

Go to https://cron-job.org (free):
1. Create account
2. Add new cron job:
   - **URL:** `https://YOUR_PROJECT.supabase.co/functions/v1/compute-analytics`
   - **Schedule:** Every 5 minutes
   - **Headers:** `Authorization: Bearer YOUR_ANON_KEY`

---

## 🎯 **Benefits for Your Project**

| Benefit | Description |
|---------|-------------|
| 🚀 **Auto-Pilot** | Analytics compute automatically, no user action |
| 💰 **Cost Efficient** | Batch multiple responses, pay once |
| 🔐 **Secure** | Service wallet hidden, never exposed to frontend |
| ⚡ **Fast UX** | Creators see instant results (already computed) |
| 📈 **Scalable** | Handles 1 or 10,000 surveys equally well |
| 🛡️ **Private** | Full Arcium MPC encryption maintained |

---

## 🤔 **Questions Answered**

### **Q: Who pays the Solana gas fees?**
A: The service wallet (backend). You can:
- Absorb costs as platform (build it into pricing)
- Charge creators per survey (add billing logic)
- Use sponsors/grants (decentralized approach)

### **Q: What if Edge Function fails?**
A: Survey marked as `needs_computation = true`, so next cron run retries automatically.

### **Q: Can respondents still see analytics?**
A: No! Only survey creators with proper permissions can query the `survey_analytics` table.

### **Q: Do I NEED Edge Functions?**
A: No! You can use **Solution 2 (Client-Side Batching)** which works great for small-medium scale. Edge Functions are for when you want:
- 24/7 automation (even when no one visits dashboard)
- Better UX (instant analytics)
- Lower frontend complexity

---

## 📊 **Comparison: With vs Without Edge Functions**

### **Without Edge Functions (Client-Side):**
```
Respondent submits → Store response ✅
---
Creator opens dashboard → Trigger computation manually/automatically
Wait 2-5 seconds → Show analytics
```
**Pros:** Simple, no backend  
**Cons:** Computation happens on every dashboard visit, slower UX

---

### **With Edge Functions (Backend):**
```
Respondent submits → Store response ✅
Edge Function (every 5 min) → Compute analytics ✅
---
Creator opens dashboard → Fetch pre-computed analytics ✅
Show instantly (no wait!)
```
**Pros:** Faster UX, scales better, automatic  
**Cons:** Requires setup (1-2 hours)

---

## 🎯 **My Recommendation**

**For MVP/Testing:**  
→ Use **Client-Side Batching** (Solution 2) - simple, works now

**For Production/Scale:**  
→ Upgrade to **Edge Functions** (Solution 1) - better UX, more professional

---

## 🛠️ **Want Me to Set It Up?**

I can:
1. ✅ Create the Edge Function code
2. ✅ Set up database triggers
3. ✅ Configure cron scheduling
4. ✅ Add error handling & logging
5. ✅ Write setup instructions

Just say **"set up Edge Functions"** and I'll create all the files! 🚀



