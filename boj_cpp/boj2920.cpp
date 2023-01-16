#include<bits/stdc++.h>

using namespace std;

const int n = 8;
int a[n];

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    for(int i = 0; i < n; i++) {
        cin >> a[i];
    }

    bool increasing = true;
    bool decreasing = true;
    for(int i = 0; i < n; i++) {
        if(a[i] != i+1) { 
            increasing = false;
        }

        if(a[i] != n-i) {
            decreasing = false;
        }

    }

    if(increasing) {
        cout << "ascending" << endl;
    } else if(decreasing) {
        cout << "descending" << endl;
    } else {
        cout << "mixed" << endl;
    }

    return 0;
}