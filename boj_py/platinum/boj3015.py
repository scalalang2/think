# 7
# 2
# 4
# 1
# 2
# 2
# 5
# 1

import sys
input = sys.stdin.readline

N = int(input())
stack = []
ans = 0

for _ in range(N):
    x = int(input())
    num = 1

    while len(stack) > 0 and stack[-1][0] <= x:
        if stack[-1][0] == x:
            ans += stack[-1][1]
            num = stack[-1][1] + 1
            stack.pop()
        else:
            ans += stack[-1][1]
            num = 1
            stack.pop()
    
    if len(stack) > 0:
        ans += 1
    
    stack.append([x, num])

print(ans)