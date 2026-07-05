#!/usr/bin/env node
'use strict';

const { execFileSync } = require('child_process');
const path = require('path');
const fs = require('fs');

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  const names = {
    'win32-x64': 'miskin-x86_64-pc-windows-msvc.exe',
    'linux-x64': 'miskin-x86_64-unknown-linux-gnu',
    'linux-arm64': 'miskin-aarch64-unknown-linux-gnu',
    'darwin-x64': 'miskin-x86_64-apple-darwin',
    'darwin-arm64': 'miskin-aarch64-apple-darwin',
  };

  const key = `${platform}-${arch}`;
  const name = names[key];

  if (!name) {
    console.error(`Unsupported platform: ${key}`);
    console.error('Install from source: cargo install --git https://github.com/sonyarianto/miskin');
    process.exit(1);
  }

  return path.join(__dirname, name);
}

function run() {
  const binary = getBinaryPath();

  if (!fs.existsSync(binary)) {
    console.error('Miskin binary not found. Run: node install.js');
    process.exit(1);
  }

  const args = process.argv.slice(2);
  try {
    execFileSync(binary, args, { stdio: 'inherit' });
  } catch (e) {
    process.exit(e.status || 1);
  }
}

run();
