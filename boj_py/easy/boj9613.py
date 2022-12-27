# 3
# 4 10 20 30 40
# 3 7 5 12
# 3 125 15 25

import sys
input = sys.stdin.readline

def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a%b)

n = int(input())
for _ in range(n):
    arr = list(map(int, input().split()))
    arr = arr[1:]

    ans = 0

    for i in range(len(arr)-1):
        for j in range(i+1, len(arr)):
            ans += gcd(max(arr[i], arr[j]), min(arr[i], arr[j]))
    
    print(ans)