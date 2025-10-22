#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');

const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const supabaseServiceKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

console.log('Testing Supabase Service Role Key...\n');
console.log('URL:', supabaseUrl);
console.log('Key (first 50 chars):', supabaseServiceKey.substring(0, 50) + '...');

const supabase = createClient(supabaseUrl, supabaseServiceKey, {
  auth: {
    autoRefreshToken: false,
    persistSession: false
  }
});

async function testAuth() {
  console.log('\n1. Testing bucket listing...');
  const { data: buckets, error: listError } = await supabase.storage.listBuckets();
  
  if (listError) {
    console.log('   ❌ Error:', listError);
    console.log('\n💡 This suggests the service role key may be incorrect or the project has restrictions.');
    console.log('\n📝 HOW TO FIX IN SUPABASE DASHBOARD:');
    console.log('   1. Go to: https://supabase.com/dashboard/project/eswjamjanympzqopbqyt/settings/api');
    console.log('   2. Scroll to "Project API keys"');
    console.log('   3. Copy the "service_role" key (NOT anon key)');
    console.log('   4. Verify it matches the key in the script');
    console.log('\n   OR try manual upload:');
    console.log('   1. Go to: https://supabase.com/dashboard/project/eswjamjanympzqopbqyt/storage/buckets');
    console.log('   2. Create/select "arcium-circuits" bucket (make it Public)');
    console.log('   3. Click "Upload file" and upload these 4 files from:');
    console.log('      /Users/progzzz/Desktop/SeQure copy 3/se_qure/build/');
    console.log('      - survey_analytics.arcis');
    console.log('      - quiz_evaluation.arcis');
    console.log('      - analytics_computation.arcis');
    console.log('      - quiz_threshold_check.arcis');
    return false;
  }
  
  console.log('   ✅ Success! Found', buckets.length, 'bucket(s)');
  buckets.forEach(b => console.log('      -', b.name, '(public:', b.public, ')'));
  
  return true;
}

testAuth().then(success => {
  if (success) {
    console.log('\n✅ Authentication working! You can proceed with upload.');
  } else {
    console.log('\n❌ Authentication failed. Follow the instructions above.');
  }
}).catch(err => {
  console.log('\n💥 Unexpected error:', err);
});


