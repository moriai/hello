ARCH ?= $(shell ../arch.sh)
UNAME ?= $(shell uname)
ifeq ($(UNAME),Darwin)
TARGET_ARCH = $(ARCH)-apple-darwin-nostd
else
TARGET_ARCH = $(ARCH)-unknown-linux-nostd
endif

TARGETS = target/$(TARGET_ARCH)/release/hello target/$(TARGET_ARCH)/release/hello.stripped
CARGO = cargo
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

target/$(TARGET_ARCH)/release/hello:
	$(CARGO) build --release --target=$(TARGET_ARCH).json

target/$(TARGET_ARCH)/release/hello.stripped: target/$(TARGET_ARCH)/release/hello
	strip -o $@ $^

clean:
	$(CARGO) clean
