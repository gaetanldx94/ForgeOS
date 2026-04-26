#include "cpu.h"
#include "vga.h"

static void cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx,
                  uint32_t *ecx, uint32_t *edx) {
    asm volatile("cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf)
        : "memory");
}

int cpu_check(CpuInfo *info) {
    uint32_t eax, ebx, ecx, edx;

    cpuid(0, &eax, &ebx, &ecx, &edx);
    ((uint32_t *)info->vendor)[0] = ebx;
    ((uint32_t *)info->vendor)[1] = edx;
    ((uint32_t *)info->vendor)[2] = ecx;
    info->vendor[12] = 0;

    cpuid(1, &eax, &ebx, &ecx, &edx);
    info->sse  = (edx >> 25) & 1;
    info->sse2 = (edx >> 26) & 1;

    cpuid(0x80000001, &eax, &ebx, &ecx, &edx);
    info->long_mode = (edx >> 29) & 1;
    info->nx        = (edx >> 20) & 1;

    if (!info->long_mode) return 0;
    return 1;
}