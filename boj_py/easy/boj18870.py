import sys
input = sys.stdin.readline

n = int(input())
arr = list(map(int, input().split()))
tmp = arr.copy()
tmp.sort()
t = {}
last = 0

def e(num):
    global last
    if num in t:
        return t[num]
    else:
        t[num] = last
        last += 1
        return t[num]

for i in range(n):
    e(tmp[i])

arr = list(map(e, arr))
print(" ".join(map(str, arr)))