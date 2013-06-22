RUSTC ?= rustc
RUSTFLAGS ?=

TEST_BINARY ?= ./run-tests

SRC ?=
SRC += src/masm.rs
SRC += src/x64.rs

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
