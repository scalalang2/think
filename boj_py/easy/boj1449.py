import sys
input = sys.stdin.readline

N, L = map(int, input().split())
arr = list(map(int, input().split()))

arr.sort()
s = 0
cnt = 0
for i in range(N):
    if s == 0:
        s = arr[i]
        cnt += 1
        continue
    if arr[i] <= s + L-1:
        continue
    else:
        s = arr[i]
        cnt += 1

print(cnt)