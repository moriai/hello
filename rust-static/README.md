# How to build

## Standard targets (self hosting / cross building on macOS host)

Tested on
+ x86_64-apple-darwin
+ x86_64-unknown-linux-gnu

```console
$ cargo build [--target target-name] [--release]
```

On a macOS host:
```toml:~/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=x86_64-elf-gcc"]
```

## Other targets (cross building)

Tested on x86_64-apple-darwin for
+ x86_64-apple-darwin-nostd.json
+ x86_64-unknown-linux-nostd.json

```console
$ cargo build -Z build-std=core,alloc --target target-name.json [--release]
```

On a macOS host:
```toml:~/.cargo/config.toml
[target.x86_64-unknown-linux-nostd]
rustflags = ["-C", "linker=x86_64-elf-gcc"]
```

See [cargo-xbuild README.md](https://github.com/rust-osdev/cargo-xbuild/blob/master/README.md).
