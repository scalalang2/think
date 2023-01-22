import sys
import math
from collections import deque
input = sys.stdin.readline

n, a, b = map(int, input().split())
S = input().rstrip()
ans = math.inf

Q = deque()
for i in range(n):
    Q.append(S[i])

def is_palindrome(Q):
    for i in range(n//2):
        if Q[i] != Q[n-i-1]:
            return False
    return True

if is_palindrome(Q):
    print(0)
else:
    cost = 0
    for j in range(n//2):
        if Q[j] != Q[n-j-1]:
            cost += b
    ans = min(ans, cost)

    init = a
    for i in range(n//2):
        cost = init
        init += a
        x = Q.popleft()
        Q.append(x)
        for j in range(n//2):
            if Q[j] != Q[n-j-1]:
                cost += b
        ans = min(ans, cost)

    print(ans)