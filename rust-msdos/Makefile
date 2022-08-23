TARGETS = target/dos/release/hello.com
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/dos/release/hello.com:
	$(CARGO) build --release

clean:
	$(CARGO) clean
