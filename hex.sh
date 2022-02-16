# takes one argument: the 8080 binary.
# produces a hexadecimal representation in the proper format for the disassembler.
#   all output of this script is in /intermediates/
bin=$1
filename=$(basename ${bin})

mkdir -p intermediates
out_file=./intermediates/${filename}.x

if test -f "$out_file"; then
  echo "FATAL: file $out_file already exists. Not overwriting."
else
  od -x --endian big ${bin} > ${out_file}
fi
