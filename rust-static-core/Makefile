TARGET_ARCH=x86_64-apple-darwin-static
TARGETS =	target/$(TARGET_ARCH)/release/hello \
			target/$(TARGET_ARCH)/release/hello.stripped
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/$(TARGET_ARCH)/release/hello:
	$(CARGO) xbuild --release

target/$(TARGET_ARCH)/release/hello.stripped: target/$(TARGET_ARCH)/release/hello
	strip -o $@ $^

clean:
	$(CARGO) clean
