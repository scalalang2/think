import sys
input = sys.stdin.readline

A = input().rstrip()
B = input().rstrip()
C = input().rstrip()

# f(i,j,k) = 각 i,j,k 문자열이 서로 같을때의 LCS, 그러면 그 이전 문자도 동일해야 한다.
ans = 0
dp = [[[-1 for _ in range(101)] for _ in range(101)] for _ in range(101)]
def f(i,j,k):
    global ans
    if dp[i][j][k] != -1:
        return dp[i][j][k]
    
    if i < 0 or j < 0 or k < 0:
        return 0
    
    if A[i] == B[j] == C[k]:
        dp[i][j][k] = f(i-1,j-1,k-1) + 1
        ans = max(ans, dp[i][j][k])
        return dp[i][j][k]
    
    dp[i][j][k] = max(dp[i][j][k], f(i-1,j,k))
    dp[i][j][k] = max(dp[i][j][k], f(i,j-1,k))
    dp[i][j][k] = max(dp[i][j][k], f(i,j,k-1))
    ans = max(ans, dp[i][j][k])
    return dp[i][j][k]

for i in range(len(A)):
    for j in range(len(B)):
        for k in range(len(C)):
            f(i,j,k)

print(ans)