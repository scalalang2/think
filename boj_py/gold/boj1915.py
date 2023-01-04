import sys
input = sys.stdin.readline

n, m = map(int, input().split())
A = [[0] * m for _ in range(n)]
dp = [[0] * m for _ in range(n)]
ans = 0

for i in range(n):
    t = input().rstrip()
    for j in range(m):
        A[i][j] = int(t[j])

for i in range(n):
    for j in range(m):
        if A[i][j] == 1:
            ans = max(ans, 1)

        if i == 0 or j == 0:
            dp[i][j] = A[i][j]
            continue

        if A[i][j] == 1:
            dp[i][j] = min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]) + 1
            ans = max(ans, dp[i][j])

print(ans ** 2)

