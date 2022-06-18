# How to build

## Standard targets (self hosting / cross building on macOS host)

Tested on
+ x86_64-apple-darwin
+ x86_64-unknown-linux-gnu
+ aarch64-apple-darwin (dynamically linked to libSystem.dylib, because aarch64 static binaries are disallowed to exec except on development kernels.)

```console
$ cargo build [--target target-name] [--release]
```

On a macOS host:
```toml:~/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-elf-gcc"
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
linker = "x86_64-elf-gcc"
```

See [cargo-xbuild README.md](https://github.com/rust-osdev/cargo-xbuild/blob/master/README.md).
