import sys
input = sys.stdin.readline

t = int(input())
for _ in range(t):
    arr = list(map(int, input().split()))
    x1,y1 = arr[0], arr[1]
    x2,y2 = arr[2], arr[3]
    x3,y3 = arr[4], arr[5]
    x4,y4 = arr[6], arr[7]

    area1 = abs(x2-x1) * abs(y2-y1)
    x3,y3 = min(max(x1,x3), x2), min(max(y1,y3), y2)
    x4,y4 = min(max(x1,x4), x2), min(max(y1,y4), y2)

    area2 = abs(x4-x3) * abs(y4-y3)

    print(area1-area2)