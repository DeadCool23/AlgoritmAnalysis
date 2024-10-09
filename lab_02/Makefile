CC = gcc
CFLAGS = # -std=c99 -Wall -Werror -Wpedantic -Wextra -Wvla -Wfloat-equal -Wfloat-conversion
TESTFLAGS = --coverage -lcheck -lm -lsubunit -lrt -lpthread
INC= -I inc

SRC = $(wildcard src/*.c)
TESTSRC = $(wildcard tests/*.c)

MAIN = src/main.c
HEADERS = $(wildcard inc/*.h)

READY_DIR = ready

OBJDIR = out
OBJ = $(patsubst src/%.c, $(OBJDIR)/%.o, $(SRC))
TESTOBJ = $(patsubst tests/%.c, $(OBJDIR)/%.o, $(TESTSRC))

BUILD = 
TARGET = $(BUILD)app.exe
DEBUG_TARGET = $(BUILD)debug.exe
TEST_TARGET = $(BUILD)test.exe

TEST_INFO_SCRIPT=tests/get_test_info.sh

READY_DIR = ready

.PHONY: build
build: $(TARGET)

.PHONY: release
release: CFLAGS += -O3
release: CFLAGS += -lm
release: $(TARGET)

.PHONY: debug
debug: CFLAGS += -g
debug: CFLAGS += -O0
debug: CFLAGS += -lm
debug: $(TARGET)
	mv $^ $(DEBUG_TARGET)

.PHONY: clean
clean:
	$(RM) ./*.exe ./*.gcov ./*.gcda -r $(OBJDIR)

.PHONY: test
test: CFLAGS += -g
test: CFLAGS += -O0
test: CFLAGS += -DTEST
test: CFLAGS += --coverage
test: $(TEST_TARGET)
	@./$(TEST_INFO_SCRIPT)

$(TARGET): $(OBJDIR) $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $@

$(TEST_TARGET): $(OBJDIR) $(OBJ) $(TESTOBJ)
	$(CC) $(CFLAGS) $(OBJ) $(TESTOBJ) -o $@ $(TESTFLAGS)

$(OBJDIR):
	@mkdir -p out

$(OBJDIR)/%.o: tests/%.c $(HEADERS)
	$(CC) $(INC) -c $< -o $@ $(CFLAGS) $(TESTFLAGS)

$(OBJDIR)/%.o: src/%.c $(HEADERS)
	$(CC) $(INC) -c $< -o $@ $(CFLAGS) 

# Для пайплайнов
$(READY_DIR)/stud-unit-test-report-prev.json:
	

$(READY_DIR)/stud-unit-test-report.json: $(READY_DIR)
	cp tests/stud-unit-test-report-prev.json $@

$(READY_DIR)/app-cli-debug: debug $(READY_DIR)
	mv $(DEBUG_TARGET) $@

$(READY_DIR)/report.pdf: $(READY_DIR)
	cp docs/report.pdf ready/report.pdf

$(READY_DIR):
	@mkdir -p ./ready
