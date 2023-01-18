import sys
input = sys.stdin.readline
S = input().rstrip()
SS = sorted(S)
SS.reverse()
print("".join(SS))