#!/bin/bash

while getopts "l:f:o:w:t:" opt
do
	echo "-$opt $OPTARG"
	case "$opt" in
	l) line=$OPTARG ;;
	f) file=$OPTARG ;;
	o) debug_out_path=$OPTARG ;;
	w) watches=$OPTARG ;;
	t) counter=$OPTARG ;;
	esac
done

rm -rf tmp/
mkdir tmp

export ASM_DBG_CONFIG=tmp/config.toml
cat <<EOF > tmp/config.toml
[agent]
log = "-"

[debug-buffer]
size = 1048576
dump-file = "tmp/debug_buffer"

[code-object-dump]
log = "-"
directory = "tmp/"

[[code-object-swap]]
when-call-count = 1
load-file = "tmp/replacement.co"
exec-before-load = """bash -o pipefail -c '\
  perl include/breakpoint.pl -ba \$ASM_DBG_BUF_ADDR -bs \$ASM_DBG_BUF_SIZE \
    -l $line -w "$watches" -s 96 -r s0 -t $counter $file \
  | /opt/rocm/opencl/bin/x86_64/clang -x assembler -target amdgcn--amdhsa -mcpu=gfx900 -mno-code-object-v3 \
    -I./include -o tmp/replacement.co -'"""
EOF

# Path to the compiled https://github.com/vsrad/debug-plug-hsa-intercept
export HSA_TOOLS_LIB=../../../hsa/build/src/libplugintercept.so
make host
# Ignore return code (supress validation errors caused by the kernel being aborted at a breakpoint)
./max_pooling || true
