#!/usr/bin/env node

import { execSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

const bumpType = process.argv[2] ?? "patch";

if (!["patch", "minor", "major"].includes(bumpType)) {
  console.error("Usage: npm run release [-- patch|minor|major]");
  process.exit(1);
}

function run(command, options = {}) {
  execSync(command, { stdio: "inherit", ...options });
}

function readVersion() {
  const pkg = JSON.parse(readFileSync("package.json", "utf8"));
  return pkg.version;
}

function bumpSemver(version, type) {
  const [major, minor, patch] = version.split(".").map(Number);

  if (type === "major") {
    return `${major + 1}.0.0`;
  }

  if (type === "minor") {
    return `${major}.${minor + 1}.0`;
  }

  return `${major}.${minor}.${patch + 1}`;
}

function setVersion(version) {
  const pkg = JSON.parse(readFileSync("package.json", "utf8"));
  pkg.version = version;
  writeFileSync("package.json", `${JSON.stringify(pkg, null, 2)}\n`);

  const cargoPath = "src-tauri/Cargo.toml";
  const cargo = readFileSync(cargoPath, "utf8").replace(
    /^version = ".*"$/m,
    `version = "${version}"`,
  );
  writeFileSync(cargoPath, cargo);

  const tauriPath = "src-tauri/tauri.conf.json";
  const tauri = JSON.parse(readFileSync(tauriPath, "utf8"));
  tauri.version = version;
  writeFileSync(tauriPath, `${JSON.stringify(tauri, null, 2)}\n`);
}

function tagExists(tag) {
  try {
    execSync(`git rev-parse ${tag}`, { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

const currentVersion = readVersion();
const nextVersion = bumpSemver(currentVersion, bumpType);
const tag = `v${nextVersion}`;

if (tagExists(tag)) {
  console.error(`Tag ${tag} already exists.`);
  process.exit(1);
}

console.log(`Checking build before ${currentVersion} -> ${nextVersion} (${tag})...`);
run("npm run build");
run("cargo test", { cwd: "src-tauri" });

console.log(`Preparing CHANGELOG.md for ${nextVersion}...`);
run(`node scripts/update-changelog.mjs ${nextVersion}`);

console.log(`Setting version to ${nextVersion}...`);
setVersion(nextVersion);
run("cargo check", { cwd: "src-tauri" });

run("git add -A");

try {
  run(`git diff --cached --quiet`);
  console.error("Nothing to commit after version bump.");
  process.exit(1);
} catch {
  // staged changes exist
}

run(`git commit -m "chore(release): ${tag}"`);
run(`git tag ${tag}`);
run("git push origin main");
run(`git push origin ${tag}`);

console.log("");
console.log(`Release ${tag} pushed.`);
console.log("GitHub Actions will build the .deb and publish the release.");
