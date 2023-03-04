import numpy as np
import re

summary_filename = "../results/summary.csv"

if __name__ == "__main__":
    with open(summary_filename, 'w') as outf:
        for measurement in range(0, 10):
            task_clock = []
            cpu_util = []
            page_faults = []
            cycles = []
            instructions = []
            branches = []
            branch_misses = []
            githash = ""
            for incr in range(0, 5):
                filename = "../results/{:02d}".format(measurement)+".result_"+str(incr)
                with open(filename, 'r') as f:
                    content = f.read()
                    githash = re.findall("hash: .+\n", content)[0]\
                            .replace("hash: ", "").replace("\n", "")
                    task_clock_str = re.findall("\n.+ msec task-clock", content)[0]\
                            .replace("\n", "").replace(" msec task-clock", "")\
                            .replace(" ", "").replace(",", "")
                    task_clock.append(float(task_clock_str))
                    cpu_util_str = re.findall("#.+CPU", content)[0]\
                            .replace("#", "").replace(" ", "").replace("CPU", "")
                    cpu_util.append(float(cpu_util_str))
                    page_faults_str = re.findall("#.+K/sec", content)[0]\
                            .replace(" ", "").replace("K/sec", "").replace("#", "")
                    page_faults.append(float(page_faults_str))
                    cycles_str = re.findall("#.+GHz", content)[0]\
                            .replace(" ", "").replace("#", "").replace("GHz", "")
                    cycles.append(float(cycles_str))
                    instr_str = re.findall("#.+insn", content)[0]\
                            .replace(" ", "").replace("#", "").replace("insn", "")
                    instructions.append(float(instr_str))
                    branches_str = re.findall("#.+M/sec", content)[0]\
                            .replace(" ", "").replace("#", "").replace("M/sec", "")
                    branches.append(float(branches_str))
                    branch_misses_str = re.findall("#.+%", content)[0]\
                            .replace(" ", "").replace("#", "").replace("%", "")
                    branch_misses.append(float(branch_misses_str))
                    print(filename, githash)
            outf.write(githash + ";")
            outf.write("{:.02f}".format(float(np.average(task_clock))) + ";")
            outf.write("{:.02f}".format(float(np.std(task_clock))) + ";")
            outf.write("{:.02f}".format(float(np.average(cpu_util))) + ";")
            outf.write("{:.02f}".format(float(np.std(cpu_util))) + ";")
            outf.write("{:.02f}".format(float(np.average(page_faults))) + ";")
            outf.write("{:.02f}".format(float(np.std(page_faults))) + ";")
            outf.write("{:.02f}".format(float(np.average(cycles))) + ";")
            outf.write("{:.02f}".format(float(np.std(cycles)))+ ";")
            outf.write("{:.02f}".format(float(np.average(instructions))) + ";")
            outf.write("{:.02f}".format(float(np.std(instructions))) + ";")
            outf.write("{:.02f}".format(float(np.average(branches))) + ";")
            outf.write("{:.02f}".format(float(np.std(branches))) + ";")
            outf.write("{:.02f}".format(float(np.average(branch_misses))) + ";")
            outf.write("{:.02f}".format(float(np.std(branch_misses))) + ";")
            outf.write("\n")

