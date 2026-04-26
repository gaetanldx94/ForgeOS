#pragma once
#include <stdint.h>

#define ELF_MAGIC       0x464C457F  // 0x7F 'E' 'L' 'F'
#define ELF_CLASS_64    2
#define ELF_TYPE_EXEC   2
#define PT_LOAD         1

typedef struct {
    uint32_t magic;
    uint8_t  class;
    uint8_t  data;
    uint8_t  version;
    uint8_t  os_abi;
    uint8_t  pad[8];
    uint16_t type;
    uint16_t machine;
    uint32_t version2;
    uint64_t entry;
    uint64_t phoff;
    uint64_t shoff;
    uint32_t flags;
    uint16_t ehsize;
    uint16_t phentsize;
    uint16_t phnum;
    uint16_t shentsize;
    uint16_t shnum;
    uint16_t shstrndx;
} __attribute__((packed)) Elf64Header;

typedef struct {
    uint32_t type;
    uint32_t flags;
    uint64_t offset;
    uint64_t vaddr;
    uint64_t paddr;
    uint64_t filesz;
    uint64_t memsz;
    uint64_t align;
} __attribute__((packed)) Elf64Phdr;

typedef struct {
    uint64_t entry;
    uint64_t start;
    uint64_t end;
} ElfInfo;

int elf_load(void *data, ElfInfo *info);