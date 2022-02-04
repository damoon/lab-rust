bench:
	RUSTFLAGS="-C target-cpu=native" taskset -c 0,2 cargo +nightly bench
