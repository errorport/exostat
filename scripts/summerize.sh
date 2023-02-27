#!/bin/bash

for i in {0..20}
do
    hash=$(git show HEAD~${i} | head -1 | awk '{print $2}')
    result_file="results/$(printf "%02d" $i).result"
    task_clock=$(cat ${result_file} | grep "task-clock" | awk '{print $1}')
    cpu_util=$(cat ${result_file} | grep "CPUs" | awk '{print $5}')
    page_faults=$(cat ${result_file} | grep "page-faults" | awk '{print $4}')
    cycles=$(cat ${result_file} | grep "cycles" | awk '{print $4}')
    instructions=$(cat ${result_file} | grep "instructions" | awk '{print $4}')
    branches=$(cat ${result_file} | grep "branches:u" | awk '{print $4}')
    branch_miss=$(cat ${result_file} | grep "branch-miss" | awk '{print $4}')
    echo "hash: ${hash}"
    echo "task_clock: ${task_clock}"
    echo "cpu_util: ${cpu_util}"
    echo "page_faults: ${page_faults}"
    echo "cycles: ${cycles}"
    echo "instructions: ${instructions}"
    echo "branches: ${branches}"
    echo "branch_misses: ${branch_miss}"
    echo "${hash}; ${task_clock}; ${cpu_util}; ${page_faults}; ${cycles}; ${instructions}; ${branches}; ${branch_miss}" >> results/results.csv
done
