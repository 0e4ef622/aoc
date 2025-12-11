import sys
from z3 import *

inp = [s.strip() for s in sys.stdin.readlines()]

def p2(targ: list[int], buttonl: list[list[int]]) -> int:
    vars = [Int(f"x{i}") for i in range(len(buttonl))]
    eqn = [v >= 0 for v in vars]
    for i, t in enumerate(targ):
        eq = 0
        for bi, b in enumerate(buttonl):
            if i in b:
                eq += vars[bi]
        eqn.append(eq == t)
    s = Optimize()
    s.add(*eqn)
    h = s.minimize(sum(vars))
    s.check()
    s.lower(h)
    m = s.model()
    return sum(m.eval(x).as_long() for x in vars)

ans = 0
for line in inp:
    (_, *targ, _), *bt, joltage = line.split()
    joltage = list(map(int, joltage[1:-1].split(",")))
    buttonl: list[list[int]] = [eval(b.replace("(", "[").replace(")", "]")) for b in bt]

    ans += p2(joltage, buttonl)
    # pri

print(ans)
