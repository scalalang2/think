#include<bits/stdc++.h>

using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    int n;
    cin >> n;
    if(n == 0) {
        cout << 1 << endl;
    } else {
        int ans = n;
        for(int i = n-1; i > 0; i--) {
            ans *= i;
        }
        cout << ans << endl;
    }

    return 0;
}