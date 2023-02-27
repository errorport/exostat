import os

if __name__ == "__main__":
    contents = ""
    results = ""
    with open('bench_result_template.svg', 'r') as f:
        contents = f.read()
    with open('results/results.csv', 'r') as f:
        results = f.read().splitlines()
    for i in range(0, 20):
        result = results[i].split(";")
        output_filename = "results/"+'{:02d}'.format(i)+"_result_card.svg"
        content = contents \
            .replace("%%NUM%%", '{:02d}'.format(19 - i)) \
            .replace("%%HASH%%", result[0]) \
            .replace("%%task_clock%%", result[1]) \
            .replace("%%cpu_util%%", result[2]) \
            .replace("%%page_faults%%", result[3]) \
            .replace("%%cycles%%", result[4]) \
            .replace("%%instructions%%", result[5]) \
            .replace("%%branches%%", result[6]) \
            .replace("%%branch_misses%%", result[7])
        with open(output_filename, 'w') as output:
            output.write(content)
        os.system("inkscape --export-type='png' " + output_filename)

