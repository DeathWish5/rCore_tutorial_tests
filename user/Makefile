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
CP := cp

elf: $(APPS)
	@cargo build --release

binary: elf
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		$(CP) $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));	\
		$(OBJDUMP) $(elf) -d > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)

CH2_TESTS := ch2 ch2t
CH3_TESTS_BASE := $(CH2_TESTS) ch3_0
CH3_TESTS_SEP := ch3_1 ch3_2 ch3_3
CH4_TESTS := ch2 ch3_0 ch4
ch5_TESTS := $(CH4_TESTS) ch5
ch6_TESTS := $(CH5_TESTS) ch6

ch2: binary
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@$(foreach t, $(CH2_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH2_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch3: binary
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/base
	@mkdir -p $(BUILD_DIR)/elf/base
	@$(foreach t, $(CH3_TESTS_BASE, mkdir -p $(BUILD_DIR)/bin/$(t); cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/base/;)
	@$(foreach t, $(CH3_TESTS_BASE, mkdir -p $(BUILD_DIR)/elf/$(t); cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/elf/base/;)
	@$(foreach t, $(CH3_TESTS_SEP, mkdir -p $(BUILD_DIR)/bin/$(t); cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/$(t)/;)
	@$(foreach t, $(CH3_TESTS_SEP), mkdir -p $(BUILD_DIR)/elf/$(t); cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/$(t)/;)

ch4: binary
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@$(foreach t, $(CH4_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH4_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch5: binary
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@$(foreach t, $(CH5_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH5_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch6: binary
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@$(foreach t, $(CH6_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH6_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

build: ch6

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean ch2 ch3