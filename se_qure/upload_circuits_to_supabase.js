#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');
const fs = require('fs');
const path = require('path');

// Supabase configuration
const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const supabaseKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjAxNzk3MDMsImV4cCI6MjA3NTc1NTcwM30.PyJsFkWDpPrE_t_6AXlxbAKPoXAsJdXAoDU_iGYtAmI';

const supabase = createClient(supabaseUrl, supabaseKey);

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
      
      // Upload to Supabase Storage
      const { data, error } = await supabase.storage
        .from('circuits')
        .upload(`arcium/${fileName}`, fileBuffer, {
          contentType: 'application/octet-stream',
          upsert: true // Overwrite if exists
        });
      
      if (error) {
        console.log(`❌ Upload failed: ${error.message}`);
        continue;
      }
      
      // Get public URL
      const { data: urlData } = supabase.storage
        .from('circuits')
        .getPublicUrl(`arcium/${fileName}`);
      
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
  
  // Save URLs to a config file for Arcium
  const configPath = path.join(__dirname, 'circuit_urls.json');
  fs.writeFileSync(configPath, JSON.stringify(uploadedUrls, null, 2));
  console.log(`\n💾 Circuit URLs saved to: ${configPath}`);
  
  return uploadedUrls;
}

// Run the upload
uploadCircuitFiles()
  .then((urls) => {
    console.log('\n🎉 Circuit file upload completed!');
    console.log('You can now use these URLs in your Arcium deployment.');
  })
  .catch((error) => {
    console.error('❌ Upload failed:', error);
    process.exit(1);
  });
