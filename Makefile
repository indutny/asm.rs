RUSTC ?= rustc
RUSTFLAGS ?=

TEST_BINARY ?= ./run-tests

SRC ?=
SRC += src/asm.rs
SRC += src/x64.rs
SRC += src/x64/base.rs
SRC += src/x64/basic.rs
SRC += src/x64/math.rs
SRC += src/x64/branching.rs

TEST_SRC ?=
TEST_SRC += test/common.rs
TEST_SRC += test/runner.rs

all: $(TEST_BINARY)
	$(TEST_BINARY)

clean:
	rm -f $(TEST_BINARY)

$(TEST_BINARY): $(SRC) $(TEST_SRC)
	$(RUSTC) $(RUSTFLAGS) --test test/runner.rs -o $@


.PHONY: all clean
