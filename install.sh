#!/bin/sh
# FavCRM CLI installer.
#
#   curl -fsSL https://favcrm.io/install.sh | sh
#
# Downloads the prebuilt `favcrm` binary for your platform from GitHub
# Releases, verifies its sha256, and installs it.
#
# Environment overrides:
#   FAVCRM_VERSION       tag to install (e.g. v0.1.4); default: latest release
#   FAVCRM_INSTALL_DIR   install directory; default: $HOME/.local/bin

set -eu

REPO="favcrm/cli"
BIN="favcrm"

info() { printf '%s\n' "$*"; }
err() { printf 'error: %s\n' "$*" >&2; exit 1; }

# --- detect platform --------------------------------------------------------
os=$(uname -s)
arch=$(uname -m)

case "$os" in
  Darwin) os_part="apple-darwin" ;;
  Linux)  os_part="unknown-linux-gnu" ;;
  *) err "unsupported OS '$os'. See https://github.com/$REPO/releases" ;;
esac

case "$arch" in
  x86_64 | amd64)  arch_part="x86_64" ;;
  arm64 | aarch64) arch_part="aarch64" ;;
  *) err "unsupported architecture '$arch'. See https://github.com/$REPO/releases" ;;
esac

target="${arch_part}-${os_part}"

if [ "$target" = "x86_64-apple-darwin" ]; then
  err "Intel Macs are not supported — favcrm ships Apple Silicon (arm64) binaries only."
fi

# --- pick a downloader ------------------------------------------------------
if command -v curl >/dev/null 2>&1; then
  dl() { curl -fsSL "$1" -o "$2"; }
  fetch() { curl -fsSL "$1"; }
elif command -v wget >/dev/null 2>&1; then
  dl() { wget -qO "$2" "$1"; }
  fetch() { wget -qO - "$1"; }
else
  err "need curl or wget installed"
fi

# --- resolve version --------------------------------------------------------
tag="${FAVCRM_VERSION:-}"
if [ -z "$tag" ]; then
  tag=$(fetch "https://api.github.com/repos/$REPO/releases/latest" \
    | grep '"tag_name"' \
    | head -n1 \
    | sed -E 's/.*"tag_name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
fi
[ -n "$tag" ] || err "could not resolve latest release tag"

# --- download ---------------------------------------------------------------
asset="${BIN}-${tag}-${target}.tar.gz"
base="https://github.com/$REPO/releases/download/$tag"

workdir=$(mktemp -d)
trap 'rm -rf "$workdir"' EXIT INT TERM

info "Downloading $BIN $tag ($target)..."
dl "$base/$asset" "$workdir/$asset" || err "download failed: $base/$asset"

# --- verify checksum --------------------------------------------------------
if dl "$base/$asset.sha256" "$workdir/$asset.sha256" 2>/dev/null; then
  if command -v shasum >/dev/null 2>&1; then
    (cd "$workdir" && shasum -a 256 -c "$asset.sha256" >/dev/null) \
      || err "checksum verification failed"
  elif command -v sha256sum >/dev/null 2>&1; then
    (cd "$workdir" && sha256sum -c "$asset.sha256" >/dev/null) \
      || err "checksum verification failed"
  else
    info "warning: no shasum/sha256sum found, skipping checksum verification"
  fi
else
  info "warning: no checksum published for $tag, skipping verification"
fi

# --- extract & install ------------------------------------------------------
tar -xzf "$workdir/$asset" -C "$workdir"
src="$workdir/${BIN}-${tag}-${target}/${BIN}"
[ -f "$src" ] || err "binary not found in archive"

install_dir="${FAVCRM_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$install_dir"
install -m 0755 "$src" "$install_dir/$BIN" 2>/dev/null \
  || { cp "$src" "$install_dir/$BIN" && chmod 0755 "$install_dir/$BIN"; }

info "Installed $BIN to $install_dir/$BIN"

# --- PATH hint --------------------------------------------------------------
case ":$PATH:" in
  *":$install_dir:"*) ;;
  *) info ""
     info "$install_dir is not on your PATH. Add it:"
     info "  export PATH=\"$install_dir:\$PATH\"" ;;
esac

info ""
"$install_dir/$BIN" --version
