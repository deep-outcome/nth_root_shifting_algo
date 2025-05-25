### SHIFTING NTH ROOT ALGORITHM STUDY
- Overflows are unchecked.
- Tests must run with `--release` to allow harmless overflows.
- See [PAPER](./paper.pdf) for analysis.
- When best guess overflows, decrements test (β =beta), i.e. iterates reversely to find match.
- Integer precision only.


```rust
assert_eq!(Some(3), root(13, 33_554_431));
assert_eq!(Some(5560), root(2, 30_913_600));
```
