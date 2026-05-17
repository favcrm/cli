# favcrm CLI

Talk to [FavCRM](https://favcrm.io) from your terminal. Wraps the public MCP server (`https://api.favcrm.io/mcp`) as ergonomic subcommands so humans get the same surface that AI agents do.

## Install

### From source

```bash
git clone https://github.com/favcrm/cli ~/Project/favcrm/cli
cd ~/Project/favcrm/cli
cargo install --path .
```

### Pre-built

```bash
brew install favcrm/tap/favcrm         # Homebrew (macOS / Linux)
curl -fsSL favcrm.io/install.sh | sh   # curl
npm install -g @favcrm/cli             # npm
```

The curl script honours `FAVCRM_VERSION` and `FAVCRM_INSTALL_DIR` overrides.

## Auth

Three ways to provide your `fav_mcp_*` API key (in priority order):

```bash
favcrm --api-key fav_mcp_...                 # CLI flag
export FAVCRM_API_KEY=fav_mcp_...            # env
favcrm login fav_mcp_...                     # writes ~/.config/favcrm/config.toml
```

Get a key at `Settings → MCP Keys` in the merchant portal.

You can also register without a key:

```bash
favcrm signup request --email owner@example.com --organisation-name "Ada Studio"
favcrm signup verify --request-id <request-id> --code <code>
```

## Quick start

```bash
favcrm whoami                                # active user / company
favcrm orgs list
favcrm orgs switch <company-id>

favcrm members search alice --limit 5
favcrm members search --inactive-days 90
favcrm members get <account-id>
favcrm members create "Ada Lovelace" --email ada@example.com --phone +15550001001
favcrm members create "Ada Member" --enroll-membership --tier-id <tier-id>

favcrm bookings list --status confirmed --limit 10
favcrm bookings stats
favcrm bookings cancel <booking-id>

favcrm invoices list --status overdue
favcrm invoices send <invoice-id>

favcrm plan status
favcrm plan check --tool create_account
favcrm plan options
favcrm plan upgrade --plan-code favcrm-lite --confirm
favcrm plan portal --confirm

favcrm team invite create --email teammate@example.com --role staff
favcrm team invite accept-request --token <invite-token>
favcrm team invite accept-verify --token <invite-token> --code <code>

favcrm whatsapp status
favcrm whatsapp connect --mode cloud-api

favcrm doctor                                # endpoint, auth, plan, channel checks

favcrm dashboard                             # headline stats

favcrm --json bookings list                  # raw JSON for jq
```

## Escape hatch

Any of the 165 registered MCP tools can be called directly:

```bash
favcrm tool query_favcrm_platform '{"query":"create a booking"}'
favcrm tool query_company_knowledge '{"query":"refund policy"}'
favcrm tool list_campaigns '{"limit":5}'
favcrm tool generate_image '{"prompt":"sunset","model":"gemini-2.5-flash-image"}'
favcrm tool report_agent_issue '{"title":"Missing MCP path","severity":"high","area":"mcp_tool_missing","expectedBehavior":"...","actualBehavior":"...","stepsTried":["..."],"aiAnalysis":"..."}'
```

See the full catalog at `https://api.favcrm.io/mcp` (JSON-RPC `tools/list`).

Public agent workflow skills for using this CLI live in [`favcrm/mcp/skills`](https://github.com/favcrm/mcp/tree/main/skills). The CLI stays the execution layer; the MCP repo is the public skill catalog.

## Output

Default: human-friendly tables. Use `--json` for machine-parseable JSON (pipes well into `jq`).

## How it works

Thin Rust client over the existing FavCRM MCP server. No business logic in the CLI — all gating (per-tool scope, per-merchant module access, plan quotas, billing links, rate limits) is enforced server-side. Token = same `fav_mcp_*` key your agents use.

## Contributing

Issues and PRs are welcome. See [`CONTRIBUTING.md`](./CONTRIBUTING.md) before opening a PR. Please report suspected vulnerabilities privately using [`SECURITY.md`](./SECURITY.md), not public issues.

## License

MIT
