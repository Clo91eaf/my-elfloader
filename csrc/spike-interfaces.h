#ifndef __SPIKE_INTERFCES_H__
#define __SPIKE_INTERFCES_H__

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <mutex>
#include <queue>
#include <string>
#include <optional>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <functional>

#include <fmt/core.h>
#include <fmt/ranges.h>

#include "disasm.h"
#include "mmu.h"
#include "processor.h"
#include "simif.h"
#include "cfg.h"
#include "decode_macros.h"

#ifdef __cplusplus
extern "C" {
#endif

// Rust callback for memory access
using rust_callback = std::function<char*(reg_t)>;
rust_callback rs_addr_to_mem;

uint64_t spike_new();
int32_t spike_execute(uint64_t spike);
int32_t spike_init(uint64_t spike, uint64_t entry_addr);
int32_t spike_register_callback(rust_callback callback);

#ifdef __cplusplus
}
#endif

#endif // __SPIKE_INTERFCES_H__
