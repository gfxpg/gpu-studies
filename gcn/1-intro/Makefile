ASM=/opt/rocm/hcc/bin/llvm-mc -mattr=+code-object-v3 -triple amdgcn-amd-amdhsa -mcpu=gfx900 -filetype=obj
HSACO=/opt/rocm/hcc/bin/clang -target amdgcn--amdhsa
CXX=/opt/rocm/hcc/bin/clang++ -I/opt/rocm/hsa/include/hsa -std=c++11 -L/opt/rocm/lib -lhsa-runtime64 -lpthread

all:
	$(ASM) test.s -o test.o
	$(HSACO) test.o -o test.co
	$(CXX) test.cpp hsa_runner.cpp hsa_utils.cpp -o test

clean:
	@rm test.o test.co test
