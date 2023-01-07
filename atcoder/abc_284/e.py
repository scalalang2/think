import sys
sys.setrecursionlimit(10**6)

input = sys.stdin.readline

MAX = 10 ** 6

n, m = map(int, input().split())
adj = [set() for _ in range(n+1)]
visited = [False for _ in range(n+1)]
for _ in range(m):
    u,v = map(int, input().split())
    if u == v:
        continue
    adj[u].add(v)
    adj[v].add(u)

acc = 0
def dfs(node):
    global acc
    acc += 1
    if acc >= MAX:
        print(MAX)
        exit(0)
 
    for nxt in adj[node]:
        if not visited[nxt]:
            visited[nxt] = True
            dfs(nxt)
            visited[nxt] = False

# def dfs(node):
#     global acc
#     stack = [node]
#     while stack:
#         node = stack.pop()
#         acc += 1
#         if acc >= MAX:
#             print(MAX)
#             exit(0)
#         for nxt in adj[node]:
#             if not visited[nxt]:
#                 visited[nxt] = True
#                 stack.append(nxt)
#                 visited[nxt] = False

visited[1] = True
dfs(1)

print(acc)