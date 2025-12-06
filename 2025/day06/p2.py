import sys

lines = [l for l in sys.stdin.read().strip().split("\n")]
opnds = lines[:-1]

opnds2 = ["" for _ in opnds[0]]
for i, r in enumerate(opnds):
    for j, c in enumerate(r):
        opnds2[j] += c
for i in range(len(opnds2)):
    opnds2[i] = opnds2[i].strip()

aids = [[]]
for o in opnds2:
    if o:
        aids[-1].append(o)
    else:
        aids.append([])

ops = lines[-1].split()

ans = 0
ll = len(lines)-1
for i in range(len(ops)):
    op = ops[i]
    e = op.join(aids[i])
    ans += eval(e)

print(ans)
