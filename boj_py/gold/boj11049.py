# 행렬 곱셈 (m x k) (k x n) = m x n 이 나옴
# 행렬 곱센의 개수 A = (m x k), B = (k x n) 두 행렬의 곱셈은 m x k x n 이 된다.

# ABC 세 개의 행렬의 곱셈 순서를 따져보다
# (AB)C 혹은 A(BC) 가 됨
# ABCD 라면?
# ((AB)C)D, A((BC)D), (A(BC))D, A(B(CD)) 로만 가능함
# 여기서 AB와 BC, CD는 중복되서 나타나므로 메모이제이션 가능해진다.

# f(i, j) = i ~ j 구간 사이의 곱셈 최솟값
# f(i, j)
# --> if i + 1 == j 라면 f(i,j) = arr[i].r x arr[i].c x arr[j].c
# --> 아니라면, (f(i+1, j) + arr[i].r x arr[i].c x arr[j].c), (f(i, j-1) + arr[i].r x arr[j].r x arr[j].c)

# final
# 구간에 대해서 조금 더 쪼개야한다.
# f(i,j) = f(i,k) + f(k+1,j) + A[i].row * A[k].col * A[j].col

import sys
import math
input = sys.stdin.readline

n = int(input())
A = []
dp = [[math.inf for _ in range(n)] for _ in range(n)]
for i in range(n):
    r,c = map(int, input().split())
    A.append((r,c))

def f(i,j):
    if dp[i][j] != math.inf:
        return dp[i][j]

    if i >= j:
        return 0
    
    if i + 1 == j:
        dp[i][j] = A[i][0] * A[i][1] * A[j][1]
        return dp[i][j]

    for k in range(i, j):
        dp[i][j] = min(dp[i][j], f(i, k) + f(k+1, j) + A[i][0] * A[k][1] * A[j][1])

    return dp[i][j]

if n == 1:
    print(0)
elif n == 2:
    print(A[0][0] * A[0][1] * A[1][1])
else:
    print(f(0, n-1))