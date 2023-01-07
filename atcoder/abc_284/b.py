import sys
input = sys.stdin.readline

# 4
# 3
# 1 2 3
# 2
# 20 23
# 10
# 6 10 4 1 5 9 8 6 5 1
# 1
# 1000000000

T = int(input())
for i in range(T):
    n = int(input())
    arr = list(map(int, input().split()))
    ans = 0
    for i in range(n):
        if arr[i] % 2 == 1:
            ans += 1
    print(ans)