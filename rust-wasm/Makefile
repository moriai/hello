TARGETS = pkg/hello_bg.wasm
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

pkg/hello_bg.wasm:
	wasm-pack build --target web
	cp src/index.html pkg

clean:
	$(CARGO) clean
	rm -fr pkg
