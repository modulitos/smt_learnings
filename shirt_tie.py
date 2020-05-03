from z3 import *


# The result should be:
# tie: false
# shirt: true
Tie, Shirt = Bools("Tie Shirt")
s = Solver()
s.add(Or(Tie, Shirt), Or(Not(Tie), Shirt), Or(Not(Tie), Not(Shirt)))
print(s.check())
print(s.model())
