const https = require('https');

async function checkUrl(url) {
  return new Promise((resolve) => {
    const req = https.get(url, (res) => {
      resolve({
        url,
        status: res.statusCode,
        accessible: res.statusCode === 200,
        contentLength: res.headers['content-length'],
        contentType: res.headers['content-type']
      });
    });
    
    req.on('error', (error) => {
      resolve({
        url,
        status: 'ERROR',
        accessible: false,
        error: error.message
      });
    });
    
    req.setTimeout(10000, () => {
      req.destroy();
      resolve({
        url,
        status: 'TIMEOUT',
        accessible: false,
        error: 'Request timeout'
      });
    });
  });
}

async function checkCircuitUrls() {
  console.log('🔍 CHECKING CIRCUIT URLs');
  console.log('=' .repeat(50));
  
  const baseUrl = 'https://eswjamjanympzqopbqyt.supabase.co/storage/v1/object/public';
  
  const circuits = [
    'quiz_threshold_check.arcis',
    'quiz_evaluation.arcis', 
    'survey_analytics.arcis',
    'analytics_computation.arcis'
  ];
  
  const urlVariants = [
    // With space (URL encoded)
    `${baseUrl}/arcium%20circuits`,
    // With hyphen
    `${baseUrl}/arcium-circuits`,
    // With underscore
    `${baseUrl}/arcium_circuits`,
    // Direct in public folder
    `${baseUrl}`,
    // In circuits folder
    `${baseUrl}/circuits`
  ];
  
  console.log('\n📋 Testing different URL patterns...\n');
  
  for (const variant of urlVariants) {
    console.log(`🔗 Testing base: ${variant}`);
    
    for (const circuit of circuits) {
      const url = `${variant}/${circuit}`;
      const result = await checkUrl(url);
      
      const status = result.accessible ? '✅ ACCESSIBLE' : '❌ NOT ACCESSIBLE';
      console.log(`   ${circuit}: ${status}`);
      
      if (result.accessible) {
        console.log(`      Status: ${result.status}`);
        console.log(`      Content-Type: ${result.contentType}`);
        console.log(`      Content-Length: ${result.contentLength}`);
        console.log(`      URL: ${url}`);
      } else {
        console.log(`      Error: ${result.error || result.status}`);
      }
    }
    console.log('');
  }
  
  console.log('\n📊 SUMMARY');
  console.log('=' .repeat(20));
  console.log('Look for URLs that return status 200 and have content.');
  console.log('These are the correct URLs to use in your Rust code.');
}

checkCircuitUrls().catch(console.error);
