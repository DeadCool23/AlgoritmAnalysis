build:
	@cargo build --release && cp target/release/lab_03.exe app.exe

debug:
	@cargo build && cp target/debug/lab_03.exe debug_app.exe

clean:
	@cargo clean
	@rm -fr plots
	@rm *app.exe
	@rm app-cli-debug

test:
	cargo test
