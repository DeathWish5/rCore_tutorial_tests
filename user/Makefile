TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
CP := cp
PY := python3
BUILD_SCRIPTS := build.py
CH_TESTS := 
CHAPTER ?= 

CH2_TESTS := ch2 ch2t
CH2_TESTS_BAD := _ch2 _ch2t
CH3_TESTS_BASE := $(CH2_TESTS) ch3_0 ch3t
CH4_TESTS := ch2 ch3_0 ch4
CH5_TESTS := $(CH4_TESTS) ch5
CH6_TESTS := $(CH5_TESTS) ch6
CH7_TESTS := $(CH6_TESTS) ch7

ifeq ($(CHAPTER), 2)
	CH_TESTS := $(CH2_TESTS)
	BUILD_SCRIPTS := ch2-build.py
else ifeq ($(CHAPTER), 2_bad)
	CH_TESTS := $(CH2_TESTS_BAD)
	BUILD_SCRIPTS := ch2-build.py
else ifeq ($(CHAPTER), 3_0)
	CH_TESTS := $(CH3_TESTS_BASE)
	BUILD_SCRIPTS := ch3-build.py
else ifeq ($(CHAPTER), 3_1)
	CH_TESTS := ch3_1
	BUILD_SCRIPTS := ch3-build.py
else ifeq ($(CHAPTER), 3_2)
	CH_TESTS := ch3_2
	BUILD_SCRIPTS := ch3-build.py
else ifeq ($(CHAPTER), 4)
	CH_TESTS := $(CH4_TESTS)
else ifeq ($(CHAPTER), 5)
	CH_TESTS := $(CH5_TESTS)
else ifeq ($(CHAPTER), 6)
	CH_TESTS := $(CH6_TESTS)
else ifeq ($(CHAPTER), 7)
	CH_TESTS := $(CH7_TESTS)
endif

binary: $(APPS)
	@$(PY) $(BUILD_SCRIPTS)
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		$(CP) $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));	\
		$(OBJDUMP) $(elf) -d > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)

pre:
	@rm -rf $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@mkdir -p $(BUILD_DIR)/asm/
	
all: binary pre
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)_*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)_*.elf $(BUILD_DIR)/elf/;)
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)_*.asm $(BUILD_DIR)/asm/;)

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean ch2 ch3