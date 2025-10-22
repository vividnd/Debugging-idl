#!/usr/bin/env node

const key = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

console.log('🔍 Verifying JWT Key...\n');

// Decode JWT (it's just base64, no verification needed for inspection)
const parts = key.split('.');
if (parts.length !== 3) {
  console.log('❌ Invalid JWT format');
  process.exit(1);
}

const payload = JSON.parse(Buffer.from(parts[1], 'base64').toString());

console.log('Decoded JWT Payload:');
console.log(JSON.stringify(payload, null, 2));

console.log('\n✅ Key Role:', payload.role);
console.log('✅ Project Ref:', payload.ref);
console.log('✅ Issued At:', new Date(payload.iat * 1000).toLocaleString());
console.log('✅ Expires At:', new Date(payload.exp * 1000).toLocaleString());

if (payload.role === 'service_role') {
  console.log('\n✅ This IS a service_role key!');
  console.log('\n📝 The key is correct, but Supabase is still blocking it.');
  console.log('\n🔧 SOLUTION: Please go to Supabase Dashboard and:');
  console.log('   1. https://supabase.com/dashboard/project/eswjamjanympzqopbqyt/storage/buckets/arcium-circuits');
  console.log('   2. Click "Policies" tab');
  console.log('   3. Click "New Policy"');
  console.log('   4. Select "For full customization"');
  console.log('   5. Name it: "Allow service_role all access"');
  console.log('   6. Target roles: service_role');
  console.log('   7. Policy definition: true');
  console.log('   8. Check ALL operations: SELECT, INSERT, UPDATE, DELETE');
  console.log('   9. Save');
  console.log('\n   OR simply toggle OFF "Enable RLS" for this bucket');
} else {
  console.log('\n❌ This is NOT a service_role key!');
  console.log('   Current role:', payload.role);
  console.log('\n📝 Please get the correct key:');
  console.log('   1. Go to: https://supabase.com/dashboard/project/eswjamjanympzqopbqyt/settings/api');
  console.log('   2. Find "service_role" key (NOT "anon" key)');
  console.log('   3. Copy it and give it to me');
}


