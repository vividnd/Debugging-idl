#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');
const fs = require('fs');
const path = require('path');
const https = require('https');

const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const serviceRoleKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

const supabase = createClient(supabaseUrl, serviceRoleKey);

async function createBucket() {
  console.log('🔧 Creating arcium-circuits bucket...\n');
  
  const { data, error } = await supabase.storage.createBucket('arcium-circuits', {
    public: true,
    fileSizeLimit: 52428800, // 50MB
  });
  
  if (error) {
    if (error.message.includes('already exists')) {
      console.log('   ✅ Bucket already exists');
      return true;
    }
    console.log('   ❌ Error:', error.message);
    return false;
  }
  
  console.log('   ✅ Bucket created successfully!');
  return true;
}

function uploadFile(fileName, fileBuffer) {
  return new Promise((resolve, reject) => {
    const url = new URL(`${supabaseUrl}/storage/v1/object/arcium-circuits/${fileName}`);
    
    const options = {
      hostname: url.hostname,
      path: url.pathname,
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${serviceRoleKey}`,
        'apikey': serviceRoleKey,
        'Content-Type': 'application/octet-stream',
        'Content-Length': fileBuffer.length,
      }
    };

    console.log(`📤 Uploading ${fileName} (${(fileBuffer.length / 1024 / 1024).toFixed(2)} MB)...`);

    const req = https.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => { data += chunk; });
      res.on('end', () => {
        if (res.statusCode === 200 || res.statusCode === 201) {
          const publicUrl = `${supabaseUrl}/storage/v1/object/public/arcium-circuits/${fileName}`;
          console.log(`   ✅ Success! ${publicUrl}\n`);
          resolve({ fileName, url: publicUrl });
        } else {
          console.log(`   ❌ Failed (${res.statusCode}): ${data}\n`);
          reject(new Error(`${res.statusCode} - ${data}`));
        }
      });
    });

    req.on('error', (error) => {
      console.log(`   ❌ Error: ${error.message}\n`);
      reject(error);
    });

    req.write(fileBuffer);
    req.end();
  });
}

async function main() {
  console.log('═══════════════════════════════════════');
  console.log('   Circuit Upload Tool');
  console.log('═══════════════════════════════════════\n');

  // Step 1: Create bucket
  const created = await createBucket();
  if (!created) {
    console.log('\n❌ Failed to create bucket. You may need to do it manually in Supabase Dashboard.');
    console.log('Go to: https://supabase.com/dashboard/project/eswjamjanympzqopbqyt/storage/buckets');
    console.log('Click "New bucket", name it "arcium-circuits", make it Public, then run this script again.');
    process.exit(1);
  }

  // Step 2: Upload files
  console.log('\n🚀 Uploading circuit files...\n');
  
  const buildDir = path.join(__dirname, 'build');
  const files = [
    'survey_analytics.arcis',
    'quiz_evaluation.arcis',
    'analytics_computation.arcis',
    'quiz_threshold_check.arcis'
  ];

  const results = [];

  for (const fileName of files) {
    const filePath = path.join(buildDir, fileName);
    if (!fs.existsSync(filePath)) {
      console.log(`❌ File not found: ${fileName}\n`);
      continue;
    }

    try {
      const fileBuffer = fs.readFileSync(filePath);
      const result = await uploadFile(fileName, fileBuffer);
      results.push(result);
    } catch (error) {
      console.log(`Error uploading ${fileName}`);
    }
  }

  console.log('\n📊 Summary:');
  console.log('===========');
  console.log(`✅ Uploaded: ${results.length}/${files.length}`);

  if (results.length > 0) {
    console.log('\n🎯 URLs:');
    results.forEach(r => console.log(`   ${r.fileName}: ${r.url}`));
    
    const urlsObj = {};
    results.forEach(r => { urlsObj[r.fileName] = r.url; });
    fs.writeFileSync(
      path.join(__dirname, 'circuit_urls.json'),
      JSON.stringify(urlsObj, null, 2)
    );
  }

  if (results.length === files.length) {
    console.log('\n✅ All files uploaded! Ready for deployment.');
    process.exit(0);
  } else {
    console.log('\n⚠️ Some files failed. Check errors above.');
    process.exit(1);
  }
}

main();


