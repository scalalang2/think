import sys
input = sys.stdin.readline

n = int(input())
values = list(map(int, input().split()))

ans_min = 10e9
ans_max = -10e9

# 차례대로 +, -, x, /
ops = list(map(int, input().split()))

# 사용한 연산자의 목록
used = [0] * 4

def dfs(curr, acc):
    global ans_min
    global ans_max

    if curr == n:
        ans_min = min(ans_min, acc)
        ans_max = max(ans_max, acc)
        return

    for i in range(4):
        if ops[i] > used[i]:
            used[i] += 1
            if i == 0:
                dfs(curr + 1, acc + values[curr])
            elif i == 1:
                dfs(curr + 1, acc - values[curr])
            elif i == 2:
                dfs(curr + 1, acc * values[curr])
            else:
                if acc < 0:
                    dfs(curr + 1, ((acc*-1) // values[curr]) * -1)
                else:
                    dfs(curr + 1, acc // values[curr])
            used[i] -= 1

dfs(1, values[0])

print(ans_max)
print(ans_min)