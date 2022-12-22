import sys
from heapq import heappush, heappop
import math
input = sys.stdin.readline

N = int(input())
M = int(input())

MAX_N = 1001

adj = [[] for _ in range(MAX_N)]
dist = [math.inf for _ in range(MAX_N)]
visited = [False for _ in range(MAX_N)]

for i in range(M):
    a, b, c = map(int, input().split())
    adj[a].append((b, c))

start, end = map(int, input().split())

queue = []
heappush(queue, (0, start))
dist[start] = 0

while len(queue) > 0:
    cost, curr = heappop(queue)
    if visited[curr]:
        continue

    visited[curr] = True
    for next, d in adj[curr]:
        if dist[next] > dist[curr] + d:
            dist[next] = dist[curr] + d
            heappush(queue, (dist[next], next))

print(dist[end])