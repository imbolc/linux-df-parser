[![License](https://img.shields.io/crates/l/linux-df-parser.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/linux-df-parser.svg)](https://crates.io/crates/linux-df-parser)
[![Docs.rs](https://docs.rs/linux-df-parser/badge.svg)](https://docs.rs/linux-df-parser)

<!-- cargo-sync-readme start -->

linux-df-parser
===============

A simple parser for the Linux `df` command. To get numbers in bytes, call `df` with `-B1`
argument: `/bin/df -B1`

Usage
-----
```rust
let s = r#"
    df: /run/user/1000/doc: Operation not permitted
    Filesystem                 1B-blocks         Used    Available Use% Mounted on
    udev                     12294803456            0  12294803456   0% /dev
    /dev/nvme0n1p2             493201408    121312256    346304512  26% /boot
"#.trim();
let df = Df::from(s);
assert_eq!(df.get_by_filesystem("/dev/nvme0n1p2").unwrap().used, 121312256);
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-readme-sync][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme-sync]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
