from collections import defaultdict
from functools import cache
import sys

inp = sys.stdin.read().strip().split("\n")

g = defaultdict(list, ((lambda w: (w[0][:-1], w[1:]))(l.split()) for l in inp))


@cache
def count_paths(node: str, to: str) -> int:
    if node == to:
        return 1
    r = 0
    for ch in g[node]:
        r += count_paths(ch, to)
    return r

print("Part 1:", count_paths("you", "out"))
print(
    "Part 2:",
    count_paths("svr", "dac") * count_paths("dac", "fft") * count_paths("fft", "out")
    + count_paths("svr", "fft") * count_paths("fft", "dac") * count_paths("dac", "out")
)
