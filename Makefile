TRIPLE := wasm32-unknown-unknown

WASM_SRC := fixtures/wasm
WAT_SRC := fixtures/wat
FIXTURE_WATS = $(wildcard $(WAT_SRC)/*.wat)

.PHONY: run
run:
	cargo run
	
wasm: $(FIXTURE_WATS)

$(FIXTURE_WATS):
	wat2wasm $@ -o $(WASM_SRC)/$(shell basename $@ .wat).wasm

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test