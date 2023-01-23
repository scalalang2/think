import sys
input = sys.stdin.readline

N = int(input())
M = 998244353
A = input().rstrip()
B = input().rstrip()

AA = []
BB = []

for i in range(N):
    AA.append(int(A[i]))
    BB.append(int(B[i]))

for i in range(N):
    if AA[i] == BB[i]:
        continue
    if AA[i] < BB[i]:
        AA[i], BB[i] = BB[i], AA[i]

print((int("".join(map(str, AA))) * int("".join(map(str, BB)))) % M)
