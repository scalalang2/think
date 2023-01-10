import sys
import math
input = sys.stdin.readline

n = int(input())
A = [list(map(int, input().split())) for _ in range(n)]
visited = [False for _ in range(n)]
ans = math.inf

def dfs(cnt, idx):
    global ans
    if cnt == n // 2:
        team_a, team_b = 0, 0
        for i in range(n):
            for j in range(n):
                if visited[i] and visited[j]:
                    team_a += A[i][j]
                elif not visited[i] and not visited[j]:
                    team_b += A[i][j]
        ans = min(ans, abs(team_a - team_b))
        return
    
    # 0번 부터 N명까지의 사람을 순회한다.
    for i in range(idx, n):
        # 방문하지 않았다면 방문 시켜 버린다.
        if not visited[i]:
            visited[i] = True
            dfs(cnt+1, i+1)
            visited[i] = False

dfs(0, 0)

print(ans)