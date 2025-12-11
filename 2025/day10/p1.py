from collections import deque
import sys

inp = [s.strip() for s in sys.stdin.readlines()]

def p1(targs: list[str], buttons: list[int]):
    targ = 0
    for ch in targs[::-1]:
        targ <<= 1
        if ch == "#":
            targ += 1

    visited = {0: 0}
    q = deque([0])
    while q:
        state = q.popleft()
        if state == targ:
            break
        for b in buttons:
            s2 = state ^ b
            if s2 not in visited:
                visited[s2] = visited[state] + 1
                q.append(s2)
    return visited[targ]

ans = 0
for line in inp:
    (_, *targ, _), *bt, _ = line.split()
    buttonl: list[list[int]] = [eval(b.replace("(", "[").replace(")", "]")) for b in bt]
    buttons = []
    for b in buttonl:
        n = 0
        for i in b:
            n |= 1<<i
        buttons.append(n)

    ans += p1(targ, buttons)

print(ans)
