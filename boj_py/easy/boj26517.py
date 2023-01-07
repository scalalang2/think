import sys
input = sys.stdin.readline

K = int(input())
a,b,c,d = list(map(int, input().split()))

def f(x):
    return a * x + b
def g(x):
    return c*x + d

if f(K) == g(K):
    print("Yes {}".format(f(K)))
else:
    print("No")