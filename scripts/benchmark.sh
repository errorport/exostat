#!/bin/bash

benchmark_length=60000
hashes=()

cd ..
mkdir -p results
for i in {0..19}
do
    hash=$(git show HEAD~${i} | head -1 | awk '{print $2}')
    hashes+="${hash} "
done

i=0
for hash in ${hashes[@]}
do
    result_file="results/$(printf "%02d" $i).result"
    echo "Benchmark: ${hash} (${i})"
    git checkout $hash
    echo "- build"
    cargo clean 2>&1 >> /dev/null
    ./build.sh 2>&1 >> /dev/null
    echo "- perftest"
    echo "${hash}" >> ${result_file}
    perf stat --timeout ${benchmark_length} target/release/exostat 2>&1 | tee ${result_file}
    echo "- *** -"
    i=$(($i + 1))
done

