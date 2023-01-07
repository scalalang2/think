import sys
input = sys.stdin.readline

# 5
# -1 1
# -1 0
# 0 0
# 1 0
# 1 1

n = int(input())
arr = []
for i in range(n):
    x, y = map(int, input().split())
    arr.append((x, y))

cnt = 0
for i in range(n-2):
    for j in range(i+1,n-1):
        for k in range(j+1, n):
            ax,ay = arr[i]
            bx,by = arr[j]
            cx,cy = arr[k]

            a = (ax - bx) ** 2 + (ay-by) ** 2
            b = (ax - cx) ** 2 + (ay-cy) ** 2
            c = (bx-cx) ** 2 + (by-cy) ** 2

            if a + b == c or b+c ==a or a+c == b:
                cnt += 1

print(cnt)