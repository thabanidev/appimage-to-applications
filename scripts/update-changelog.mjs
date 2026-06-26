#!/usr/bin/env node

import { readFileSync, writeFileSync } from "node:fs";

const version = process.argv[2];
const date = process.argv[3] ?? new Date().toISOString().slice(0, 10);

if (!version || !/^\d+\.\d+\.\d+$/.test(version)) {
  console.error("Usage: node scripts/update-changelog.mjs 0.1.3 [YYYY-MM-DD]");
  process.exit(1);
}

const path = "CHANGELOG.md";
const changelog = readFileSync(path, "utf8");
const unreleasedHeader = "## [Unreleased]";
const unreleasedStart = changelog.indexOf(unreleasedHeader);

if (unreleasedStart === -1) {
  console.error("CHANGELOG.md is missing an [Unreleased] section.");
  process.exit(1);
}

const afterUnreleased = changelog.indexOf("\n", unreleasedStart) + 1;
const nextVersionHeader = changelog.indexOf("\n## [", afterUnreleased);

if (nextVersionHeader === -1) {
  console.error("CHANGELOG.md is missing a previous version section.");
  process.exit(1);
}

const unreleasedBody = changelog.slice(afterUnreleased, nextVersionHeader).trim();

if (!unreleasedBody) {
  console.error(
    "CHANGELOG.md [Unreleased] is empty. Add release notes before running npm run release.",
  );
  process.exit(1);
}

const newSection = `## [Unreleased]

## [${version}] - ${date}

${unreleasedBody}
`;

const updated =
  changelog.slice(0, unreleasedStart) +
  newSection +
  changelog.slice(nextVersionHeader + 1);

writeFileSync(path, updated);
console.log(`Prepared CHANGELOG.md for v${version}.`);
