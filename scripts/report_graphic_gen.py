import os

if __name__ == "__main__":
    contents = ""
    results = ""
    with open('bench_result_template.svg', 'r') as f:
        contents = f.read()
    with open('../results/summary.csv', 'r') as f:
        results = f.read().splitlines()
    for i in range(0, 9):
        result = results[i].split(";")
        output_filename = '{:02d}'.format(i)+"_result_card.svg"
        content = contents \
            .replace("%%NUM%%", '{:02d}'.format(9 - i)) \
            .replace("%%HASH%%", result[0]) \
            .replace("%%task_clock%%", result[1]) \
            .replace("%%std_task_clock%%", result[2]) \
            .replace("%%cpu_util%%", result[3]) \
            .replace("%%std_cpu_util%%", result[4]) \
            .replace("%%page_faults%%", result[5]) \
            .replace("%%std_page_faults%%", result[6]) \
            .replace("%%cycles%%", result[7]) \
            .replace("%%std_cycles%%", result[8]) \
            .replace("%%instructions%%", result[9]) \
            .replace("%%std_instructions%%", result[10]) \
            .replace("%%branches%%", result[11]) \
            .replace("%%std_branches%%", result[12]) \
            .replace("%%branch_misses%%", result[13]) \
            .replace("%%std_branch_misses%%", result[14])
        with open(output_filename, 'w') as output:
            output.write(content)
        os.system("inkscape --export-type='png' " + output_filename)
