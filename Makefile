# Cross-platform Makefile for Linux, macOS, Windows (MSYS2/MinGW)

# Detect OS
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Linux)
    TARGET = tct
    RM = rm -f
    MKDIR = mkdir -p
else ifeq ($(UNAME_S),Darwin)
    TARGET = tct
    RM = rm -f
    MKDIR = mkdir -p
else
    TARGET = tct.exe
    RM = del /Q
    MKDIR = mkdir
endif

CC = gcc
CFLAGS = -Wall -Wextra -O2
SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)
OUT = build/$(TARGET)

.PHONY: all clean

all: $(OUT)

$(OUT): $(SRC)
	$(MKDIR) build
	$(CC) $(CFLAGS) -o $@ $^

clean:
	$(RM) build/$(TARGET)
