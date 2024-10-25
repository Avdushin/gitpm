APP_NAME := gitpm
BIN_DIR := bin

# Команды для кросс-компиляции
TARGET_WINDOWS := x86_64-pc-windows-gnu
TARGET_LINUX := x86_64-unknown-linux-gnu
TARGET_MAC := x86_64-apple-darwin

RUSTFLAGS := --release

.PHONY: all clean prepare

all: prepare build-windows build-linux build-mac

prepare:
	mkdir -p $(BIN_DIR)

build-windows:
	cargo build $(RUSTFLAGS) --target $(TARGET_WINDOWS)
	mv target/$(TARGET_WINDOWS)/release/$(APP_NAME).exe $(BIN_DIR)/$(APP_NAME)-windows.exe

build-linux:
	cargo build $(RUSTFLAGS) --target $(TARGET_LINUX)
	mv target/$(TARGET_LINUX)/release/$(APP_NAME) $(BIN_DIR)/$(APP_NAME)-linux

build-mac:
	cargo build $(RUSTFLAGS) --target $(TARGET_MAC)
	mv target/$(TARGET_MAC)/release/$(APP_NAME) $(BIN_DIR)/$(APP_NAME)-mac

clean:
	cargo clean
	rm -rf $(BIN_DIR)
