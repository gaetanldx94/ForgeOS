#include "memory.h"
#include "vga.h"

void memory_read(MemMap *map) {
    uint32_t *base = (uint32_t *)MEM_MAP_ADDR;
    map->count = *base;
    if (map->count > MEM_MAP_MAX) map->count = MEM_MAP_MAX;

    MemMapEntry *entries = (MemMapEntry *)(base + 1);
    map->total_free = 0;

    for (uint32_t i = 0; i < map->count; i++) {
        map->entries[i] = entries[i];
        if (entries[i].type == MEM_TYPE_FREE) {
            map->total_free += entries[i].length;
        }
    }
}

int memory_validate(MemMap *map, uint64_t kernel_size) {
    if (map->total_free < 4 * 1024 * 1024) return 0;

    for (uint32_t i = 0; i < map->count; i++) {
        if (map->entries[i].type == MEM_TYPE_FREE &&
            map->entries[i].base <= 0x100000 &&
            map->entries[i].base + map->entries[i].length
                >= 0x100000 + kernel_size) {
            return 1;
        }
    }
    return 0;
}