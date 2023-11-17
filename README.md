# rust-file-sequence-renamer-script

[![Rust](https://github.com/shinshin86/rust-file-sequence-renamer-script/actions/workflows/rust.yml/badge.svg)](https://github.com/shinshin86/rust-file-sequence-renamer-script/actions/workflows/rust.yml)

The script renames the files in the target directory with sequential numbers starting from 1.

I created this to try running it as a script in Rust.

```sh
cargo +nightly -Zscript src/main.rs <path_to_directory>
```

## Test

```
cargo test
```