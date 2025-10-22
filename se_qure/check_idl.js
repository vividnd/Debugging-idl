const fs = require('fs');
const idl = JSON.parse(fs.readFileSync('target/idl/se_qure.json', 'utf8'));

console.log('IDL top-level sections:');
console.log('address:', !!idl.address);
console.log('metadata:', !!idl.metadata);
console.log('instructions:', !!idl.instructions);
console.log('accounts:', !!idl.accounts);
console.log('types:', !!idl.types);
console.log('events:', !!idl.events);
console.log('errors:', !!idl.errors);
console.log('state:', !!idl.state);
console.log('constants:', !!idl.constants);

if (idl.accounts) {
  console.log('Accounts count:', idl.accounts.length);
  console.log('First few accounts:');
  idl.accounts.slice(0, 3).forEach(acc => {
    console.log('  -', acc.name, acc.type);
  });
} else {
  console.log('❌ No top-level accounts section found');
}






