d = 50
ans = 0
while True:
    try:
        line = input()
    except:
        break
    n = int(line[1:])
    if line[0] == 'L':
        n = -n
    d = (d + n) % 100
    if d == 0:
        ans += 1
print(ans)
