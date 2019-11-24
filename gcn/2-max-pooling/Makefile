ASM=/opt/rocm/hcc/bin/clang -x assembler -target amdgcn--amdhsa -mcpu=gfx900 -mno-code-object-v3 -I./include
CXX=/opt/rocm/hcc/bin/clang++ -I/opt/rocm/hsa/include/hsa -std=c++11 -L/opt/rocm/lib -lhsa-runtime64 -lpthread

all: asm host

asm: max_pooling.s
	$(ASM) max_pooling.s -o max_pooling.co

host: *.cpp *.hpp
	$(CXX) max_pooling.cpp hsa_runner.cpp hsa_utils.cpp -o max_pooling

clean:
	@rm max_pooling.co max_pooling
