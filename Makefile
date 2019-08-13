TARGETS = c nasm swift $(CARGO_TARGETS)
CARGO_TARGETS = rust rust-dynamic rust-static
MAKE = make

.PHONY: all clean $(TARGETS)

all: build

build: ACTION = build

clean: ACTION = clean

measure: ACTION = measure

build measure clean: $(TARGETS)

distclean: clean
	-for d in $(CARGO_TARGETS); do rm -f $$d/Cargo.lock; done

$(TARGETS):
	(cd $@; $(MAKE) $(ACTION))
