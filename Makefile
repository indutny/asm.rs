RUSTC ?= rustc
RUSTFLAGS ?=

TEST_BINARY ?= ./run-tests

SRC ?=
SRC += src/asm.rs
SRC += src/ia32/base.rs
SRC += src/ia32/basic.rs
SRC += src/ia32/math.rs
SRC += src/ia32/branching.rs
SRC += src/ia32/fp.rs
SRC += src/x64/base.rs
SRC += src/x64/basic.rs
SRC += src/x64/math.rs
SRC += src/x64/branching.rs
SRC += src/x64/fp.rs

TEST_SRC ?=
TEST_SRC += test/common.rs
TEST_SRC += test/ia32.rs
TEST_SRC += test/x64.rs
TEST_SRC += test/mod.rs

all: $(TEST_BINARY)
	$(TEST_BINARY)

clean:
	rm -f $(TEST_BINARY)

$(TEST_BINARY): $(SRC) $(TEST_SRC)
	$(RUSTC) $(RUSTFLAGS) --test test/mod.rs -o $@


.PHONY: all clean
