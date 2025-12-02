def f(a, b):
    if a < b:
        a = a // 100
        b = b // 100
    else:
        a = (a - 1) // 100
        b = (b - 1) // 100
    return abs(b - a)

if __name__ == "__main__":
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

        ans += f(d, d+n)
        d = (d + n) % 100

    print(ans)
