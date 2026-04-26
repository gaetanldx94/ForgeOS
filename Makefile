NASM        := nasm
GCC         := gcc
LD          := ld
CARGO       := cargo
QEMU        := qemu-system-x86_64

BIOS_DIR    := boot/bios
LOADER_DIR  := boot/loader
KERNEL_DIR  := kernel
BUILD_DIR   := build

LOADER_OBJS := $(BUILD_DIR)/gdt.o      \
               $(BUILD_DIR)/pmode.o    \
               $(BUILD_DIR)/paging.o   \
               $(BUILD_DIR)/longmode.o \
               $(BUILD_DIR)/boot.o     \
               $(BUILD_DIR)/vga.o      \
               $(BUILD_DIR)/cpu.o      \
               $(BUILD_DIR)/memory.o   \
               $(BUILD_DIR)/elf.o

BOOTSECTOR  := $(BUILD_DIR)/bootsector.bin
ENTRY_BIN   := $(BUILD_DIR)/entry.bin
LOADER_BIN  := $(BUILD_DIR)/loader.bin
KERNEL_BIN  := $(BUILD_DIR)/kernel.bin
OS_IMG      := $(BUILD_DIR)/forgeos.img

.PHONY: all clean run debug

all: $(OS_IMG)

$(OS_IMG): $(BOOTSECTOR) $(ENTRY_BIN) $(LOADER_BIN) $(KERNEL_BIN)
	@echo "[IMG] Assemblage de l'image..."
	dd if=/dev/zero of=$@ bs=512 count=4096 2>/dev/null
	dd if=$(BOOTSECTOR) of=$@ bs=512 seek=0  conv=notrunc 2>/dev/null
	dd if=$(ENTRY_BIN)  of=$@ bs=512 seek=1  conv=notrunc 2>/dev/null
	dd if=$(LOADER_BIN) of=$@ bs=512 seek=2  conv=notrunc 2>/dev/null
	dd if=$(KERNEL_BIN) of=$@ bs=512 seek=22 conv=notrunc 2>/dev/null
	@echo "[IMG] forgeos.img prête"

$(BOOTSECTOR): $(BIOS_DIR)/bootsector.asm | $(BUILD_DIR)
	@echo "[NASM] bootsector..."
	$(NASM) -f bin $< -o $@

$(ENTRY_BIN): $(LOADER_DIR)/entry.asm $(LOADER_BIN) | $(BUILD_DIR)
	@echo "[NASM] entry.asm -> bin"
	$(eval SETUP_GDT_ADDR := 0x$(shell nm $(BUILD_DIR)/loader.elf | grep ' setup_gdt' | awk '{print $$1}'))
	@echo "  setup_gdt @ $(SETUP_GDT_ADDR)"
	$(NASM) -f bin -D SETUP_GDT_ADDR=$(SETUP_GDT_ADDR) $(LOADER_DIR)/entry.asm -o $@

$(BUILD_DIR)/gdt.o: $(LOADER_DIR)/gdt.asm | $(BUILD_DIR)
	$(NASM) -f elf64 $< -o $@

$(BUILD_DIR)/pmode.o: $(LOADER_DIR)/pmode.asm | $(BUILD_DIR)
	$(NASM) -f elf64 $< -o $@

$(BUILD_DIR)/paging.o: $(LOADER_DIR)/paging.asm | $(BUILD_DIR)
	$(NASM) -f elf64 $< -o $@

$(BUILD_DIR)/longmode.o: $(LOADER_DIR)/longmode.asm | $(BUILD_DIR)
	$(NASM) -f elf64 $< -o $@

GCC_FLAGS := -ffreestanding -nostdlib -fno-stack-protector \
             -fno-pic -mno-red-zone -mcmodel=kernel

$(BUILD_DIR)/boot.o: $(LOADER_DIR)/boot.c | $(BUILD_DIR)
	@echo "[GCC] $<"
	$(GCC) $(GCC_FLAGS) -c $< -o $@

$(BUILD_DIR)/vga.o: $(LOADER_DIR)/vga.c | $(BUILD_DIR)
	@echo "[GCC] $<"
	$(GCC) $(GCC_FLAGS) -c $< -o $@

$(BUILD_DIR)/cpu.o: $(LOADER_DIR)/cpu.c | $(BUILD_DIR)
	@echo "[GCC] $<"
	$(GCC) $(GCC_FLAGS) -c $< -o $@

$(BUILD_DIR)/memory.o: $(LOADER_DIR)/memory.c | $(BUILD_DIR)
	@echo "[GCC] $<"
	$(GCC) $(GCC_FLAGS) -c $< -o $@

$(BUILD_DIR)/elf.o: $(LOADER_DIR)/elf.c | $(BUILD_DIR)
	@echo "[GCC] $<"
	$(GCC) $(GCC_FLAGS) -c $< -o $@

$(LOADER_BIN): $(LOADER_OBJS) linker.ld | $(BUILD_DIR)
	@echo "[LD] Linkage loader..."
	$(LD) -T linker.ld -o $(BUILD_DIR)/loader.elf $(LOADER_OBJS)
	objcopy -O binary $(BUILD_DIR)/loader.elf $@

$(KERNEL_BIN): | $(BUILD_DIR)
	@echo "[CARGO] Compilation du kernel..."
	cd $(KERNEL_DIR) && $(CARGO) build --release
	cp $(KERNEL_DIR)/target/x86_64-unknown-none/release/kernel $@
	@echo "[CARGO] Kernel compilé"

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

run: $(OS_IMG)
	@echo "[QEMU] Démarrage de ForgeOS..."
	$(QEMU) \
		-drive format=raw,file=$(OS_IMG) \
		-m 128M \
		-serial stdio \
		-no-reboot \
		-no-shutdown

debug: $(OS_IMG)
	@echo "[QEMU] Mode debug sur port 1234..."
	$(QEMU) \
		-drive format=raw,file=$(OS_IMG) \
		-m 128M \
		-serial stdio \
		-no-reboot \
		-no-shutdown \
		-s -S

clean:
	@echo "[CLEAN] Nettoyage..."
	rm -rf $(BUILD_DIR)
	cd $(KERNEL_DIR) && $(CARGO) clean
	@echo "[CLEAN] Done"