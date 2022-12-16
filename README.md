# Think deeply on problems

### 그래프 문제
| 문제 | 링크 | 알고리즘 |
|---|---|---|
| [백준 3780](https://www.acmicpc.net/problem/3780) | [풀이](./boj_rust/boj3780.rs) | 유니온-파인드와 경로 압축시 센터와의 거리 계산 로직 추가 |

### 동적 계획법
| 문제 | 링크 | 알고리즘 |
|---|---|---|
| [백준 9465](https://www.acmicpc.net/problem/9465) | [풀이](./boj_rust/boj9465.rs) | f(c,s) = max(f(c+1,0), f(c+1,1) + v(c,0) + f(c+1,2) + v(c,1) <br/> 오른쪽에서 부터 답을 채워서 0번째 열에서 결론을 내는 방식으로 풀이 |
| [백준 2294](https://www.acmicpc.net/problem/2294) | [풀이](./boj_rust/boj2294.rs) | f(k) = min(f(k-coin) + 1 `for all coins`) |
| [백준 2533](https://www.acmicpc.net/problem/2533) | [풀이](./boj_rust/boj2533.rs) | tree dp 어려워 |
| [백준 9252](https://www.acmicpc.net/problem/9252) | [풀이](./boj_rust/boj9252.rs) | s[i] == s[j] ? dp[i][j] = dp[i-1][j-1] + 1 : dp[i][j] = max(dp[i-1][j], dp[i][j-1]) |
| [백준 14002](https://www.acmicpc.net/problem/14002) | [풀이](./boj_rust/gold/boj14002.rs) | LIS 알고리즘 dp[i] = max(dp[i], dp[j]+1) |
| [백준 2302](https://www.acmicpc.net/problem/2302) | [풀이](./boj_rust/silver/boj2302.rs) | [노트](./notes/boj2302.md) |

### 트리 문제
| 문제 | 링크 | 알고리즘 |
|---|---|---|
| [백준 22968 : AVL 트리 균형](https://www.acmicpc.net/problem/22968) | [풀이](./boj_rust/boj22968.rs) | 성질 파악하기 d[i] = d[i-2] + d[i-1] + 1 |
| [백준 5639 : 이진 검색 트리](https://www.acmicpc.net/problem/5639) | [풀이](./boj_rust/boj5639.rs) | 이진 탐색 트리 구현하고 post order로 방문하기 |
| [백준 12015](https://www.acmicpc.net/problem/12015) | [풀이](./boj_rust/gold/boj12015.rs) | SegmentTree로 LIS 구하기 |
| [백준 2357](https://www.acmicpc.net/problem/2357) | [풀이](./boj_rust/gold/boj2357.rs) | segtree 2개로 min, max 구하기 |

### 애드-훅
| 문제 | 링크 | 알고리즘 |
|---|---|---|
| [백준 25595](https://www.acmicpc.net/problem/25595) | [풀이](./boj_rust/boj25595.rs) | 신우젠의 위치를 (x,y)라고 하고 레기온의 위치를 (a,b) 라고 할 때, `abs(x-a) + abs(y-b)` 가 홀수면 잡을 수 있고 짝수면 불가능 |

### 기초 문제
| 문제 | 링크 | 알고리즘 |
|---|---|---|
| [백준 10757](https://www.acmicpc.net/problem/10757) | [풀이](./boj_rust/boj10757.rs) | 10^10000 큰 수 더하는 문제, 곱하기나 나누기는 어떠려나? |
| [백준 1302](https://www.acmicpc.net/problem/1302) | [풀이](./boj_rust/boj1302.rs) | `HashMap`을 한 번 사용해보자.  |