"use strict";

// postinstall: warn (do not fail) if no per-platform binary package was
// installed for this host. npm skips optionalDependencies that fail or do
// not match os/cpu, so a missing one is a soft problem we surface early.

const PLATFORM_PACKAGES = {
  "darwin arm64": "@favcrm/cli-darwin-arm64",
  "darwin x64": "@favcrm/cli-darwin-x64",
  "linux arm64": "@favcrm/cli-linux-arm64",
  "linux x64": "@favcrm/cli-linux-x64",
  "win32 x64": "@favcrm/cli-win32-x64",
};

const key = `${process.platform} ${process.arch}`;
const pkg = PLATFORM_PACKAGES[key];

if (!pkg) {
  console.warn(
    `favcrm: no prebuilt binary for ${key}. ` +
      `See https://github.com/favcrm/cli/releases`,
  );
} else {
  const exe = process.platform === "win32" ? "favcrm.exe" : "favcrm";
  try {
    require.resolve(`${pkg}/bin/${exe}`);
  } catch {
    console.warn(
      `favcrm: ${pkg} was not installed. If the CLI fails to run, ` +
        `reinstall with optional dependencies enabled.`,
    );
  }
}
