#!/bin/bash

cd ..
for i in {1..21}
do
    hash=$(git show HEAD~${i} | head -1 | awk '{print $2}')
    avg_task_clock=0
    avg_cpu_util=0
    avg_page_faults=0
    avg_cycles=0
    avg_instructions=0
    avg_branches=0
    avg_branch_miss=0
    for iter in {0..4}
    do
        result_file="results/$(printf "%02d" $(($i-1))).result_${iter}"
        task_clock=$(cat ${result_file} | grep "task-clock" | awk '{print $1}')
        avg_task_clock=$((${task_clock} + ${avg_task_clock}))
        cpu_util=$(cat ${result_file} | grep "CPUs" | awk '{print $5}')
        avg_cpu_util=$((${cpu_util} + ${avg_cpu_util}))
        page_faults=$(cat ${result_file} | grep "page-faults" | awk '{print $4}')
        avg_page_faults=$((${page_faults} + ${avg_page_faults}))
        cycles=$(cat ${result_file} | grep "cycles" | awk '{print $4}')
        avg_cycles=$((${cycles} + ${avg_cycles}))
        instructions=$(cat ${result_file} | grep "instructions" | awk '{print $4}')
        avg_instructions=$((${instructions} + ${avg_instructions}))
        branches=$(cat ${result_file} | grep "branches:u" | awk '{print $4}')
        avg_branches=$((${branches} + ${avg_branches}))
        branch_miss=$(cat ${result_file} | grep "branch-miss" | awk '{print $4}')
        avg_branch_miss=$((${branch_miss} + ${avg_branch_miss}))
    done
    avg_task_clock=$((avg_task_clock / 5))
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
