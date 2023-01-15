import sys
sys.setrecursionlimit(10**5 + 1)
input = sys.stdin.readline
n = int(input())

hashmap = {}
idx = 0
adj = [[] for _ in range(n*2)]
visited = [False for _ in range(n*2)]
fin = [False for _ in range(n*2)]

def get_idx(text):
    global idx
    if text in hashmap:
        return hashmap[text]
    else:
        hashmap[text] = idx
        idx += 1
        return hashmap[text]

for i in range(n):
    s, t = input().split()
    s = get_idx(s)
    t = get_idx(t)
    adj[s].append(t)

def has_cycle(v):
    visited[v] = True
    fin[v] = True
    for u in adj[v]:
        if fin[u]:
            fin[v] = False
            return True
        if has_cycle(u):
            fin[v] = False
            return True
    fin[v] = False
    return False

for i in range(n*2):
    if not visited[i]:
        if has_cycle(i):
            print("No")
            sys.exit()

print("Yes")