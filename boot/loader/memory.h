#pragma once
#include <stdint.h>

#define MEM_MAP_ADDR    0x7000
#define MEM_MAP_MAX     32

#define MEM_TYPE_FREE       1
#define MEM_TYPE_RESERVED   2
#define MEM_TYPE_ACPI       3
#define MEM_TYPE_NVS        4
#define MEM_TYPE_BAD        5

typedef struct {
    uint64_t base;
    uint64_t length;
    uint32_t type;
} __attribute__((packed)) MemMapEntry;

typedef struct {
    uint32_t     count;
    uint64_t     total_free;
    MemMapEntry  entries[MEM_MAP_MAX];
} MemMap;

void memory_read(MemMap *map);
int  memory_validate(MemMap *map, uint64_t kernel_size);