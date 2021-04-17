default:
	cargo build --release
install:
	sudo cp target/release/distilatex /usr/bin/
