import sys
input = sys.stdin.readline

# 3
# 1 6 4 3 5 2 7

k = int(input())
R = list(map(int, input().split()))
tree = [[] for _ in range(k)]

def f(lv: int, start: int, end: int):
    if start >= end:
        return

    mid = (start + end) // 2
    tree[lv].append(R[mid])

    f(lv + 1, start, mid)
    f(lv + 1, mid + 1, end)

f(0, 0, len(R))

for t in tree:
    print(" ".join(map(str, t)))