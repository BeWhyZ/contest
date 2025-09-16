[E. Adjacent XOR](https://codeforces.com/contest/2131/problem/E)
time limit per test2 seconds
memory limit per test256 megabytes

You're given an array 𝑎
 of length 𝑛
. For each index 𝑖
 such that 1≤𝑖<𝑛
, you can perform the following operation at most once:

Assign 𝑎𝑖:=𝑎𝑖⊕𝑎𝑖+1
, where ⊕
 denotes the bitwise XOR operation. .
You can choose indices and perform the operations in any sequential order.

Given another array 𝑏
 of length 𝑛
, determine if it is possible to transform 𝑎
 to 𝑏
.

Input
Each test contains multiple test cases. The first line contains the number of test cases 𝑡
 (1≤𝑡≤104
). The description of the test cases follows.

The first line of each test case contains one integer 𝑛
 (2≤𝑛≤2⋅105
).

The second line of each test case contains 𝑛
 integers 𝑎1,𝑎2,…,𝑎𝑛
 (0≤𝑎𝑖<230
).

The third line of each test case contains 𝑛
 integers 𝑏1,𝑏2,…,𝑏𝑛
 (0≤𝑏𝑖<230
).

It is guaranteed that the sum of 𝑛
 over all test cases does not exceed 2⋅105
.

Output
For each test case, output "YES" (quotes excluded) if 𝑎
 can be transformed to 𝑏
; otherwise, output "NO". You can output the answer in any case (upper or lower). For example, the strings "yEs", "yes", "Yes", and "YES" will be recognized as positive responses.

input
```shell
7
5
1 2 3 4 5
3 2 7 1 5
3
0 0 1
1 0 1
3
0 0 1
0 0 0
4
0 0 1 2
1 3 3 2
6
1 1 4 5 1 4
0 5 4 5 5 4
3
0 1 2
2 3 2
2
10 10
11 10
```
output
```shell
YES
NO
NO
NO
YES
NO
NO
```


Note
In the first test case, you can perform the operations in the following order:

Choose index 𝑖=3
 and assign 𝑎3:=𝑎3⊕𝑎4=7
, and 𝑎
 becomes [1,2,7,4,5]
.
Choose index 𝑖=4
 and assign 𝑎4:=𝑎4⊕𝑎5=1
, and 𝑎
 becomes [1,2,7,1,5]
.
Choose index 𝑖=1
 and assign 𝑎1:=𝑎1⊕𝑎2=3
, and 𝑎
 becomes [3,2,7,1,5]
.