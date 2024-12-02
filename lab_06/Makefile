READY_DIR = ready
DEBUG_EXE = debug_app.exe
RELEASE_EXE = release_app.exe

build:
	@cargo build --release && cp target/release/lab_06.exe $(RELEASE_EXE)

debug:
	@cargo build && cp ./target/debug/lab_06.exe $(DEBUG_EXE)

clean:
	@cargo clean
	@rm -fr plots $(READY_DIR)
	@rm -f *app.exe

test:
	cargo test

# pipelines
$(READY_DIR)/stud-unit-test-report-prev.json:
	

$(READY_DIR)/stud-unit-test-report.json: $(READY_DIR)
	cp tests/stud-unit-test-report-prev.json $@

$(READY_DIR)/app-cli-debug: debug $(READY_DIR)
	mv $(DEBUG_EXE) $@

$(READY_DIR)/report.pdf: $(READY_DIR)
	cp docs/report/report.pdf ready/report.pdf

$(READY_DIR):
	@mkdir -p ./ready
