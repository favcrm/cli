# Security Policy

## Reporting a vulnerability

Please do not open a public GitHub issue for suspected vulnerabilities.

Report security concerns privately by emailing `security@favcrm.io`. Include:

- Affected CLI version and operating system.
- Impact and exploitability.
- Steps to reproduce.
- Whether a token, customer record, merchant record, or local config file may have been exposed.
- Logs or timestamps if the command reached the FavCRM API.

We aim to acknowledge reports within 3 business days and will coordinate fixes and disclosure based on severity.

## Scope

In scope:

- API key leakage through logs, errors, config files, shell output, or release artifacts.
- Unsafe handling of local configuration under `~/.config/favcrm`.
- Auth or transport bugs that send credentials to the wrong host.
- CLI behavior that performs destructive or external-world actions without clear user intent.
- Vulnerabilities in public release packaging.

Out of scope:

- Account support, billing, and feature requests.
- Reports that require access to data you do not own.
- Automated scans without a concrete exploit path.

