bench:
    cargo test
    cargo bench
    critcmp -g '(sort)' --list base
