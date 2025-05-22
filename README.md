### SHIFTING NTH ROOT ALGORITHM STUDY
- Overflows are unchecked.
- Tests must run with `--release` to allow harmless overflows.
- See [PAPER](./paper.pdf) for analysis.
- When best guess overflows, decrements test (β, beta), i.e. iterates reversely to find match.
