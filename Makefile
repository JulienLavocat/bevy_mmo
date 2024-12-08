server:
	watchexec -i 'client/**' -c -r cargo run --features bevy/dynamic_linking --bin server
client:
	cargo run --features bevy/dynamic_linking --bin client 

.PHONY: client
.PHONY: server
