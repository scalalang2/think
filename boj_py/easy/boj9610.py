import sys
input = sys.stdin.readline

n = int(input())
ans = [0,0,0,0,0]

for i in range(n):
    x, y = map(int, input().split())
    if x == 0 or y == 0:
        ans[4] += 1
        continue
    if x >= 1 and y >= 1:
        ans[0] += 1
    if x < 0 and y >= 1:
        ans[1] += 1
    if x > 0 and y < 0:
        ans[3] += 1
    if x < 0 and y < 0:
        ans[2] += 1
    
print("Q1: {}".format(ans[0]))
print("Q2: {}".format(ans[1]))
print("Q3: {}".format(ans[2]))
print("Q4: {}".format(ans[3]))
print("AXIS: {}".format(ans[4]))