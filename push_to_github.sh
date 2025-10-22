#!/bin/bash

# Script to push debugging files to GitHub repository
# Repository: https://github.com/vividnd/Debugging-idl

echo "🚀 Pushing Arcium IDL debugging files to GitHub..."

# Initialize git if not already done
if [ ! -d ".git" ]; then
    echo "📁 Initializing git repository..."
    git init
    git remote add origin https://github.com/vividnd/Debugging-idl.git
fi

# Add all the debugging files
echo "📄 Adding debugging files..."
git add README.md
git add current_broken_idl.json
git add old_working_idl.json
git add arcium_config.toml
git add cargo_lock.txt
git add package_json.txt
git add rust_program_sample.rs
git add useAnchorProgram.ts
git add encrypted_ixs_cargo.toml

# Commit the files
echo "💾 Committing files..."
git commit -m "Add Arcium IDL debugging files

- Current broken IDL (missing account type definitions)
- Old working IDL (same issue)
- Arcium configuration files
- Version information (Cargo.lock, package.json)
- Rust program sample
- Frontend hook usage
- Encrypted instructions config

Issue: arcium build generates incomplete IDLs that break BorshCoder type registration"

# Push to GitHub
echo "🌐 Pushing to GitHub..."
git branch -M main
git push -u origin main

echo "✅ Successfully pushed debugging files to GitHub!"
echo "🔗 Repository: https://github.com/vividnd/Debugging-idl"

