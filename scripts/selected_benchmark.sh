#!/bin/bash

benchmark_length=60000
hashes=(
    ab3b34b9e7cd348c6b9eac58d344c8e45181bdcc
    ceb96c63d84023e1522324d0d2301fa1b360460d
    85f340c14120024cc34109804f84a8cfbdbaf656
    bcded62a277012e6a19fb595b1760d6cb05c6a6b
    bfb819c8eacf4c2e201de38dc744be89290b6193
    1db79ee8f3ecebefd76a87509512bce8c58e8ef8
    0d7db4f00e44775f117aacf0245a7d571f62a90e
    24a19c0b07d7136b046897b182ccf387e652ba11
    ee6bee7332bb7cdb71ab90a5267693b3ac6bb53c
    2a041bbc3e62d9567e256d745840437d8850c43e
)

cd ..
mkdir -p results
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
    for repeat in {0..4}
    do
        perf stat --timeout ${benchmark_length} target/release/exostat 2>&1 | tee ${result_file}_${repeat}
        echo "hash: ${hash}" >> ${result_file}_${repeat}
    done
    echo "- *** -"
    i=$(($i + 1))
done

