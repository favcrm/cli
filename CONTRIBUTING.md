# Contributing to favcrm CLI

The CLI is a thin Rust client over the FavCRM MCP API. Keep changes focused on command ergonomics, output formatting, installation, release packaging, and safe API transport behavior.

## Before opening an issue

1. Search existing issues.
2. Check `favcrm --help` and the README.
3. Redact API keys, customer data, merchant data, request IDs that expose private data, emails, and phone numbers.
4. For bugs, include CLI version, operating system, command, actual output, expected output, and UTC timestamp if the command reached the API.

Do not open public issues for suspected vulnerabilities. Follow `SECURITY.md`.

## Pull requests

Good PRs are small and include test evidence. If a change affects a command, update README examples or help text when needed.

Reviewers will pay particular attention to:

- Secret handling and config-file behavior.
- JSON output compatibility for scripts.
- Human-readable output stability.
- Auth headers and MCP envelope parsing.
- Commands that trigger destructive or external-world effects.

## Local checks

Run these before opening a PR:

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

For live smoke testing, use a non-production workspace when possible:

```bash
FAVCRM_API_KEY=fav_mcp_... ./smoke.sh
```

## Releasing

Pushing a `vX.Y.Z` tag triggers `.github/workflows/release.yml`, which builds
the binaries, publishes the GitHub Release, updates the Homebrew tap, and
publishes the npm packages. The tag version **must** match `version` in
`Cargo.toml` — the npm packages take their version from the tag.

