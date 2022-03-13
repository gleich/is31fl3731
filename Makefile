build-all:
	cargo build --target=armv7-unknown-linux-gnueabihf --examples

dev-deploy: build-all
	scp target/armv7-unknown-linux-gnueabihf/debug/examples/rpi pi@mgdev.local:~/is31fl3731