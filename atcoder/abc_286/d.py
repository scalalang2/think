import sys
input = sys.stdin.readline

# 일반 DP 동전 문제 이지만 동전에 개수 제한이 존재한다.

N, X = map(int, input().split())
coins = [0] * N
counts = [0] * N

for i in range(N):
    a, b = map(int, input().split())
    coins[i], counts[i] = a, b

dp = [False for _ in range(X+1)]

# 초기화, 0원은 항상 정답이다.
dp[0] = True

# f(n, k) = n 번째 동전으로 k를 만들 수 있는 경우의 수
# f(n, k) = f(n-1, k) + f(n, k-coins[n])
# 근데 이건 동전이 무한한 경우에서 성립하는 식이다.
# 흠..

# 가장 마지막에 채워진 동전 부터 채워나간다.
# 2원 3개로 표현 가능한 건 [0, 2, 4, 6] 임, 그리고 5원 5개로 채운다고 생각하면
# 2원 + 5원 <- 이걸 5번씩 각각 하는거지. 왜냐면 이전 동전으로 이미 채워졌고 그 다음엔 그 다음 동전으로 하는 거니까

for i in range(N):
    a, b = coins[i], counts[i]
    for j in range(X, -1, -1):
        for k in range(1, b+1):
            if j - a * k < 0:
                break
            dp[j] |= dp[j - a * k]

print("Yes" if dp[X] else "No")