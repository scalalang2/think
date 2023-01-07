import sys
input = sys.stdin.readline

# 3
# Takahashi
# Aoki
# Snuke

n = int(input())
arr = []

for i in range(n):
    arr.append(input().rstrip())

arr.reverse()

for i in range(n):
    print(arr[i])