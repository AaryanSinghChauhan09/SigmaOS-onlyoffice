# SigmaOS-onlyoffice

SigmaOS integration component for **ONLYOFFICE editors**.

## Commands

```bash
sigmaos-onlyoffice install   # install package + deploy defaults
sigmaos-onlyoffice deploy    # copy config templates to $HOME
sigmaos-onlyoffice status    # binary/package/config report
sigmaos-onlyoffice reset     # remove deployed config
sigmaos-onlyoffice open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
