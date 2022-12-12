import math
 
A, B = map(int, input().split(' '))
 
MAX = 10 ** 18
 
def f(x):
    return B * x + A / math.sqrt(x + 1)
 
def solve():
    # 이분 탐색
    left = 0
    right = MAX
    while left <= right:
        mid = (left + right) // 2
        if f(mid) > f(mid + 1):
            left = mid + 1
        else:
            right = mid - 1
 
    return f(left)
 
ans = solve()
print(ans)