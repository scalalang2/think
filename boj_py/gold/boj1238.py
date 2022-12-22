import sys
import math
from heapq import heappush, heappop

input = sys.stdin.readline
N, M, X = map(int, input().split())
adj = [[] for _ in range(N+1)]

for i in range(M):
    a, b, c = map(int, input().split())
    adj[a].append((b, c))

def dijkstra(x):
    queue = []
    dist = [math.inf for _ in range(N+1)]
    visited = [False for _ in range(N+1)]
    dist[x] = 0
    heappush(queue, (0, x))

    while len(queue) > 0:
        cost, now = heappop(queue)
        if visited[now]:
            continue

        visited[now] = True
        for next, d in adj[now]:
            if dist[next] > dist[now] + d:
                dist[next] = dist[now] + d
                heappush(queue, (dist[next], next))

    return dist

from_x = dijkstra(X)
ans = 0
for i in range(1, N+1):
    if i == X:
        continue
    to_x = dijkstra(i)
    ans = max(ans, from_x[i] + to_x[X])

print(ans)