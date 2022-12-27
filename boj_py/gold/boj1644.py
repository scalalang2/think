n = int(input())
MAX = 4000001
che = [True] * MAX
che[0] = False
che[1] = False
psum = [0]

for i in range(2, MAX):
    if not che[i]:
        continue
        
    psum.append(psum[-1] + i)
    j = 2
    while i*j < MAX:
        che[i*j] = False
        j += 1

l, r = 0,1
ans = 0
#n = int(input())

while r < len(psum):
    k = psum[r] - psum[l]
    if k > n:
        l += 1
    elif k < n:
        r += 1
    else:
        ans += 1
        r += 1

print(ans)