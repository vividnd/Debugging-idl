#!/bin/bash

# Script to push both MRE files to their respective GitHub repos
# Run this from your Mac Terminal (not Cursor's broken terminal)

set -e  # Exit on any error

echo "🚀 Starting MRE push process..."

# Save the original directory
ORIGINAL_DIR="/Users/progzzz/Desktop/SeQure copy 3"

# Create clean temp directory for first MRE
echo "📦 Processing MRE_Arcium_Array_Arguments.md..."
mkdir -p /tmp/mre1
cd /tmp/mre1
git init
cp "$ORIGINAL_DIR/MRE_Arcium_Array_Arguments.md" .
git add MRE_Arcium_Array_Arguments.md
git commit -m "Add array arguments MRE"
git branch -m main
git remote add origin https://github.com/vividnd/MRE_Arcium_Array_Arguments.md.git
git push -f origin main
echo "✅ MRE_Arcium_Array_Arguments.md pushed successfully!"

# Create clean temp directory for second MRE
echo "📦 Processing MRE_Transaction_Size_And_Double_Signing.md..."
mkdir -p /tmp/mre2
cd /tmp/mre2
git init
cp "$ORIGINAL_DIR/MRE_Transaction_Size_And_Double_Signing.md" .
git add MRE_Transaction_Size_And_Double_Signing.md
git commit -m "Add transaction size and double signing MRE"
git branch -m main
git remote add origin https://github.com/vividnd/MRE_Transaction_Size_And_Double_Signing.md.git
git push -f origin main
echo "✅ MRE_Transaction_Size_And_Double_Signing.md pushed successfully!"

# Cleanup
echo "🧹 Cleaning up temporary directories..."
cd ~
rm -rf /tmp/mre1 /tmp/mre2

echo ""
echo "🎉 All done! Both MREs have been pushed to GitHub."
echo ""
echo "Repository URLs:"
echo "  - https://github.com/vividnd/MRE_Arcium_Array_Arguments.md"
echo "  - https://github.com/vividnd/MRE_Transaction_Size_And_Double_Signing.md"

