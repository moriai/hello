# How to build

+ Default target (self hosting)
    - Tested on
        - x86_64-apple-darwin
        - x86_64-unknown-linux-gnu

```
$ cargo build [--target target-name] [--release]
```

+ Other targets (cross building)
    - Tested for
        - x86_64-apple-darwin-nostd.json
        - x86_64-unknown-linux-nostd.json

```
$ cargo build -Z build-std=core,alloc --target target-name [--release]
```

See [cargo-xbuild README.md](https://github.com/rust-osdev/cargo-xbuild/blob/master/README.md).
