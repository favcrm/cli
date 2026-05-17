#!/usr/bin/env bash
# Render the Homebrew formula from the template using release checksums.
#
# Usage: render.sh <tag> <sha256-dir>
#   <tag>         release tag, e.g. v0.1.4
#   <sha256-dir>  directory holding favcrm-<tag>-<target>.tar.gz.sha256 files
#
# Prints the rendered formula to stdout.
set -euo pipefail

tag="${1:?tag required}"
shadir="${2:?sha256 dir required}"
tmpl="$(dirname "$0")/favcrm.rb.tmpl"

repo="favcrm/cli"
base="https://github.com/$repo/releases/download/$tag"
version="${tag#v}"

# sha256 of favcrm-<tag>-<target>.tar.gz — first field of the sidecar file.
sha_for() {
  local target="$1"
  local file="$shadir/favcrm-$tag-$target.tar.gz.sha256"
  [[ -f "$file" ]] || { echo "missing $file" >&2; exit 1; }
  local hash
  hash="$(awk '{print $1}' "$file")"
  [[ -n "$hash" ]] || { echo "empty checksum in $file" >&2; exit 1; }
  echo "$hash"
}

url_for() { echo "$base/favcrm-$tag-$1.tar.gz"; }

sed \
  -e "s|__VERSION__|$version|g" \
  -e "s|__URL_DARWIN_ARM__|$(url_for aarch64-apple-darwin)|g" \
  -e "s|__SHA_DARWIN_ARM__|$(sha_for aarch64-apple-darwin)|g" \
  -e "s|__URL_LINUX_ARM__|$(url_for aarch64-unknown-linux-gnu)|g" \
  -e "s|__SHA_LINUX_ARM__|$(sha_for aarch64-unknown-linux-gnu)|g" \
  -e "s|__URL_LINUX_X64__|$(url_for x86_64-unknown-linux-gnu)|g" \
  -e "s|__SHA_LINUX_X64__|$(sha_for x86_64-unknown-linux-gnu)|g" \
  "$tmpl"
