debug: setup_dev
	cargo build
	./get_modules.py

run: debug
	cargo run

release: setup_dev
	cargo build --release
	./get_modules.py

clean:
	cargo clean

setup_dev:
	git submodule init
	git submodule update
	./apply_patches.sh
