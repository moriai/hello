TARGETS = target/wasm32-wasi/release/hello.wasm \
          target/wasm32-wasi/release/hello-stripped.wasm
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/wasm32-wasi/release/hello.wasm:
	$(CARGO) build --release

target/wasm32-wasi/release/hello-stripped.wasm: target/wasm32-wasi/release/hello.wasm
	wasm-snip -o $@ $^

clean:
	$(CARGO) clean
