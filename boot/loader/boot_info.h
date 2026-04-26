#pragma once
#include <stdint.h>
#include "memory.h"

#define BOOT_MAGIC 0xB007CAFE

typedef struct {
    uint32_t    magic;
    uint64_t    mem_total;
    uint32_t    mem_map_count;
    MemMapEntry mem_map[MEM_MAP_MAX];
    uint64_t    kernel_start;
    uint64_t    kernel_end;
    uint64_t    kernel_entry;
} __attribute__((packed)) BootInfo;