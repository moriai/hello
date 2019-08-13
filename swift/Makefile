SWIFTC = swiftc
SWIFTCFLAGS = -Osize
TARGETS = hello hello.stripped
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

hello: hello.swift
	$(SWIFTC) $(SWIFTCFLAGS) $^

hello.stripped: hello
	strip -o $@ $^

clean:
	-rm -f $(TARGETS)
