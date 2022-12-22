import sys
from collections import deque
input = sys.stdin.readline

while True:
    L, R, C = map(int, input().split())
    if L == 0 and R == 0 and C == 0:
        break

    bb = [[[0 for _ in range(C)] for _ in range(R)] for _ in range(L)]
    visited = [[[False for _ in range(C)] for _ in range(R)] for _ in range(L)]

    def inrange(nl, nr, nc):
        return 0 <= nl < L and 0 <= nr < R and 0 <= nc < C

    dir = [(1, 0,0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]
    sl, sr, sc = -1, -1, -1
    el, er, ec = -1, -1, -1

    for l in range(L):
        for i in range(R):
            tmp = input().rstrip()
            for j in range(C):
                if tmp[j] == 'S':
                    sl, sr, sc = l, i, j
                    continue
                
                if tmp[j] == 'E':
                    el, er, ec = l, i, j
                    continue

                bb[l][i][j] = tmp[j]

        input().rstrip()

    queue = deque([])
    queue.appendleft((sl, sr, sc, 0))
    ans = -1

    while len(queue) > 0:
        cl, cr, cc, cost = queue.popleft()
        if cl == el and cr == er and cc == ec:
            ans = cost
            break

        if visited[cl][cr][cc]:
            continue

        visited[cl][cr][cc] = True
        for i in range(len(dir)):
            nl, nr, nc = dir[i]
            nl, nr, nc = cl + nl, cr + nr, cc + nc
            if inrange(nl, nr, nc) and not visited[nl][nr][nc] and bb[nl][nr][nc] != '#':
                queue.append((nl, nr, nc, cost + 1))

    if ans != -1:
        print("Escaped in {} minute(s).".format(ans))
    else:
        print("Trapped!")