.PHONY: frontend backend
all: backend frontend

frontend: frontend/dist
backend: target/x86_64-unknown-linux-gnu/release/kermitbot

frontend/dist:
	${MAKE} -C frontend

target/x86_64-unknown-linux-gnu/release/kermitbot:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-gnu

target/x86_64-unknown-linux-musl/release/kermitbot:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
