import sys
from bisect import bisect_left

input = sys.stdin.readline
n = int(input())
arr = list(map(int, input().split()))
L = [arr[0]]

for i in range(1, n):
    if arr[i] > L[-1]:
        L.append(arr[i])
        continue

    idx = bisect_left(L, arr[i])
    L[idx] = arr[i]

print(len(L))
print(" ".join(list(map(str, L))))