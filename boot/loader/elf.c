#include "elf.h"
#include "vga.h"

static void my_memcpy(void *dst, const void *src, uint32_t n) {
    uint8_t *d = (uint8_t *)dst;
    const uint8_t *s = (const uint8_t *)src;
    while (n--) *d++ = *s++;
}

static void my_memset(void *dst, uint8_t val, uint32_t n) {
    uint8_t *d = (uint8_t *)dst;
    while (n--) *d++ = val;
}

int elf_load(void *data, ElfInfo *info) {
    Elf64Header *hdr = (Elf64Header *)data;

    if (hdr->magic != ELF_MAGIC) return 0;
    if (hdr->class != ELF_CLASS_64) return 0;

    info->entry = hdr->entry;
    info->start = (uint64_t)-1;
    info->end   = 0;

    Elf64Phdr *ph = (Elf64Phdr *)((uint8_t *)data + (uint32_t)hdr->phoff);

    for (int i = 0; i < hdr->phnum; i++) {
        if (ph[i].type != PT_LOAD) continue;

        my_memcpy((void *)(uint32_t)ph[i].paddr,
                  (uint8_t *)data + (uint32_t)ph[i].offset,
                  (uint32_t)ph[i].filesz);

        if (ph[i].memsz > ph[i].filesz) {
            my_memset((void *)(uint32_t)(ph[i].paddr + ph[i].filesz),
                      0,
                      (uint32_t)(ph[i].memsz - ph[i].filesz));
        }

        if (ph[i].paddr < info->start) info->start = ph[i].paddr;
        if (ph[i].paddr + ph[i].memsz > info->end)
            info->end = ph[i].paddr + ph[i].memsz;
    }

    return 1;
}