import sys
input = [[[*map(int, s.split(", "))] for s in l.split(" @ ")] for l in sys.stdin.read().splitlines()]

# 3 rocks collide with an unknown 4th rock. Calculate the time of collision for the first rock.
def solve_t1(rocks):
    (x1, y1, z1), (vx1, vy1, vz1) = rocks[0]
    (x2, y2, z2), (vx2, vy2, vz2) = rocks[1]
    (x3, y3, z3), (vx3, vy3, vz3) = rocks[2]
    yz = y1*(z2 - z3) + y2*(-z1 + z3) + y3*(z1 - z2)
    xz = x1*(-z2 + z3) + x2*(z1 - z3) + x3*(-z1 + z2)
    xy = x1*(y2 - y3) + x2*(-y1 + y3) + x3*(y1 - y2)
    vxvy = vx1*(vy2 - vy3) + vx2*(-vy1 + vy3) + vx3*(vy1 - vy2)
    vxvz = vx1*(-vz2 + vz3) + vx2*(vz1 - vz3) + vx3*(-vz1 + vz2)
    vyvz = vy1*(vz2 - vz3) + vy2*(-vz1 + vz3) + vy3*(vz1 - vz2)

    n = (vx2 - vx3)*yz + (vy2 - vy3)*xz + (vz2 - vz3)*xy
    d = (z2 - z3)*vxvy + (y2 - y3)*vxvz + (x2 - x3)*vyvz
    return n / d

t1 = solve_t1(input)
t2 = solve_t1([input[1], input[0], input[2]])

c1 = [x + v*t1 for x, v in zip(*input[0])]
c2 = [x + v*t2 for x, v in zip(*input[1])]
v = [(x2 - x1) / (t2 - t1) for x1, x2 in zip(c1, c2)]
p = [x + pv*t1 - vv*t1 for x, pv, vv in zip(*input[0], v)]
print(sum(p))
