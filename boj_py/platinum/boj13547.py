import sys

input = sys.stdin.readline

arr = [0] * 100001
bucket = [0] * 320 # 100001 ** 0.5 ~= 316
cnt = [0 for _ in range(1000001)]
ans = 0

# inputs
N = int(input())
nums = list(map(int, input().split()))
for i in range(N):
    arr[i+1] = nums[i]
M = int(input())
sqrtN = int(N ** .5)

def add(x): 
    global ans
    if cnt[x] == 0:
        ans += 1
    cnt[x] += 1

def remove(x):
    global ans
    cnt[x] -= 1
    if cnt[x] == 0:
        ans -= 1

queries = []
answers = [0 for _ in range(M)]
for i in range(M):
    L, R = map(int, input().split())
    queries.append((L, R, i))

queries.sort(key=lambda x: (x[0] // sqrtN, x[1]))
currL, currR, x = queries[0][0], queries[0][1], queries[0][2]

for i in range(currL, currR+1):
    add(arr[i])

answers[x] = ans

for i in range(1, len(queries)):
    L, R, x = queries[i]

    while currL < L:
        remove(arr[currL])
        currL += 1
    while currL > L:
        currL -= 1
        add(arr[currL])
    
    while currR < R:
        currR += 1
        add(arr[currR])
    while currR > R:
        remove(arr[currR])
        currR -= 1
    
    answers[x] = ans

for i in range(M):
    print(answers[i])