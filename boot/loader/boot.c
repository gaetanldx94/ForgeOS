#include <stdint.h>
#include "vga.h"
#include "cpu.h"
#include "memory.h"
#include "elf.h"
#include "boot_info.h"

static BootInfo boot_info;

typedef void (*KernelEntry)(BootInfo *);

static void ok(const char *msg)
{
    vga_print("[OK] ", COLOR(GREEN, BLACK));
    vga_print(msg, COLOR(WHITE, BLACK));
}

static void err(const char *msg)
{
    vga_print("[ERR] ", COLOR(RED, BLACK));
    vga_print(msg, COLOR(WHITE, BLACK));
}

static void info(const char *msg)
{
    vga_print("[..] ", COLOR(YELLOW, BLACK));
    vga_print(msg, COLOR(WHITE, BLACK));
}

void boot_main()
{
    vga_clear();

    vga_print("ForgeOS Bootloader", COLOR(CYAN, BLACK));
    vga_print("==================", COLOR(CYAN, BLACK));

    info("Checking CPU...");
    CpuInfo cpu;
    if (!cpu_check(&cpu))
    {
        err("CPU does not support long mode!");
        while (1)
        {
        }
    }
    ok("CPU OK - vendor: ");
    vga_print(cpu.vendor, COLOR(WHITE, BLACK));
    if (cpu.sse)
        ok("SSE supported");
    if (cpu.sse2)
        ok("SSE2 supported");
    if (cpu.nx)
        ok("NX bit supported");

    info("Reading memory map...");
    MemMap mem;
    memory_read(&mem);
    ok("Memory map read - entries: ");
    vga_print_int(mem.count, COLOR(WHITE, BLACK));
    ok("Total free memory: ");
    vga_print_int(mem.total_free / 1024 / 1024, COLOR(WHITE, BLACK));
    vga_print(" MB", COLOR(WHITE, BLACK));

    info("Validating memory...");
    if (!memory_validate(&mem, 4 * 1024 * 1024))
    {
        err("Not enough memory or kernel zone not free!");
        while (1)
        {
        }
    }
    ok("Memory OK");

    info("Loading kernel ELF...");
    ElfInfo elf;
    if (!elf_load((void *)0x20000, &elf))
    {
        err("Invalid ELF kernel!");
        while (1)
        {
        }
    }
    ok("Kernel loaded");
    ok("Entry point: ");
    vga_print_hex(elf.entry, COLOR(WHITE, BLACK));

    boot_info.magic = BOOT_MAGIC;
    boot_info.mem_total = mem.total_free;
    boot_info.mem_map_count = mem.count;
    boot_info.kernel_start = elf.start;
    boot_info.kernel_end = elf.end;
    boot_info.kernel_entry = elf.entry;
    for (uint32_t i = 0; i < mem.count; i++)
    {
        boot_info.mem_map[i] = mem.entries[i];
    }

    typedef void (*KernelEntry)(void);
    KernelEntry entry = (KernelEntry)elf.entry;
    entry();

    err("Kernel returned!");
    while (1)
    {
    }
}