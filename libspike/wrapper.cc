#include "wrapper.h"
#include "include/riscv/processor.h"

VBridgeImpl::VBridgeImpl(const std::string &config_path, const CosimConfig &cosim_config)
    : config(config_path.c_str()),
      varch(fmt::format("vlen:{},elen:{}", config.v_len, config.elen)),
      sim(1l << 32), isa("rv32gcv", "M"),
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
          /*sout*/ std::cerr),
      se_to_issue(nullptr), tl_req_record_of_bank(config.tl_bank_number),
      tl_req_waiting_ready(config.tl_bank_number),
      tl_req_ongoing_burst(config.tl_bank_number),
      bin(cosim_config.bin_path),
      wave(cosim_config.wave_path),
      perf_path(cosim_config.perf_path),
      timeout(cosim_config.timeout),
      tck(cosim_config.tck),

#ifdef COSIM_VERILATOR
      ctx(nullptr),
#endif
      vrf_shadow(std::make_unique<uint8_t[]>(config.v_len_in_bytes *
                                             config.vreg_number))
                                             {

  auto &csrmap = proc.get_state()->csrmap;
  csrmap[CSR_MSIMEND] = std::make_shared<basic_csr_t>(&proc, CSR_MSIMEND, 0);
  proc.enable_log_commits();

  this->using_dramsim3 = cosim_config.dramsim3_config_path.has_value();

  if(this->using_dramsim3) {
    for(int i = 0; i < config.tl_bank_number; ++i) {
      std::string result_dir = cosim_config.dramsim3_config_path.value() + "/channel." + std::to_string(i);
      std::filesystem::create_directories(result_dir);
      auto completion = [i, this](uint64_t address) {
        this->dramsim_resolve(i, address);
      };

      drams.emplace_back(dramsim3::MemorySystem(cosim_config.dramsim3_config_path.value(), result_dir, completion, completion), 0);
      // std::cout<<"Relative tck ratio on channel "<<i<<" = "<<tck / drams[i].first.GetTCK()<<std::endl;
    }
  }
}

ProcessorPtr Processor_new();