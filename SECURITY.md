# Security Policy

## Purpose and authorised use

Bankai is an educational offensive-security project. It contains working attack
techniques (reverse shell, keylogger, DLL hijacking, buffer-overflow exploitation,
port scanning, phishing generation) and an intentionally vulnerable service.

Use these tools **only** on systems you own or for which you hold explicit written
authorisation, and **only** on an isolated network. Unauthorised access to computer
systems is illegal in most jurisdictions and can carry criminal penalties. The
authors accept no liability for misuse.

Do not run `VulnerableCode/VulnServerRevOver.py` or the Flask collector on an
internet-facing host — both are insecure by design.

## Reporting a vulnerability

This is a learning project, not production software, so it has no supported
release or patch process. If you find a flaw that goes beyond the intentional
vulnerabilities (for example a problem that could harm a user running the tools
as documented), open a GitHub issue describing it, or contact the maintainers
privately before disclosing publicly.
