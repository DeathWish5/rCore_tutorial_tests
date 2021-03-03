TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
ELF_DIR := build
APPS := $(wildcard $(APP_DIR)/*.rs)
CH3-APPS := $(wildcard $(APP_DIR)/ch2*.rs $(APP_DIR)/ch3*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
CH3-ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(CH3-APPS))
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
CP := cp
PY := python3

elf: $(APPS)
	@cargo build --release

binary: elf
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		$(CP) $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));	\
		$(OBJDUMP) $(elf) -d > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)

ch3-binary:
	@$(PY) ch3-build.py
	@$(foreach elf, $(CH3-ELFS), \
    		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
    		$(CP) $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));	\
    		$(OBJDUMP) $(elf) -d > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)
pre:
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/

CH2_TESTS := ch2 ch2t
CH3_TESTS_BASE := $(CH2_TESTS) ch3_0 ch3t
CH4_TESTS := ch2 ch3_0 ch4
CH5_TESTS := $(CH4_TESTS) ch5
CH6_TESTS := $(CH5_TESTS) ch6
CH7_TESTS := $(CH6_TESTS) ch7

ch2: binary pre
	@$(foreach t, $(CH2_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH2_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch3_0: ch3-binary pre
	@$(foreach t, $(CH3_TESTS_BASE), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH3_TESTS_BASE), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch3_1: ch3-binary pre
	@cp $(TARGET_DIR)/ch3_1_*.bin $(BUILD_DIR)/bin/
	@cp $(TARGET_DIR)/ch3_1_*.elf $(BUILD_DIR)/elf/

ch3_2: ch3-binary pre
	@cp $(TARGET_DIR)/ch3_2_*.bin $(BUILD_DIR)/bin/
	@cp $(TARGET_DIR)/ch3_2_*.elf $(BUILD_DIR)/elf/

ch4: binary pre
	@$(foreach t, $(CH4_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH4_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch5: binary pre
	@$(foreach t, $(CH5_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH5_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch6: binary pre
	@$(foreach t, $(CH6_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH6_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

ch7: binary pre
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@$(foreach t, $(CH7_TESTS), cp $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH7_TESTS), cp $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)

build: ch7

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean ch2 ch3