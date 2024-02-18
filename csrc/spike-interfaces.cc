#include "spike-interfaces.h"

class sim_t : public simif_t {
 public:
  sim_t(uint64_t size) {
    mem = new char[size];
    mem_size = size;
  }
  ~sim_t() { delete[] mem; }
  char* addr_to_mem(reg_t addr) override {
    return &mem[addr];
  }

  bool mmio_load(reg_t addr, size_t len, uint8_t* bytes) override {}
  bool mmio_store(reg_t addr, size_t len, const uint8_t* bytes) override {}

  bool load_elf(reg_t addr, size_t len, const uint8_t* bytes) {
    memcpy(&mem[addr], bytes, len);
    return true;
  }

  virtual void proc_reset(unsigned id) override {}
  virtual const char* get_symbol(uint64_t addr) override {}
  [[nodiscard]] const cfg_t& get_cfg() const override {}
  [[nodiscard]] const std::map<size_t, processor_t*>& get_harts() const override {}

 private:
  char* mem;
  uint64_t mem_size;
};

class Spike {
 public:
  Spike(uint64_t mem_size);

  processor_t* get_proc() { return &proc; }
  sim_t* get_sim() { return &sim; }

 private:
  std::string varch;
  cfg_t cfg;
  sim_t sim;
  isa_parser_t isa;
  processor_t proc;
};

Spike::Spike(uint64_t mem_size)
    : sim(mem_size),
      varch(fmt::format("vlen:{},elen:{}", 1024, 32)),
      isa("rv32gcv", "M"),
      cfg(/*default_initrd_bounds=*/std::make_pair((reg_t)0, (reg_t)0),
          /*default_bootargs=*/nullptr,
          /*default_isa=*/DEFAULT_ISA,
          /*default_priv=*/DEFAULT_PRIV,
          /*default_varch=*/varch.data(),
          /*default_misaligned=*/false,
          /*default_endianness*/ endianness_little,
          /*default_pmpregions=*/16,
          /*default_mem_layout=*/std::vector<mem_cfg_t>(),
          /*default_hartids=*/std::vector<size_t>(),
          /*default_real_time_clint=*/false,
          /*default_trigger_count=*/4),
      proc(
          /*isa*/ &isa,
          /*cfg*/ &cfg,
          /*sim*/ &sim,
          /*id*/ 0,
          /*halt on reset*/ true,
          /*log_file_t*/ nullptr,
          /*sout*/ std::cerr) {
  auto& csrmap = proc.get_state()->csrmap;
  constexpr uint32_t CSR_MSIMEND = 0x7cc;
  csrmap[CSR_MSIMEND] = std::make_shared<basic_csr_t>(&proc, CSR_MSIMEND, 0);
  proc.enable_log_commits();
}

// Error codes
enum ErrorCode {
  SPIKE_SUCCESS,
  SPIKE_ERROR,
  SPIKE_LOAD_ERROR,
  SPIKE_LOAD_ELF_ERROR,
  SPIKE_STORE_ERROR,
  SPIKE_INVALID_REG,
};

uint64_t spike_new(uint64_t mem_size) {
  Spike* spike = new Spike(mem_size);

  return (uint64_t)spike;
}

int32_t spike_delete(uint64_t spike) {
  Spike* s = (Spike*)spike;
  delete s;

  return SPIKE_SUCCESS;
}

int32_t spike_execute(uint64_t spike) {
  Spike* s = (Spike*)spike;
  processor_t* proc = s->get_proc();

  auto state = proc->get_state();
  auto fetch = proc->get_mmu()->load_insn(state->pc);

  std::cerr << "pc:" << fmt::format("{:08x}", state->pc) << " ";
  std::cerr << "disasm:" << proc->get_disassembler()->disassemble(fetch.insn)
            << "\n";

  reg_t pc = fetch.func(proc, fetch.insn, state->pc);

  // Bypass CSR insns commitlog stuff.
  if ((pc & 1) == 0) {
    state->pc = pc;
  } else {
    switch (pc) {
      case PC_SERIALIZE_BEFORE:
        state->serialized = true;
        break;
      case PC_SERIALIZE_AFTER:
        break;
      default:
        std::cerr << "Unknown PC: " << fmt::format("{:08x}", pc) << "\n";
    }
  }

  return SPIKE_SUCCESS;
}


int32_t spike_get_reg(uint64_t spike, uint64_t index, uint64_t* content) {
  Spike* s = (Spike*)spike;
  processor_t* proc = s->get_proc();
  state_t* state = proc->get_state();
  *content = state->XPR[index];
  return SPIKE_SUCCESS;
}

int32_t spike_set_reg(uint64_t spike, uint64_t index, uint64_t content) {
  Spike* s = (Spike*)spike;
  processor_t* proc = s->get_proc();
  if (index >= NXPR) {
    return SPIKE_INVALID_REG;
  }
  state_t* state = proc->get_state();
  state->XPR.write(index, content);
  return SPIKE_SUCCESS;
}

int32_t spike_ld(uint64_t spike, uint64_t addr, uint64_t len, uint8_t* bytes) {
  Spike* s = (Spike*)spike;
  processor_t* proc = s->get_proc();
  sim_t* sim = s->get_sim();
  bool success = sim->mmio_load(addr, len, bytes);
  if (success) {
    return SPIKE_SUCCESS;
  } else {
    return SPIKE_LOAD_ERROR;
  }
}

int32_t spike_sd(uint64_t spike, uint64_t addr, uint64_t len, uint8_t* bytes) {
  Spike* s = (Spike*)spike;
  sim_t* sim = s->get_sim();
  bool success = sim->mmio_store(addr, len, bytes);
  if (success) {
    return SPIKE_SUCCESS;
  } else {
    return SPIKE_STORE_ERROR;
  }
}

int32_t spike_ld_elf(uint64_t spike, uint64_t addr, uint64_t len, uint8_t* bytes) {
  Spike* s = (Spike*)spike;
  sim_t* sim = s->get_sim();
  bool success = sim->load_elf(addr, len, bytes);
  if (success) {
    return SPIKE_SUCCESS;
  } else {
    return SPIKE_LOAD_ELF_ERROR;
  }
}

int32_t spike_init(uint64_t spike, uint64_t entry_addr) {
  Spike* s = (Spike*)spike;
  processor_t* proc = s->get_proc();

  proc->reset();

  // Set the virtual supervisor mode and virtual user mode
  // auto status = proc->get_state()->sstatus->read() | SSTATUS_VS | SSTATUS_FS;
  // proc->get_state()->sstatus->write(status);
  proc->get_state()->pc = entry_addr;

  return SPIKE_SUCCESS;
}
