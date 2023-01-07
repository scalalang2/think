import sys
input = sys.stdin.readline

MAX = int((9 * (10 ** 18)) ** (.1/.3))
che = [True] * MAX
che[0] = False
che[1] = False
primes = []

for i in range(2, MAX):
    if che[i]: 
        primes.append(i)
        j = i*i
        while j < MAX:
            che[j] = False
            j += i * 2

t = int(input())
for i in range(t):
    N = int(input())
    for j in range(len(primes)):
        p = primes[j]
        if N % p != 0:
            continue
        if (N // p) % p == 0:
            q = N // p // p
            print(p, q)
            break
        else:
            q = N // (p*p)
            print(p, q)
            break