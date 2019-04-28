CFLAGS = -Oz
TARGETS = hello hello.stripped
SIZE = ../size.pl

all: build

build: $(TARGETS)

measure: $(TARGETS)
	@$(SIZE) $^

hello: hello.o

hello.stripped: hello
	strip -o $@ $^

clean:
	-rm -f $(TARGETS) *.o
