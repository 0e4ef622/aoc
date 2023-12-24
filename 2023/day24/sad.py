from sympy import solve, symbols, Integer
import sys

input = [[[*map(int, s.split(", "))] for s in l.split(" @ ")] for l in sys.stdin.read().splitlines()]

t1, t2, t3 = symbols("t1 t2 t3")
# x1, x2, x3 = symbols("x1 x2 x3")
# y1, y2, y3 = symbols("y1 y2 y3")
# z1, z2, z3 = symbols("z1 z2 z3")

# vx1, vx2, vx3 = symbols("vx1 vx2 vx3")
# vy1, vy2, vy3 = symbols("vy1 vy2 vy3")
# vz1, vz2, vz3 = symbols("vz1 vz2 vz3")

x1, y1, z1 = input[0][0]
x2, y2, z2 = input[1][0]
x3, y3, z3 = input[2][0]
vx1, vy1, vz1 = input[0][1]
vx2, vy2, vz2 = input[1][1]
vx3, vy3, vz3 = input[2][1]

p1x = x1 + t1*vx1
p1y = y1 + t1*vy1
p1z = z1 + t1*vz1
p2x = x2 + t2*vx2
p2y = y2 + t2*vy2
p2z = z2 + t2*vz2
p3x = x3 + t3*vx3
p3y = y3 + t3*vy3
p3z = z3 + t3*vz3

eqs = [
    (p2x-p1x)*(t3-t2) - (p3x-p2x)*(t2-t1),
    (p2y-p1y)*(t3-t2) - (p3y-p2y)*(t2-t1),
    (p2z-p1z)*(t3-t2) - (p3z-p2z)*(t2-t1),
]

sol = solve(eqs, [t1, t2, t3])
print(sol)
for s in sol:
    if all(isinstance(n, Integer) for n in s):
        t1, t2, t3 = s
v = [
    (x1 + vx1*t1 - (x2 + vx2*t2)) / (t1 - t2),
    (y1 + vy1*t1 - (y2 + vy2*t2)) / (t1 - t2),
    (z1 + vz1*t1 - (z2 + vz2*t2)) / (t1 - t2),
]
p = [
    x1 + vx1*t1 - v[0]*t1,
    y1 + vy1*t1 - v[1]*t1,
    z1 + vz1*t1 - v[2]*t1,
]
print(p, v)
print(sum(p))
