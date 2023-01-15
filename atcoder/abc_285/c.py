import sys
input = sys.stdin.readline

txt = input().rstrip()

def char_to_num(ch):
    return ord(ch) - ord('A') + 1

cnt = 0
ans = 0
for i in range(len(txt)-1, -1, -1):
    n = char_to_num(txt[i])
    ans += n * (26 ** cnt)
    cnt += 1

print(ans)