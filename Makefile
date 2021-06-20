release:
	cargo rustc --bin minigrep --offline --release -- -C prefer-dynamic

clean:
	rm -rf target

doc:
	cargo doc --offline

test: 
	cargo test --offline
