#!/usr/bin/python
from z3 import *

# example problem:
# https://xkcd.com/287/

(
    mixed_fruit,
    french_fries,
    side_salad,
    hot_wings,
    mozarella_sticks,
    sampler_plate,
) = Ints("mixed_fruit french_fries side_salad hot_wings mozarella_sticks sampler_plate")

s = Solver()

s.add(mixed_fruit >= 0)
s.add(french_fries >= 0)
s.add(side_salad >= 0)
s.add(hot_wings >= 0)
s.add(mozarella_sticks >= 0)
s.add(sampler_plate >= 0)

# constraints:
s.add(
    (215 * mixed_fruit)
    + (275 * french_fries)
    + (335 * side_salad)
    + (355 * hot_wings)
    + (420 * mozarella_sticks)
    + (580 * sampler_plate)
    == 1505
)

# TODO: In what order does s.check() return its models?

# traverse through each solution:
while s.check() == sat:
    mod = s.model()

    print("model solution:\n{}".format(mod))  # print each solution
    print()

    # if we want to debug the values of the given variable present in
    # the model:

    to_print = [
        (x, mod[x])
        for x in [
            mixed_fruit,
            french_fries,
            side_salad,
            hot_wings,
            mozarella_sticks,
            sampler_plate,
        ]
        # mod.eval(x) evaluates the expression. This is equivalent to
        # `if mod[x].as_long() > 0`
        if mod.eval(x > 0)
    ]
    for (symbol, num) in to_print:
        print("{} of {}".format(num, symbol))

    # add a new condition so that the next .check() iteration skips
    # the current solution:
    s.add(
        Or(
            [
                mod[mixed_fruit] != mixed_fruit,
                mod[french_fries] != french_fries,
                mod[side_salad] != side_salad,
                mod[hot_wings] != hot_wings,
                mod[mozarella_sticks] != mozarella_sticks,
                mod[sampler_plate] != sampler_plate,
            ]
        )
    )
    print()
