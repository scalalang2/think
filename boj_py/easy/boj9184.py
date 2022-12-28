import sys
input = sys.stdin.readline

dp = [[[-1 for _ in range(21)] for _ in range(21)] for _ in range(21)]

def calc(v):
    if v <= 0:
        return 0
    elif v > 20:
        return 20
    else:
        return v

def w(a, b, c):
    x,y,z = calc(a), calc(b), calc(c)

    if dp[x][y][z] != -1:
        return dp[x][y][z]

    if a <= 0 or b <= 0 or c <= 0:
        dp[x][y][z] = 1
        return 1
    
    if a > 20 or b > 20 or c > 20:
        dp[x][y][z] = w(20, 20, 20)
        return dp[x][y][z]
    
    if a < b and b < c:
        dp[x][y][z] = w(a,b,c-1) + w(a,b-1,c-1) - w(a, b-1,c)
        return dp[x][y][z]
    
    dp[x][y][z] =  w(a-1, b, c) + w(a-1, b-1, c) + w(a-1, b, c-1) - w(a-1, b-1, c-1)
    return dp[x][y][z]

while True:
    x,y,z = map(int, input().split())
    if x == -1 and y == -1 and z == -1:
        break

    dp = [[[-1 for _ in range(21)] for _ in range(21)] for _ in range(21)]
    print("w({}, {}, {}) = {}".format(x,y,z, w(x,y,z)))