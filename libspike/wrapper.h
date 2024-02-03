#include <string>

#include "mmu.h"
#include "processor.h"

class Wrapper {
public:
  Wrapper();

private:
  std::string varch;
  cfg_t cfg;
  simif_t sim;
  isa_parser_t isa;
  processor_t proc;

};

typedef void* Wrapper;

Wrapper Processor_new(const char* isa, const char* priv, const char* varch, const char* vext, const char* vpriv);