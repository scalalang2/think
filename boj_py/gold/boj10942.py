import sys
input = sys.stdin.readline

N = int(input())
A = list(map(int, input().split()))
M = int(input())

TRUE = 1
FALSE = 0

dp = [[-1 for _ in range(2001)] for _ in range(2001)]

def f(s,e):
    if s == e or s > e:
        return TRUE

    if dp[s][e] != -1:
        return dp[s][e]
    
    dp[s][e] = TRUE if A[s-1] == A[e-1] and f(s+1, e-1) else FALSE
    return dp[s][e]

for i in range(M):
    S, E = map(int, input().split())
    # f(S,E) = A[S] ~ A[E] 사이의 구간이 펠린드롬인가? 아닌가?
    # f(S,E) = S == E and f(S-1, E-1) 이면, 펠린드롬

    print(f(S,E))