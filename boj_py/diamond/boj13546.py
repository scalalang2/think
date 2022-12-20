import sys
from collections import deque
input = sys.stdin.readline

N, K = map(int, input().split())
sz = int(N ** 0.5)
arr = [0 for _ in range(N+1)]
for i, x in enumerate(list(map(int, input().split()))):
    arr[i+1] = x


cnt = [0] * (N+1)
bucket = [0] * (300)
pos = [deque() for _ in range(K+1)]
M = int(input())

LEFT = 0
RIGHT = 1

def plus(x, dir):
    dq = pos[arr[x]]
    if len(dq) != 0:
        prev = dq[-1] - dq[0]
        cnt[prev] -= 1
        bucket[prev//sz] -= 1
    if dir == LEFT :
        dq.appendleft(x)
    else:
        dq.append(x)

    now = dq[-1] - dq[0]
    cnt[now] += 1
    bucket[now//sz] += 1

def minus(x, dir):
    dq = pos[arr[x]]
    prev = dq[-1] - dq[0]
    cnt[prev] -= 1
    bucket[prev//sz] -= 1
    if dir == LEFT :
        dq.popleft()
    else:
        dq.pop()
    
    if len(dq) != 0:
        now = dq[-1] - dq[0]
        cnt[now] += 1
        bucket[now//sz] += 1

def query():
    for i in range(len(bucket)-1, -1, -1):
        if bucket[i] == 0: 
            continue
        for j in range(sz-1, -1, -1):
            if cnt[i*sz+j] > 0:            
                return i*sz+j

queries = []
for i in range(M):
    L, R = map(int, input().split())
    queries.append((L, R, i))

queries.sort(key=lambda x: (x[0]//sz, x[1]))

ans = [0] * M
currL, currR, x = queries[0][0], queries[0][1], queries[0][2]
for i in range(currL, currR+1):
    plus(i, RIGHT)

ans[x] = query()

for i in range(1, M):
    l, r, x = queries[i][0], queries[i][1], queries[i][2]
    while currL < l:
        minus(currL, LEFT)
        currL += 1
    while currL > l:
        currL -= 1
        plus(currL, LEFT)
    while currR < r:
        currR += 1
        plus(currR, RIGHT)
    while currR > r:
        minus(currR, RIGHT)
        currR -= 1
    
    x = query()

for i in range(M):
    print(ans[i])