#!/usr/bin/env node
"use strict";

// Launcher for the `favcrm` CLI. The real binary ships in a per-platform
// optional dependency (@favcrm/cli-<platform>-<arch>); npm installs only the
// one matching the host. This shim resolves it and execs it.

const { execFileSync } = require("node:child_process");

const PLATFORM_PACKAGES = {
  "darwin arm64": "@favcrm/cli-darwin-arm64",
  "darwin x64": "@favcrm/cli-darwin-x64",
  "linux arm64": "@favcrm/cli-linux-arm64",
  "linux x64": "@favcrm/cli-linux-x64",
  "win32 x64": "@favcrm/cli-win32-x64",
};

function resolveBinary() {
  const key = `${process.platform} ${process.arch}`;
  const pkg = PLATFORM_PACKAGES[key];
  if (!pkg) {
    throw new Error(
      `favcrm: no prebuilt binary for ${key}. ` +
        `See https://github.com/favcrm/cli/releases`,
    );
  }
  const exe = process.platform === "win32" ? "favcrm.exe" : "favcrm";
  try {
    return require.resolve(`${pkg}/bin/${exe}`);
  } catch {
    throw new Error(
      `favcrm: the ${pkg} package is missing. ` +
        `Reinstall with optional dependencies enabled ` +
        `(npm install favcrm).`,
    );
  }
}

try {
  execFileSync(resolveBinary(), process.argv.slice(2), { stdio: "inherit" });
} catch (err) {
  if (typeof err.status === "number") {
    process.exit(err.status);
  }
  console.error(err.message || String(err));
  process.exit(1);
}
