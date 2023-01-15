import sys
input = sys.stdin.readline

N = int(input())
S = input().rstrip()
T = sorted(S)
Q = int(input())

for i in range(Q):
    a, b, c = input().split()

# a 가 1번인 경우 S의 b번 문자열을 c로 대체해야함
# a 가 2번인 경우 구간 L, R에 대해서 S[L:R]이 T=sorted(S)의 부분 문자열인지 판별해야함

# 그러면 S가 바뀌면 T도 바뀔텐데 
# 매번 다시 정렬해? -> 이걸 어떻게 최적화하지?

# 무조건 세그먼트 트리를 이용한 구간 최적화 같은데
# 어떻게 써먹는 건지 모르겠다 ㅋㅋ