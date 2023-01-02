import sys
input = sys.stdin.readline

sys.setrecursionlimit(500 ** 2 + 1)

n = int(input().rstrip())
arr = [list(map(int, input().rstrip().split())) for _ in range(n)]

dx = [0,0,1,-1]
dy = [1,-1,0,0]
dp =[[-1 for _ in range(n)] for _ in range(n)]

def inrange(x,y):
    return x >= 0 and x < n and y >= 0 and y < n

def f(x,y):
    if dp[x][y] != -1:
        return dp[x][y]
        
    r = 1
    re = 0
    for i in range(4):
        nx = x + dx[i]
        ny = y + dy[i]
        
        if not inrange(nx, ny):
            continue
        
        if arr[nx][ny] > arr[x][y]:
            re = max(re, f(nx, ny))
    
    r += re
    dp[x][y] = r
    return r

ans = 0

for i in range(n):
    for j in range(n):
        ans = max(ans, f(i,j))
        
print(ans)

# f(x,y) = max(f(동서남북) + 1)