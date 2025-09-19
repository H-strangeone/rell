#include <iostream>
#include <bits/stdc++.h>
#include <vector>
using namespace std;

int main(){
    int n=11;
    vector<vector<int>> pascal;
    pascal.push_back({1});
    pascal.push_back({1,1});
    for(int i=2;i<n;i++){
        vector<int> ve(i+1,1);
        for(int k=0;k<i+1;k++){
            if(k==0 || k==i){
                continue;
            }
            else{
                ve[k]=pascal[i-1][k-1]+pascal[i-1][k];
            }
        }
        pascal.push_back(ve);
    }
    for(int i=0;i<n;i++){
        for(int k=0;k<i+1;k++){
            cout<<pascal[i][k]<<" ";
        }
        cout<<endl;
    }
    return 0;
}