CC = gcc
CFLAGS = # -std=c99 -Wall -Werror -Wpedantic -Wextra -Wvla -Wfloat-equal -Wfloat-conversion
INC= -I inc

FUNC_TESTING = testing.sh
TEST_DIR = func_test
SRC = $(wildcard src/*.c)
MAIN = src/main.c
HEADERS = $(wildcard inc/*.h)

OBJDIR = out
OBJ = $(patsubst src/%.c, $(OBJDIR)/%.o, $(SRC))

BUILD = 
TARGET = $(BUILD)app.exe

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

.PHONY: clean
clean:
	$(RM) ./*.exe -r $(OBJDIR)

$(TARGET): $(OBJDIR) $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $@

$(OBJDIR):
	@mkdir -p out

$(OBJDIR)/%.o: src/%.c $(HEADERS)
	$(CC) $(CFLAGS) $(INC) -c $< -o $@
