# abcd
# 3
# P x
# L
# P y

# P <- 추가
# B <- 삭제
# L <- 왼쪽 이동
# D <- 오른쪽 이동

import sys
input = sys.stdin.readline

txt = input().rstrip()
m = int(input())

lstack = []
rstack = []

for i in range(len(txt)):
    lstack.append(txt[i])

for i in range(m):
    commands = list(map(str, input().split()))
    if commands[0] == 'P':
        lstack.append(commands[1])
    elif commands[0] == 'B':
        if len(lstack) > 0:
            lstack.pop()
    elif commands[0] == 'L':
        if len(lstack) > 0:
            last = lstack.pop()
            rstack.append(last)
    else:
        if len(rstack) > 0:
            last = rstack.pop()
            lstack.append(last)

ans = ""
ans += "".join(lstack)
rstack.reverse()
ans += "".join(rstack)

print(ans)