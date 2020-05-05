TARGETS = target/dos/release/hello.com
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/dos/release/hello.com:
	$(CARGO) xbuild --release

clean:
	$(CARGO) clean
