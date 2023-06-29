!/bin/bash

while getopts "i:d:" opt; do
  case $opt in
    i)
      arg_i=$OPTARG
      ;;
    d)
      arg_d=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

if [[ -z $arg_i || -z $arg_d ]]; then
  echo "Usage: $0 -i <int> -d <int>"
  exit 1
fi

echo "arg_i: $arg_i"
echo "arg_d: $arg_d"

cd wrapper
. make_lib.sh --pic yes
cd ..

unset RUSTFLAGS

module load rust/1.60.0
cargo build --release
cargo run --release opt-level=2 -- -i $arg_i -d $arg_d
