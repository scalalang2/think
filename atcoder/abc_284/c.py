import sys
input = sys.stdin.readline

# 5 3
# 1 2
# 1 3
# 4 5

n, m = map(int, input().split())
adj = [[] for _ in range(n+1)]
visited = [False for _ in range(n+1)]

for i in range(m):
    u,v = map(int, input().split())
    adj[u].append(v)
    adj[v].append(u)

def dfs(curr):
    for nxt in adj[curr]:
        if not visited[nxt]:
            visited[nxt] = True
            dfs(nxt)

ans = 0
for i in range(1, n+1):
    if not visited[i]:
        ans += 1
        visited[i] = True
        dfs(i)

print(ans)
    