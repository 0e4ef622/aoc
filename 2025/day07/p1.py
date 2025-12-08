import sys

inp = sys.stdin.read().split()
w = len(inp[0])
h = len(inp)


p = {inp[0].find("S")}
ans=0
for i in range(1, h):
    p2 = set()
    for j in p:
        if inp[i][j] == "^":
            p2.add(j+1)
            p2.add(j-1)
            ans+=1
        else:
            p2.add(j)
    p = p2
print(ans)
