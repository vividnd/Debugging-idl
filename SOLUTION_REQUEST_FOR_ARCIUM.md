# Arcium IDL Issue - Solution Request

## Problem Summary

Arcium 0.3.0's `arcium build` generates incomplete IDLs where accounts have discriminators but missing type definitions. This breaks Anchor 0.31.1's `BorshCoder` initialization.

## What We've Tried

1. ✅ **Account enrichment** - Adding type definitions from `types` array to `accounts`  
   Result: Fixes `BorshCoder` but breaks `AccountClient` (case sensitivity: `ClockAccount` vs `clockAccount`)

2. ✅ **Empty accounts array** - Remove accounts to avoid `AccountClient` errors  
   Result: Fixes `AccountClient` but `BorshCoder` type registry stays empty

3. ✅ **Explicit `BorshCoder`** - Manually create coder and pass to `Program`  
   Result: Same as #1 - AccountClient still fails

## The Core Conflict

- **Anchor's `Program` constructor does TWO things:**
  1. Creates `BorshCoder` for instruction encoding (needs `types` OR enriched `accounts`)
  2. Creates `AccountClient` for `program.account.*` methods (needs valid account definitions with correct casing)

- **Arcium's IDL only supports #1, not #2**

## Question for Arcium Team

**Is there a recommended way to use Arcium 0.3.0 IDLs with Anchor 0.31.1's `Program` class that:**
- ✅ Allows instruction calls (e.g., `program.methods.createSurvey()`)
- ✅ Properly encodes enum types (e.g., `SurveyType`)
- ❌ Doesn't require `program.account.*` methods (we can fetch accounts manually)

## Proposed Solutions

### Option A: Arcium generates complete IDLs
Update `arcium build` to include full account type definitions like:
```json
{
  "accounts": [
    {
      "name": "Survey",
      "discriminator": [...],
      "type": {  // ← Add this
        "kind": "struct",
        "fields": [...]
      }
    }
  ]
}
```

### Option B: Provide utility to enrich IDLs
Ship a helper like `@arcium/idl-utils` that can:
```typescript
import { enrichArciumIdl } from '@arcium/idl-utils';
const fullIdl = enrichArciumIdl(arciumIdl);
const program = new Program(fullIdl, provider);
```

### Option C: Document the workaround
If there's an existing solution, please document it in the Arcium docs.

## Environment
- Arcium CLI: 0.3.0
- Arcium SDK: 0.3.0
- Anchor: 0.31.1
- Node.js: 24.8.0

## Files for Reference
- `current_broken_idl.json` - Current Arcium-generated IDL
- `old_working_idl.json` - Previous version (identical issue)
- Full debugging details in repo README

Thank you!
