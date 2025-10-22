#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');

const baseUrl = 'https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public/arcium-circuits';

const files = [
  { name: 'survey_analytics.arcis', expectedSize: 3.4 },
  { name: 'quiz_evaluation.arcis', expectedSize: 6.9 },
  { name: 'analytics_computation.arcis', expectedSize: 5.7 },
  { name: 'quiz_threshold_check.arcis', expectedSize: 3.6 }
];

function checkFile(fileName) {
  return new Promise((resolve) => {
    const url = `${baseUrl}/${fileName}`;
    
    https.get(url, (res) => {
      if (res.statusCode === 200) {
        const contentLength = parseInt(res.headers['content-length'] || '0');
        const sizeMB = (contentLength / 1024 / 1024).toFixed(2);
        
        res.on('data', () => {}); // Consume the stream
        res.on('end', () => {
          resolve({ success: true, size: sizeMB, url });
        });
      } else {
        resolve({ success: false, status: res.statusCode, url });
      }
    }).on('error', (err) => {
      resolve({ success: false, error: err.message, url });
    });
  });
}

async function verifyAll() {
  console.log('🔍 Verifying uploaded circuit files...\n');
  
  let allGood = true;
  
  for (const file of files) {
    console.log(`Checking ${file.name}...`);
    const result = await checkFile(file.name);
    
    if (result.success) {
      console.log(`   ✅ Accessible (${result.size} MB)`);
      console.log(`   🔗 ${result.url}`);
      
      // Check if size is reasonable
      const sizeDiff = Math.abs(parseFloat(result.size) - file.expectedSize);
      if (sizeDiff > 1) {
        console.log(`   ⚠️  Size mismatch! Expected ~${file.expectedSize} MB, got ${result.size} MB`);
        allGood = false;
      }
    } else {
      console.log(`   ❌ Not accessible (Status: ${result.status || 'Error'})`);
      if (result.error) console.log(`   Error: ${result.error}`);
      allGood = false;
    }
    console.log('');
  }
  
  console.log('═══════════════════════════════════════');
  if (allGood) {
    console.log('✅ All files verified successfully!');
    console.log('✅ Ready to proceed with deployment!');
  } else {
    console.log('⚠️  Some files have issues. Check above.');
  }
  console.log('═══════════════════════════════════════');
  
  return allGood;
}

verifyAll();


