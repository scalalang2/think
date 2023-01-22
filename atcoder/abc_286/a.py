n, p, q, r, s = map(int, input().split())
a = list(map(int, input().split()))
p -= 1
r -= 1
a[p:q], a[r:s] = a[r:s], a[p:q]
print(*a)