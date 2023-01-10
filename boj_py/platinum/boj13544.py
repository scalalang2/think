import sys
from bisect import bisect_right
input = sys.stdin.readline

MAX = 1 << 18

class MergeSortTree:
    def __init__(self, max_size:int):
        self.max_size = max_size
        self.tree = [[] for _ in range(max_size)]

    def update(self, node, low, high, pos, value):
        # 구간이 pos - [low, high] 이거나 [low,high] - pos 인 경우
        # 구간이 벗어나면 그냥 리턴함
        if pos < low or pos > high:
            return
        
        self.tree[node].append(value)
        if low < high:
            m = (low + high)//2
            self.update(node*2, low, m, pos, value)
            self.update(node*2+1, m+1, high, pos, value)
        
    def query(self, node, low, high, left, right, k):
        if right < low or left > high:
            return 0
        
        # 구간이 완벽히 포개진다면
        if left <= low and high <= right:
            x = bisect_right(self.tree[node], k)
            return len(self.tree[node]) - x

        mid = (low + high) // 2
        return self.query(node*2, low, mid, left, right, k) + self.query(node*2+1, mid+1, high, left, right, k)

mst = MergeSortTree(MAX)
n = int(input())
values = list(map(int, input().split()))

for i in range(n):
    mst.update(1, 1, n, i+1, values[i])

for i in range(MAX):
    mst.tree[i].sort()

m = int(input())
ans = 0
for _ in range(m):
    i, j, k = map(int, input().split())
    a = i ^ ans
    b = j ^ ans
    k = k ^ ans

    ans = mst.query(1, 1, n, a, b, k)
    print(ans)