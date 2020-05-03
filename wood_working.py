#!/usr/bin/python
from z3 import *

s = Optimize()
workpieces_total = Int("workpieces_total")
cut_A, cut_B, cut_C, cut_D = Ints("cut_A cut_B cut_C cut_D")
out_A, out_B = Ints("out_A out_B")

s.add(workpieces_total == cut_A + cut_B + cut_C + cut_D)
s.add(cut_A >= 0)
s.add(cut_B >= 0)
s.add(cut_C >= 0)
s.add(cut_D >= 0)

s.add(out_A == cut_A * 3 + cut_B * 2 + cut_C * 1)
s.add(out_B == cut_A * 1 + cut_B * 6 + cut_C * 9 + cut_D * 13)
s.add(out_A == 800)
s.add(out_B == 400)
s.minimize(workpieces_total)

print(s.check())
print(s.model())
