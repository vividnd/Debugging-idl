#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');
const fs = require('fs');
const path = require('path');

// Supabase configuration with SERVICE ROLE key
const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const supabaseServiceKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

const supabase = createClient(supabaseUrl, supabaseServiceKey);

async function uploadCircuitFiles() {
  console.log('🚀 Uploading Arcium Circuit Files to Supabase Storage...');
  console.log('=====================================================');
  
  const buildDir = path.join(__dirname, 'build');
  const circuitFiles = [
    'survey_analytics.arcis',
    'quiz_evaluation.arcis', 
    'analytics_computation.arcis',
    'quiz_threshold_check.arcis'
  ];
  
  const uploadedUrls = {};
  
  for (const fileName of circuitFiles) {
    const filePath = path.join(buildDir, fileName);
    
    if (!fs.existsSync(filePath)) {
      console.log(`❌ File not found: ${fileName}`);
      continue;
    }
    
    console.log(`\n📁 Uploading ${fileName}...`);
    
    try {
      // Read the file
      const fileBuffer = fs.readFileSync(filePath);
      
      // Upload to Supabase Storage in arcium-circuits bucket
      const { data, error } = await supabase.storage
        .from('arcium-circuits')
        .upload(fileName, fileBuffer, {
          contentType: 'application/octet-stream',
          upsert: true // Overwrite if exists
        });
      
      if (error) {
        console.log(`❌ Upload failed: ${error.message}`);
        console.log('   Error details:', error);
        continue;
      }
      
      // Get public URL
      const { data: urlData } = supabase.storage
        .from('arcium-circuits')
        .getPublicUrl(fileName);
      
      const publicUrl = urlData.publicUrl;
      uploadedUrls[fileName] = publicUrl;
      
      console.log(`✅ Uploaded successfully!`);
      console.log(`   Public URL: ${publicUrl}`);
      console.log(`   File size: ${(fileBuffer.length / 1024 / 1024).toFixed(2)} MB`);
      
    } catch (err) {
      console.log(`❌ Error uploading ${fileName}: ${err.message}`);
    }
  }
  
  console.log('\n📋 Summary of Uploaded Circuit Files:');
  console.log('=====================================');
  for (const [fileName, url] of Object.entries(uploadedUrls)) {
    console.log(`${fileName}: ${url}`);
  }
  
  // Save URLs to a config file
  const configPath = path.join(__dirname, 'circuit_urls.json');
  fs.writeFileSync(configPath, JSON.stringify(uploadedUrls, null, 2));
  console.log(`\n💾 Circuit URLs saved to: ${configPath}`);
  
  return uploadedUrls;
}

// Run the upload
uploadCircuitFiles()
  .then((urls) => {
    console.log('\n🎉 Circuit file upload completed!');
    console.log('You can now proceed with deployment.');
  })
  .catch((error) => {
    console.error('❌ Upload failed:', error);
    process.exit(1);
  });


