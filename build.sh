set -ex

if [ ! -f ./libspike/libspike-interfaces.a ]; then
    mkdir -p libspike
    g++ -c -fPIC -I./riscv-isa-sim/riscv  -I./riscv-isa-sim/build -I./riscv-isa-sim/softfloat -I./riscv-isa-sim/fesvr src/spike-interfaces.cc -o libspike/spike-interfaces.o
    ar rcs libspike/libspike-interfaces.a libspike/spike-interfaces.o
fi