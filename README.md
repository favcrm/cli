# favcrm CLI

Talk to [FavCRM](https://favcrm.io) from your terminal. Wraps the public MCP server (`https://api.favcrm.io/mcp`) as ergonomic subcommands so humans get the same surface that AI agents do.

## Install

### From source

```bash
git clone https://github.com/favcrm/cli ~/Project/favcrm/cli
cd ~/Project/favcrm/cli
cargo install --path .
```

### Pre-built (planned)

```bash
brew install favcrm/tap/favcrm        # planned
curl -fsSL favcrm.io/install.sh | sh   # planned
```

## Auth

Three ways to provide your `fav_mcp_*` API key (in priority order):

```bash
favcrm --api-key fav_mcp_...                 # CLI flag
export FAVCRM_API_KEY=fav_mcp_...            # env
favcrm login fav_mcp_...                     # writes ~/.config/favcrm/config.toml
```

Get a key at `Settings → MCP Keys` in the merchant portal.

## Quick start

```bash
favcrm whoami                                # active user / company
favcrm orgs list
favcrm orgs switch <company-id>

favcrm members search alice --limit 5
favcrm members search --inactive-days 90
favcrm members get <account-id>

favcrm bookings list --status confirmed --limit 10
favcrm bookings stats
favcrm bookings cancel <booking-id>

favcrm invoices list --status overdue
favcrm invoices send <invoice-id>

favcrm dashboard                             # headline stats

favcrm --json bookings list                  # raw JSON for jq
```

## Escape hatch

Any of the 154 registered MCP tools can be called directly:

```bash
favcrm tool list_campaigns '{"limit":5}'
favcrm tool generate_image '{"prompt":"sunset","model":"gemini-2.5-flash-image"}'
```

See the full catalog at `https://api.favcrm.io/mcp` (JSON-RPC `tools/list`).

Public agent workflow skills for using this CLI live in [`favcrm/mcp/skills`](https://github.com/favcrm/mcp/tree/main/skills). The CLI stays the execution layer; the MCP repo is the public skill catalog.

## Output

Default: human-friendly tables. Use `--json` for machine-parseable JSON (pipes well into `jq`).

## How it works

Thin Rust client over the existing FavCRM MCP server. No business logic in the CLI — all gating (per-tool scope, per-merchant module access, rate limits) is enforced server-side. Token = same `fav_mcp_*` key your agents use.

## License

MIT
