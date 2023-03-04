#!/bin/bash

benchmark_length=60000
hashes=()

cd ..
mkdir -p results
for i in {0..25}
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
    for repeat in {0..1}
    do
        perf stat --timeout ${benchmark_length} target/release/exostat 2>&1 | tee ${result_file}_${repeat}
        echo "hash: ${hash}" >> ${result_file}
    done
    echo "- *** -"
    i=$(($i + 1))
done

