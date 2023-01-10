import sys
from bisect import bisect_right
input = sys.stdin.readline

MAX = 1 << 18

# 쿼리를 사사삭 날리는데
# 쿼리 자체는 임의의 수 r 에 대해서, r 보다 작은게 몇개 있는지 알려주는 걸로 만든다.
# 이걸 파라메트릭 서치를 통해서 개수가 k가 되는 r을 찾으면 된다 ㅋㅋ

class MergeSortTree:
    def __init__(self, max_size:int):
        self.max_size = max_size
        self.tree = [[] for _ in range(max_size)]

    def update(self, node, low, high, pos, value):
        if pos < low or pos > high:
            return
        
        self.tree[node].append(value)
        if low < high:
            m = (low + high)//2
            self.update(node*2, low, m, pos, value)
            self.update(node*2+1, m+1, high, pos, value)
    
    def sort(self):
        for i in range(self.max_size):
            self.tree[i].sort()

    def query(self, node, low, high, left, right, k):
        if right < low or left > high:
            return 0
        
        # 구간이 완벽히 포개진다면
        if left <= low and high <= right:
            return bisect_right(self.tree[node], k)

        mid = (low + high) // 2
        return self.query(node*2, low, mid, left, right, k) + self.query(node*2+1, mid+1, high, left, right, k)

N, Q = map(int, input().split())
arr = list(map(int, input().split()))
mst = MergeSortTree(MAX)
for i in range(N):
    mst.update(1, 1, N, i+1, arr[i])

mst.sort()

import time

for _ in range(Q):
    i, j, k = map(int, input().split())
    s, e = -1*(10**9), 10**9
    ans = 0 
    while s < e:
        mid = (s+e)//2
        qry = mst.query(1, 1, N, i, j, mid)
        # print(s,e,mid, qry)
        # time.sleep(0.1)
        # 개수가 k 보다 많다면?, r값을 줄여야 하는거지
        # 임의의 값 보다 작은게 많다면 그 임의의 값을 줄여야 k를 찾지 바보야
        if qry < k:
            s = mid + 1
        elif qry > k:
            e = mid
        else:
            ans = mid
            e = mid
    print(ans)
