add_library(libspike STATIC IMPORTED GLOBAL)
set_target_properties(libspike PROPERTIES
  IMPORTED_LOCATION "/nix/store/8c4px73jj15h155vsqfv4bn1m21f2x7n-libspike-1.1.0/lib/libriscv.so")
target_include_directories(libspike AFTER INTERFACE
  "/nix/store/8c4px73jj15h155vsqfv4bn1m21f2x7n-libspike-1.1.0/include"
  "/nix/store/8c4px73jj15h155vsqfv4bn1m21f2x7n-libspike-1.1.0/include/riscv"
  "/nix/store/8c4px73jj15h155vsqfv4bn1m21f2x7n-libspike-1.1.0/include/fesvr"
  "/nix/store/8c4px73jj15h155vsqfv4bn1m21f2x7n-libspike-1.1.0/include/softfloat"
)

