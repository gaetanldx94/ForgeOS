#pragma once
#include <stdint.h>

typedef struct {
    uint8_t long_mode;
    uint8_t sse;
    uint8_t sse2;
    uint8_t nx;
    char vendor[13];
} CpuInfo;

int cpu_check(CpuInfo *info);