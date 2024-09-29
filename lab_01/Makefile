build:
	@cargo build --release && cp target/release/lab_01.exe app.exe

debug:
	@cargo build && cp target/debug/lab_01.exe debug_app.exe

test:
	@cargo test

clean:
	@cargo clean
	@rm -fr plots
	@rm *app.exe