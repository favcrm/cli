## Summary

<!-- What changed and why? -->

## Type

- [ ] CLI behavior
- [ ] Output formatting
- [ ] Auth/config
- [ ] Release/install
- [ ] Documentation
- [ ] Repository maintenance

## Review checklist

- [ ] I linked the relevant issue or explained why there is none.
- [ ] I did not commit API keys, customer data, merchant data, or local config.
- [ ] I updated README/help text for user-facing behavior changes.
- [ ] I considered `--json` output compatibility.
- [ ] I considered whether this changes destructive/external-world operations.

## Test evidence

```text
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

<!-- Paste output or explain why a check was not run. -->

