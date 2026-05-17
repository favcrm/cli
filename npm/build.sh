#!/usr/bin/env bash
# Build publishable npm packages from downloaded release archives.
#
# Usage: build.sh <tag> <archive-dir> <out-dir>
#   <tag>          release tag, e.g. v0.1.4
#   <archive-dir>  dir holding favcrm-<tag>-<target>.tar.gz / .zip
#   <out-dir>      output dir; gets one subdir per package
#
# Produces, under <out-dir>:
#   cli/                       -> the "favcrm" launcher package
#   cli-<platform>-<arch>/     -> five "@favcrm/cli-*" binary packages
set -euo pipefail

tag="${1:?tag required}"
archive_dir="${2:?archive dir required}"
out_dir="${3:?out dir required}"
here="$(cd "$(dirname "$0")" && pwd)"
version="${tag#v}"

mkdir -p "$out_dir"

# rust target -> "npm-name-suffix os cpu exe"
targets=(
  "aarch64-apple-darwin|darwin-arm64|darwin|arm64|favcrm"
  "x86_64-apple-darwin|darwin-x64|darwin|x64|favcrm"
  "aarch64-unknown-linux-gnu|linux-arm64|linux|arm64|favcrm"
  "x86_64-unknown-linux-gnu|linux-x64|linux|x64|favcrm"
  "x86_64-pc-windows-msvc|win32-x64|win32|x64|favcrm.exe"
)

for row in "${targets[@]}"; do
  IFS='|' read -r target suffix os cpu exe <<<"$row"

  archive_base="favcrm-$tag-$target"
  pkg_dir="$out_dir/cli-$suffix"
  mkdir -p "$pkg_dir/bin"

  # extract the binary from the release archive
  work="$(mktemp -d)"
  if [[ "$os" == "win32" ]]; then
    unzip -q "$archive_dir/$archive_base.zip" -d "$work"
  else
    tar -xzf "$archive_dir/$archive_base.tar.gz" -C "$work"
  fi
  cp "$work/$archive_base/$exe" "$pkg_dir/bin/$exe"
  chmod +x "$pkg_dir/bin/$exe"
  rm -rf "$work"

  # render package.json
  sed \
    -e "s|__NAME__|@favcrm/cli-$suffix|g" \
    -e "s|__VERSION__|$version|g" \
    -e "s|__OS__|$os|g" \
    -e "s|__CPU__|$cpu|g" \
    "$here/platforms/template/package.json" >"$pkg_dir/package.json"
done

# the main launcher package
cp -r "$here/cli" "$out_dir/cli"
node -e '
  const fs = require("fs");
  const p = process.argv[1];
  const v = process.argv[2];
  const j = JSON.parse(fs.readFileSync(p, "utf8"));
  j.version = v;
  for (const k of Object.keys(j.optionalDependencies || {})) {
    j.optionalDependencies[k] = v;
  }
  fs.writeFileSync(p, JSON.stringify(j, null, 2) + "\n");
' "$out_dir/cli/package.json" "$version"

echo "built npm packages for $tag in $out_dir"
