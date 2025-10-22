#!/usr/bin/env node

const { createClient } = require('@supabase/supabase-js');
const fs = require('fs');
const path = require('path');

// Supabase configuration with SERVICE ROLE key
const supabaseUrl = 'https://eswjamjanympzqopbqyt.supabase.co';
const supabaseServiceKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVzd2phbXphbnltcHpxb3BicXl0Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2MDE3OTcwMywiZXhwIjoyMDc1NzU1NzAzfQ.0DnNVjYD433YiLZVxBSTMvR8jRSmMcQRcDQ1iQBS8tE';

const supabase = createClient(supabaseUrl, supabaseServiceKey);

async function setupBucket() {
  console.log('🔧 Setting up Supabase Storage Bucket...\n');
  
  const bucketName = 'arcium-circuits';
  
  // Step 1: Check if bucket exists
  console.log('1️⃣ Checking if bucket exists...');
  const { data: buckets, error: listError } = await supabase.storage.listBuckets();
  
  if (listError) {
    console.log(`   ❌ Error listing buckets: ${listError.message}`);
    return false;
  }
  
  const bucketExists = buckets.some(b => b.name === bucketName);
  
  if (bucketExists) {
    console.log(`   ✅ Bucket '${bucketName}' already exists`);
  } else {
    // Step 2: Create bucket if it doesn't exist
    console.log(`   ⚠️  Bucket '${bucketName}' does not exist. Creating it...`);
    
    const { data: newBucket, error: createError } = await supabase.storage.createBucket(bucketName, {
      public: true, // Make bucket public so URLs are accessible
      fileSizeLimit: 52428800, // 50 MB limit per file
      allowedMimeTypes: ['application/octet-stream']
    });
    
    if (createError) {
      console.log(`   ❌ Error creating bucket: ${createError.message}`);
      console.log(`   💡 TIP: You may need to create the bucket manually in Supabase Dashboard:`);
      console.log(`      1. Go to: ${supabaseUrl.replace('https://', 'https://supabase.com/dashboard/project/')}/storage/buckets`);
      console.log(`      2. Click "New bucket"`);
      console.log(`      3. Name it: ${bucketName}`);
      console.log(`      4. Make it Public`);
      console.log(`      5. Run this script again`);
      return false;
    }
    
    console.log(`   ✅ Bucket '${bucketName}' created successfully!`);
  }
  
  return true;
}

async function uploadCircuitFiles() {
  console.log('\n🚀 Uploading Arcium Circuit Files...');
  console.log('=====================================\n');
  
  const buildDir = path.join(__dirname, 'build');
  const circuitFiles = [
    'survey_analytics.arcis',
    'quiz_evaluation.arcis', 
    'analytics_computation.arcis',
    'quiz_threshold_check.arcis'
  ];
  
  const uploadedUrls = {};
  let successCount = 0;
  
  for (const fileName of circuitFiles) {
    const filePath = path.join(buildDir, fileName);
    
    if (!fs.existsSync(filePath)) {
      console.log(`❌ File not found: ${fileName}`);
      continue;
    }
    
    console.log(`📁 Uploading ${fileName}...`);
    
    try {
      // Read the file
      const fileBuffer = fs.readFileSync(filePath);
      const fileSizeMB = (fileBuffer.length / 1024 / 1024).toFixed(2);
      console.log(`   File size: ${fileSizeMB} MB`);
      
      // Upload to Supabase Storage
      const { data, error } = await supabase.storage
        .from('arcium-circuits')
        .upload(fileName, fileBuffer, {
          contentType: 'application/octet-stream',
          upsert: true, // Overwrite if exists
          cacheControl: '3600'
        });
      
      if (error) {
        console.log(`   ❌ Upload failed: ${error.message}`);
        console.log(`   Error code: ${error.statusCode || error.status}`);
        
        if (error.message.includes('signature verification failed')) {
          console.log(`\n   💡 TROUBLESHOOTING: Bucket policy issue detected!`);
          console.log(`   The bucket exists but doesn't allow uploads. Fix this by:`);
          console.log(`   1. Go to Supabase Dashboard → Storage → arcium-circuits bucket`);
          console.log(`   2. Click "Policies" tab`);
          console.log(`   3. Create a new policy with:`);
          console.log(`      - Name: "Allow authenticated uploads"`);
          console.log(`      - Operation: INSERT`);
          console.log(`      - Policy: (auth.role() = 'authenticated') OR (auth.role() = 'service_role')`);
          console.log(`   4. Repeat for UPDATE operation (for upserts)`);
        }
        continue;
      }
      
      // Get public URL
      const { data: urlData } = supabase.storage
        .from('arcium-circuits')
        .getPublicUrl(fileName);
      
      const publicUrl = urlData.publicUrl;
      uploadedUrls[fileName] = publicUrl;
      successCount++;
      
      console.log(`   ✅ Uploaded successfully!`);
      console.log(`   🔗 URL: ${publicUrl}\n`);
      
    } catch (err) {
      console.log(`   ❌ Error uploading ${fileName}: ${err.message}\n`);
    }
  }
  
  console.log('\n📋 Upload Summary:');
  console.log('==================');
  console.log(`✅ Successful: ${successCount}/${circuitFiles.length}`);
  console.log(`❌ Failed: ${circuitFiles.length - successCount}/${circuitFiles.length}`);
  
  if (successCount > 0) {
    console.log('\n🎯 Uploaded Circuit URLs:');
    for (const [fileName, url] of Object.entries(uploadedUrls)) {
      console.log(`   ${fileName}: ${url}`);
    }
    
    // Save URLs to config file
    const configPath = path.join(__dirname, 'circuit_urls.json');
    fs.writeFileSync(configPath, JSON.stringify(uploadedUrls, null, 2));
    console.log(`\n💾 URLs saved to: ${configPath}`);
  }
  
  return { successCount, total: circuitFiles.length, urls: uploadedUrls };
}

// Main execution
async function main() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('   Arcium Circuit Upload Tool');
  console.log('═══════════════════════════════════════════════════════\n');
  
  // Step 1: Setup bucket
  const bucketReady = await setupBucket();
  
  if (!bucketReady) {
    console.log('\n❌ Bucket setup failed. Please fix the issues above and try again.');
    process.exit(1);
  }
  
  // Step 2: Upload files
  const result = await uploadCircuitFiles();
  
  if (result.successCount === result.total) {
    console.log('\n🎉 All circuit files uploaded successfully!');
    console.log('✅ You can now proceed with deployment.\n');
    process.exit(0);
  } else if (result.successCount > 0) {
    console.log('\n⚠️  Some files uploaded, but some failed.');
    console.log('Please fix the issues above and run again.\n');
    process.exit(1);
  } else {
    console.log('\n❌ No files were uploaded successfully.');
    console.log('Please fix the bucket policies in Supabase Dashboard.\n');
    process.exit(1);
  }
}

main().catch(error => {
  console.error('\n💥 Fatal error:', error);
  process.exit(1);
});


