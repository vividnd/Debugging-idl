#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');

const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const serviceRoleKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

const supabase = createClient(supabaseUrl, serviceRoleKey);

async function deleteOldCircuits() {
  console.log('🗑️  Deleting old circuit files from Supabase...\n');

  const filesToDelete = [
    'survey_analytics.arcis',
    'quiz_evaluation.arcis',
    'analytics_computation.arcis',
    'quiz_threshold_check.arcis'
  ];

  for (const fileName of filesToDelete) {
    console.log(`Deleting ${fileName}...`);
    
    const { data, error } = await supabase.storage
      .from('arcium-circuits')
      .remove([fileName]);
    
    if (error) {
      console.log(`   ⚠️  ${error.message}`);
    } else {
      console.log(`   ✅ Deleted`);
    }
  }

  console.log('\n✅ Old files deleted! Now upload the new ones from Finder.');
}

deleteOldCircuits();


