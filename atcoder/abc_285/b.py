import sys 
input = sys.stdin.readline

N = int(input())
txt = input().rstrip()
for i in range(1, N):
    ans = 0
    for k in range(0, N - i):
        if txt[k] != txt[k + i]:
            ans += 1
        else:
            break
    print(ans)