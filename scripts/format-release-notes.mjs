#!/usr/bin/env node

import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";

const tag = process.argv[2];

if (!tag || !/^v\d+\.\d+\.\d+$/.test(tag)) {
  console.error("Usage: node scripts/format-release-notes.mjs v0.1.2");
  process.exit(1);
}

const version = tag.slice(1);
const changelog = readFileSync("CHANGELOG.md", "utf8");
const section = extractChangelogSection(changelog, version);

if (!section) {
  console.error(`No CHANGELOG section found for version ${version}.`);
  process.exit(1);
}

const previousTag = getPreviousTag(tag);
const compareUrl = previousTag
  ? `https://github.com/thabanidev/appimage-to-applications/compare/${previousTag}...${tag}`
  : `https://github.com/thabanidev/appimage-to-applications/releases/tag/${tag}`;

const { summary, body } = splitSection(section);
const debFile = `AppImage.to.Applications_${version}_amd64.deb`;

process.stdout.write(`Release **${version}** of AppImage to Applications for Linux.

${summary}

${body}

## Install

Download \`${debFile}\` below, then:

\`\`\`bash
sudo dpkg -i ${debFile}
sudo apt-get install -f
\`\`\`

Dependency: \`desktop-file-utils\` (installed automatically with the \`.deb\` when needed).

## Full changelog

${compareUrl}
`);

function extractChangelogSection(markdown, targetVersion) {
  const header = `## [${targetVersion}]`;
  const start = markdown.indexOf(header);

  if (start === -1) {
    return null;
  }

  const bodyStart = markdown.indexOf("\n", start) + 1;
  const rest = markdown.slice(bodyStart);
  const nextHeader = rest.search(/\n## \[/);

  if (nextHeader === -1) {
    return rest.trim();
  }

  return rest.slice(0, nextHeader).trim();
}

function splitSection(section) {
  const lines = section.split("\n");
  const firstHeadingIndex = lines.findIndex((line) => line.startsWith("### "));

  if (firstHeadingIndex === -1) {
    const summary = lines.find((line) => line.trim())?.trim() ?? "";
    return { summary, body: "" };
  }

  const summary = lines
    .slice(0, firstHeadingIndex)
    .map((line) => line.trim())
    .filter(Boolean)
    .join(" ");
  const body = lines.slice(firstHeadingIndex).join("\n").trim();

  return {
    summary: summary || "Update for AppImage to Applications.",
    body,
  };
}

function getPreviousTag(tag) {
  try {
    return execSync(`git describe --tags --abbrev=0 ${tag}^`, {
      encoding: "utf8",
    }).trim();
  } catch {
    return null;
  }
}
