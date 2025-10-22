#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');

const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const serviceRoleKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

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
        'x-upsert': 'true' // This will overwrite if exists
      }
    };

    console.log(`\n📤 Uploading ${fileName}...`);
    console.log(`   Size: ${(fileBuffer.length / 1024 / 1024).toFixed(2)} MB`);
    console.log(`   URL: ${url.href}`);

    const req = https.request(options, (res) => {
      let data = '';

      res.on('data', (chunk) => {
        data += chunk;
      });

      res.on('end', () => {
        if (res.statusCode === 200 || res.statusCode === 201) {
          console.log(`   ✅ Success!`);
          const publicUrl = `${supabaseUrl}/storage/v1/object/public/arcium-circuits/${fileName}`;
          console.log(`   🔗 ${publicUrl}`);
          resolve({ fileName, url: publicUrl });
        } else {
          console.log(`   ❌ Failed (${res.statusCode}): ${data}`);
          reject(new Error(`Upload failed: ${res.statusCode} - ${data}`));
        }
      });
    });

    req.on('error', (error) => {
      console.log(`   ❌ Request error: ${error.message}`);
      reject(error);
    });

    req.write(fileBuffer);
    req.end();
  });
}

async function uploadAllCircuits() {
  console.log('🚀 Uploading Circuit Files to Supabase');
  console.log('========================================\n');

  const buildDir = path.join(__dirname, 'build');
  const circuitFiles = [
    'survey_analytics.arcis',
    'quiz_evaluation.arcis',
    'analytics_computation.arcis',
    'quiz_threshold_check.arcis'
  ];

  const results = [];

  for (const fileName of circuitFiles) {
    const filePath = path.join(buildDir, fileName);
    
    if (!fs.existsSync(filePath)) {
      console.log(`❌ File not found: ${fileName}`);
      continue;
    }

    try {
      const fileBuffer = fs.readFileSync(filePath);
      const result = await uploadFile(fileName, fileBuffer);
      results.push(result);
    } catch (error) {
      console.log(`   Error: ${error.message}`);
    }
  }

  console.log('\n📊 Upload Summary');
  console.log('==================');
  console.log(`✅ Successful: ${results.length}/${circuitFiles.length}`);
  
  if (results.length > 0) {
    console.log('\n🎯 Uploaded URLs:');
    results.forEach(r => console.log(`   ${r.fileName}: ${r.url}`));
    
    const urlsObj = {};
    results.forEach(r => { urlsObj[r.fileName] = r.url; });
    fs.writeFileSync(
      path.join(__dirname, 'circuit_urls.json'),
      JSON.stringify(urlsObj, null, 2)
    );
    console.log('\n💾 URLs saved to circuit_urls.json');
  }

  if (results.length === circuitFiles.length) {
    console.log('\n✅ All files uploaded successfully!');
    process.exit(0);
  } else {
    console.log('\n⚠️  Some uploads failed. See errors above.');
    console.log('\n🔧 TROUBLESHOOTING:');
    console.log('If uploads keep failing, the bucket might have RLS policies blocking uploads.');
    console.log('\nTo fix in Supabase Dashboard:');
    console.log('1. Go to Storage → arcium-circuits bucket');
    console.log('2. Click "Policies" tab');
    console.log('3. Disable RLS or add policy: (auth.role() = \'service_role\')');
    console.log('\nOr just manually drag & drop the 4 files from:');
    console.log(`   ${buildDir}/`);
    process.exit(1);
  }
}

uploadAllCircuits();


