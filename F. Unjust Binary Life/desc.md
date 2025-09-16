F. Unjust Binary Life([https://codeforces.com/problemset/problem/2131/F])
time limit per test2 seconds
memory limit per test256 megabytes

Yuri is given two binary strings 𝑎
 and 𝑏
, both of which are of length 𝑛
. The two strings dynamically define an 𝑛×𝑛
 grid. Let (𝑖,𝑗)
 denote the cell in the 𝑖
-th row and 𝑗
-th column. The initial value of cell (𝑖,𝑗)
 has the value of 𝑎𝑖⊕𝑏𝑗
, where ⊕
 denotes the bitwise XOR operation. .

Yuri's journey always starts at cell (1,1)
. From a cell (𝑖,𝑗)
, she can only move down to (𝑖+1,𝑗)
 or right to (𝑖,𝑗+1)
. Her journey is possible if there exists a valid path such that all cells on the path, including (1,1)
, have a value of 0.

Before her departure, she can do the following operation for any number of times:

Choose one index 1≤𝑖≤𝑛
, and flip the value of either 𝑎𝑖
 or 𝑏𝑖
 (0
 becomes 1
, and 1
 becomes 0
). The grid will also change accordingly.
Let 𝑓(𝑥,𝑦)
 denote the minimum required operations so that Yuri can make her journey to the cell (𝑥,𝑦)
. You must determine the sum of 𝑓(𝑥,𝑦)
 over all 1≤𝑥,𝑦≤𝑛
.

Note that each of these 𝑛2
 cases is independent, meaning you need to assume the grid is in its original state in each case (i.e., no actual operations are performed).

Input
Each test contains multiple test cases. The first line contains the number of test cases 𝑡
 (1≤𝑡≤104
). The description of the test cases follows.

The first line of each test case contains one integer 𝑛
 (1≤𝑛≤2⋅105
).

The second line of each test case contains a binary string 𝑎
 (|𝑎|=𝑛
, 𝑎𝑖∈{0,1}
).

The third line of each test case contains a binary string 𝑏
 (|𝑏|=𝑛
, 𝑏𝑖∈{0,1}
).

It is guaranteed that the sum of 𝑛
 over all test cases does not exceed 2⋅105
.

Output
For each test case, output one integer — the sum of minimum operations over all possible cells.

Example
InputCopy
3
2
11
00
2
01
01
4
1010
1101
OutputCopy
5
4
24
Note
In the first test case, the 2×2
 grid is shown below.

1111

In the initial state, Yuri cannot reach any cell.

Yuri can flip 𝑎1
 so that the grid becomes:

0011

and Yuri can travel to cells (1,1)
 and (1,2)
.

On the other hand, Yuri can flip 𝑏1
 so that the grid becomes:

0101

and Yuri can travel to cells (1,1)
 and (2,1)
.

To move to the cell (2,2)
, it can be shown that she must perform at least two operations. For example, she can flip both 𝑎1
 and 𝑎2
 so that the grid becomes:

0000

Therefore, the answer is 1+1+1+2=5
.


