import sys
sys.setrecursionlimit(10 ** 5)
input = sys.stdin.readline

tmp = input().rstrip()
A = []
dp = [-1 for _ in range(len(tmp))]
if len(tmp) == 0 or tmp[0] == '0':
    print(0)
    exit(0)
for i in range(len(tmp)):
    A.append(int(tmp[i]))

# f(pos) = f(pos-1) <- 요고 아님?
# f(pos) += f(pos-2) if 이전 두개의 값이 하나의 유효한 암호인 경우에만 가능하다 이거지
# f(pos-1) <- 여러개인거고
# 흠..
# f(0) = 1
# f(1) = 

dp[0] = 1
dp[1] = 1
def f(pos):
    if dp[pos] != -1:
        return dp[pos]

    if A[pos-1] > 0:
        dp[pos] = f(pos-1) % 1000000
    if pos >= 2:
        r = int(str(A[pos-2]) + str(A[pos-1]))
        if r >= 10 and r <= 26:
            dp[pos] += f(pos-2) % 1000000
    
    return dp[pos] % 1000000

f(len(tmp)-1)
print(dp[len(tmp)-1])