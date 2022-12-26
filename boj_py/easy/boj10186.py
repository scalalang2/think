import sys
input = sys.stdin.readline

N = int(input())
arr = list(map(int, input().split()))
maps = {}

for i in range(N):
    if arr[i] not in maps:
        maps[arr[i]] = 1
    else:
        maps[arr[i]] += 1

M = int(input())
As = list(map(int, input().split()))
ans = []

for i in range(M):
    if As[i] not in maps:
        ans.append(0)
    else:
        ans.append(maps[As[i]])

print(" ".join(list(map(str, ans))))