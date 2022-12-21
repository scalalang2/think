import sys
input = sys.stdin.readline
N, M = map(int, input().split())
sqrtN = int(N ** .5)
arr = list(map(int, input().split()))
h = {}
cnt = [0] * 500001
ans = 0

for i in range(N):
    # 큰 값을 좌표 i 로 압축함
    if arr[i] not in h:
        h[arr[i]] = i

def insert(x):
    global ans
    if cnt[h[x]] == 2:
        ans -= 1
        cnt[h[x]] += 1
    else:
        cnt[h[x]] += 1
        if cnt[h[x]] == 2:
            ans += 1

def remove(x):
    global ans
    if cnt[h[x]] == 2:
        ans -= 1
        cnt[h[x]] -= 1
    else:
        cnt[h[x]] -= 1
        if cnt[h[x]] == 2:
            ans += 1

Q = []
for i in range(M):
    L, R = map(int, input().split())
    Q.append((L-1, R-1, i))

Q.sort(key=lambda x: (x[0] // sqrtN, x[1]))
currL, currR, x = Q[0]
answers = [0] * M
for i in range(currL, currR+1):
    insert(arr[i])

answers[x] = ans

for i in range(1, len(Q)):
    L, R, x = Q[i]

    while currL < L:
        remove(arr[currL])
        currL += 1
    while currL > L:
        currL -= 1
        insert(arr[currL])
    
    while currR < R:
        currR += 1
        insert(arr[currR])
    while currR > R:
        remove(arr[currR])
        currR -= 1
    
    answers[x] = ans

for i in range(len(answers)):
    print(answers[i])