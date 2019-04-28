TARGETS = target/release/hello target/release/hello.stripped
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/release/hello:
	$(CARGO) build --release

target/release/hello.stripped: target/release/hello
	strip -o $@ $^

clean:
	$(CARGO) clean
