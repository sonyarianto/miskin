'use strict';

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const VERSION = 'v0.1.0';
const REPO = 'sonyarianto/miskin';

const PLATFORMS = {
  'win32-x64':   { target: 'x86_64-pc-windows-msvc',   ext: '.zip' },
  'linux-x64':   { target: 'x86_64-unknown-linux-gnu',  ext: '.tar.gz' },
  'linux-arm64': { target: 'aarch64-unknown-linux-gnu', ext: '.tar.gz' },
  'darwin-x64':  { target: 'x86_64-apple-darwin',       ext: '.tar.gz' },
  'darwin-arm64':{ target: 'aarch64-apple-darwin',      ext: '.tar.gz' },
};

function getPlatform() {
  const key = `${process.platform}-${process.arch}`;
  const info = PLATFORMS[key];
  if (!info) throw new Error(`Unsupported platform: ${key}`);
  return info;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (res) => {
      if (res.statusCode === 302 || res.statusCode === 301) {
        https.get(res.headers.location, (r) => {
          r.pipe(file).on('finish', resolve).on('error', reject);
        });
      } else if (res.statusCode === 200) {
        res.pipe(file).on('finish', resolve).on('error', reject);
      } else {
        reject(new Error(`HTTP ${res.statusCode}`));
      }
    }).on('error', reject);
  });
}

function extract(archivePath, dir) {
  if (archivePath.endsWith('.tar.gz')) {
    execSync(`tar xzf "${archivePath}" -C "${dir}"`, { stdio: 'pipe' });
  } else if (archivePath.endsWith('.zip')) {
    execSync(`unzip -o "${archivePath}" -d "${dir}"`, { stdio: 'pipe' });
  }
}

async function install() {
  const platform = getPlatform();
  const archiveName = `miskin-${platform.target}${platform.ext}`;
  const url = `https://github.com/${REPO}/releases/download/${VERSION}/${archiveName}`;

  console.log(`Miskin ${VERSION} — downloading for ${platform.target}...`);

  const tmpDir = path.join(__dirname, '.tmp');
  fs.mkdirSync(tmpDir, { recursive: true });

  const archivePath = path.join(tmpDir, archiveName);
  await downloadFile(url, archivePath);

  extract(archivePath, tmpDir);

  const binName = process.platform === 'win32' ? 'miskin.exe' : 'miskin';
  const srcPath = path.join(tmpDir, 'miskin');
  const destPath = path.join(__dirname, binName);

  if (fs.existsSync(srcPath)) {
    fs.copyFileSync(srcPath, destPath);
  } else if (fs.existsSync(path.join(tmpDir, 'miskin.exe'))) {
    fs.copyFileSync(path.join(tmpDir, 'miskin.exe'), destPath);
  }

  if (process.platform !== 'win32') {
    fs.chmodSync(destPath, 0o755);
  }

  fs.rmSync(tmpDir, { recursive: true });
  console.log('miskin installed successfully!');
}

install().catch((err) => {
  console.error('Install failed:', err.message);
  console.error('Install from source: cargo install --git https://github.com/sonyarianto/miskin');
  process.exit(1);
});
