TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
ELF_DIR := build
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf: $(APPS)
	@cargo build --release

binary: elf
	@$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

CH2_TESTS := ch2
CH2T_TESTS := ch2t
CH3_TESTS := $(CH2_TESTS) ch3_0 ch3_1 ch3_2 ch3_3
CH3T_TESTS := ch2t ch3t

ch2: binary
	@rm -rf $(BUILD_DIR)
	@$(foreach t, $(CH2_TESTS) $(CH2T_TESTS), mkdir -p $(BUILD_DIR)/bin/$(t); cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/$(t)/;)
	@$(foreach t, $(CH2_TESTS) $(CH2T_TESTS), mkdir -p $(BUILD_DIR)/elf/$(t); cp $(ELFS) $(BUILD_DIR)/elf/$(t)/;)

ch3: binary
	@rm -rf $(BUILD_DIR)
	@$(foreach t, $(CH3_TESTS) $(CH3T_TESTS), mkdir -p $(BUILD_DIR)/bin/$(t); cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/$(t)/;)
	@$(foreach t, $(CH3_TESTS) $(CH3T_TESTS), mkdir -p $(BUILD_DIR)/elf/$(t); cp $(ELFS) $(BUILD_DIR)/elf/$(t)/;)

build: ch3

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean ch2 ch3