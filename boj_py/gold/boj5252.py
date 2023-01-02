# 2
# 3
# 911
# 97625999
# 91125426
# 5
# 113
# 12340
# 123440
# 12345
# 98346

t = int(input())
for _ in range(t):
    n = int(input())
    arr = []
    for _ in range(n):
        arr.append(input().rstrip())

    arr.sort()
    ans = "YES"
    for i in range(1, n):
        if arr[i-1] == arr[i][:len(arr[i-1])]:
            ans = "NO"

    print(ans)

