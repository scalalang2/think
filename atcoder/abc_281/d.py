import sys
input = sys.stdin.readline

n, K, D = map(int, input().split())
arr = list(map(int, input().split()))

# dp[i][j][k] = i 번째 수를 포함해야 하고, j 개의 수를 골라서 합했을 때 d로 나누어 떨어지는 것 중 가장 큰 값
dp = [[[-1] * D for _ in range(K + 1)] for _ in range(n + 1)]
dp[0][0][0] = 0

for i in range(0, n):
    for j in range(0, K + 1):
        for k in range(0, D):
            if dp[i][j][k] == -1:
                continue

            # a_i 는 고르지 않는 경우
            # 이전 상태에서 값을 가져와서 비교한다
            dp[i+1][j][k] = max(dp[i+1][j][k], dp[i][j][k])

            if j != K:
                # a_i 를 고르는 경우
                dp[i+1][j+1][(k+arr[i])%D] = max(dp[i+1][j+1][(k+arr[i])%D], dp[i][j][k] + arr[i]);

print(dp[n][K][0])