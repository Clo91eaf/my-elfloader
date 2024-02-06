set -ex

if [ ! -f ./libspike/target/spike-interfaces.a ]; then
    rm -rf libspike/target
    mkdir -p libspike/target
    g++ -c -fPIC \
      -I./libspike/include/fesvr \
      -I./libspike/include/softfloat \
      -I./libspike/include/riscv \
      -I./libspike/include/lib \
      -I./libspike/include \
      libspike/spike-interfaces.cc \
      -o libspike/target/spike-interfaces.o

    ar rcs libspike/target/spike-interfaces.a \
      libspike/target/spike-interfaces.o
fi