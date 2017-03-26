import subprocess
import re

def print_list(xs):
    for x in xs:
        print x.rstrip()

args = ["cargo", "run", "--release", "--", "roms/cpu_instrs.gb"]
popen = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
(stdout, stderr) = popen.communicate()

ref_file = open("roms/cpu_instrs.log")

last_five = []
for i, (ref, act) in enumerate(zip(ref_file, stdout.split("\n"))):
    last_five.append(ref.rstrip())
    if len(last_five) > 5:
        last_five.pop(0)
    ref_groups = re.match(r"\s*(\d+):\s*(.*)PC:\s([0-fx]+)\s*AF:\s([0-fx]+)\s*BC:\s([0-fx]+)\s*DE:\s([0-fx]+)\s*HL:\s([0-fx]+)\s*SP:\s([0-fx]+)", ref.strip())
    act_groups = re.match(r"(.*)PC:\s([0-fx]+)\s*AF:\s([0-fx]+)\s*BC:\s([0-fx]+)\s*DE:\s([0-fx]+)\s*HL:\s([0-fx]+)\s*SP:\s([0-fx]+)", act.strip())

    if act_groups is None:
        print_list(last_five)
        print "Missing output from GBrs at", ref_groups.group(1)
        break
    if ref_groups.group(2).strip() != act_groups.group(1).strip():
        print_list(last_five)
        print "Mismatched opcode at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(3).strip() != act_groups.group(2).strip():
        print_list(last_five)
        print "Mismatched PC at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(4).strip() != act_groups.group(3).strip():
        print_list(last_five)
        print "Mismatched AF at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(5).strip() != act_groups.group(4).strip():
        print_list(last_five)
        print "Mismatched BC at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(6).strip() != act_groups.group(5).strip():
        print_list(last_five)
        print "Mismatched DE at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(7).strip() != act_groups.group(6).strip():
        print_list(last_five)
        print "Mismatched HL at", ref_groups.group(1)
        print act.strip()
        break
    if ref_groups.group(8).strip() != act_groups.group(7).strip():
        print_list(last_five)
        print "Mismatched SP at", ref_groups.group(1)
        print act.strip()
        break

print ""
print stderr
