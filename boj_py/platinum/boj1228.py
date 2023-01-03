import sys
import math
input = sys.stdin.readline

tmp = {}
tmp_idx = 1

N, M = map(int, input().split())
adj = [[] for _ in range(250)]
cost = [math.inf for _ in range(250)]
indegree = [0 for _ in range(250)]
tsize = [0 for _ in range(250)] # root 로 부터 시작한 자식 노드의 개수를 저장하는 테이블
dp = [[math.inf for _ in range(250)] for _ in range(250)]

def name_to_idx(name:str):
    global tmp_idx
    global tmp

    if name in tmp:
        return tmp[name]
    else:
        tmp[name] = tmp_idx
        tmp_idx += 1
        return tmp[name]

for _ in range(N):
    its = input().split()
    s = name_to_idx(its[0])
    cost[s] = int(its[1])

    for v in its[2:]:
        e = name_to_idx(v)
        adj[s].append(e)
        indegree[e] += 1

for i in range(1, N+1):
    if indegree[i] == 0:
        adj[0].append(i)

def dfs(node):
    cnt = 1
    for nxt in adj[node]:
        cnt += dfs(nxt)
    tsize[node] = cnt
    return cnt

dfs(0)

# dp[node][c] => node에서 시작해서 c개 이상 정복한 애들의 최소 비용
# dp[node][T] = min(dp[node][T], dp[자식][i] + dp[부모][t-i]) 의 비용과 같다, 자식 중 하나를 선택해서 i개 이상 정복하라고 하고 부모는 t-i 개를 정복하면 동일함
# 부모에 딸린 노드 개수에 대해 for loop를 돌린다.
# dp[node][c] = (dp[node][c], cost[curr]) <- 부모만 정복하면 모든 자식을 정복할 수 있기 떄문에 각 자식놈들 개수마다 돌림
def f(curr):
    dp[curr][0] = 0
    for nxt in adj[curr]:
        f(nxt)

        for t in range(M, -1, -1):
            for i in range(0, t+1):
                dp[curr][t] = min(dp[curr][t], dp[nxt][i] + dp[curr][t-i])
    
    for i in range(tsize[curr]+1):
        dp[curr][i] = min(dp[curr][i], cost[curr])

f(0)
print(dp[0][M])