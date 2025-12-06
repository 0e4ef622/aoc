import sys

lines = [l.split() for l in sys.stdin.read().strip().split("\n")]
print([len(l) for l in lines])

ans = 0
for i in range(len(lines[0])):
    op = lines[-1][i]
    opnds = [a[i] for a in lines[:-1]]
    ans += eval(op.join(opnds))

print(ans)
