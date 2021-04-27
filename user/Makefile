TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
CP := cp
PY := python3
BUILD_SCRIPTS := build.py
CH_TESTS := 
CHAPTER ?= 
HAVE_USERTESTS := 5 6 7

CH2_TESTS := ch2_ ch2t_
CH2_TESTS_BAD := _ch2_ _ch2t_
CH3_TESTS_BASE := $(CH2_TESTS) ch3_0_ ch3t_
CH4_TESTS := ch2_ ch3_0_ ch4_
CH5_TESTS := $(CH4_TESTS) ch5_
CH6_TESTS := $(CH5_TESTS) ch6_
CH7_TESTS := $(CH6_TESTS) ch7_
CH8_TESTS := $(CH7_TESTS) ch8_

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
	CH_TESTS := ch3_1_
	BUILD_SCRIPTS := ch3-build.py
else ifeq ($(CHAPTER), 3_2)
	CH_TESTS := ch3_2_
	BUILD_SCRIPTS := ch3-build.py
else ifeq ($(CHAPTER), 4)
	CH_TESTS := $(CH4_TESTS)
else ifeq ($(CHAPTER), 4_only)
	CH_TESTS := ch4
else ifeq ($(CHAPTER), 5)
	CH_TESTS := $(CH5_TESTS)
else ifeq ($(CHAPTER), 5_only)
	CH_TESTS := ch5
else ifeq ($(CHAPTER), 6)
	CH_TESTS := $(CH6_TESTS)
else ifeq ($(CHAPTER), 6_only)
	CH_TESTS := ch6
else ifeq ($(CHAPTER), 7)
	CH_TESTS := $(CH7_TESTS)
else ifeq ($(CHAPTER), 7_only)
	CH_TESTS := ch7
else ifeq ($(CHAPTER), 8)
	CH_TESTS := $(CH8_TESTS)
else ifeq ($(CHAPTER), 8_only)
	CH_TESTS := ch8
endif

APPS := $(foreach c, $(CH_TESTS), $(wildcard $(APP_DIR)/$(c)*.rs))
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

binary: $(APPS)
	@$(PY) $(BUILD_SCRIPTS) $(CHAPTER)
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
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)*.bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)*.elf $(BUILD_DIR)/elf/;)
	@$(foreach t, $(CH_TESTS), $(CP) $(TARGET_DIR)/$(t)*.asm $(BUILD_DIR)/asm/;)
    ifneq ($(filter $(CHAPTER), $(HAVE_USERTESTS)),)
		@cp $(BUILD_DIR)/elf/ch$(CHAPTER)_usertest.elf $(BUILD_DIR)/elf/initproc.elf
		@cp $(TARGET_DIR)/ch$(CHAPTER)_usertest $(TARGET_DIR)/initproc
    endif


clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

.PHONY: elf binary build clean ch2 ch3
