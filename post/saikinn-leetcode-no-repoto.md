---
title: 平成最后的 leetcode 解题报告
categories: Coding
tags:
date: 2019-04-17 20:49:30
---
# 平成最后的 leetcode 解题报告

最近一段时间积攒的 leetcode 解题报告（虽然都是水题啦）。

## 146 lru cache

模拟一个 LRU 缓存。一般的做法应该是用 hashmap 存索引，用 queue 存实际内容（和去除过期内容）。但是由于 rust 这里的指针问题我不想存索引（咦）。于是用了个取巧的做法，用版本来避免对 queue 的更新。过这道题是足够了。插入新元素和取出元素的效率都是均摊 O(1)。

[代码](https://github.com/laxect/leetcode/blob/master/src/h146_lru_cache.rs)

## 204 计算质数

欧拉筛就完事了。

```C++
bool flag[MAXN] = {1};    //将标识初始化为true
void erat(int maxn)
{
	flag[0]=0;            //0不是質数
	flag[1]=0;            //1不是質数
	for(int i=2;i<=maxn;++i)
	{
         /*当i为質数时，i的所有倍数都不是質数*/
		if(flag[i])
		{
			for(int j=i*i;j<=maxn;j+=i)
			{
				flag[j]=0;
			}
		}
	}
}
```

这个是 wiki 的版本。（比我写的干净）

## 268 missing number

还算挺有趣的题吧（还是水）。简单来说思路就是把 i 放到数组的第 i 个，那么全放完后 n 的位置就是丢失的数（本该在的位置）。特殊情况就是消失的是 n。

[代码](https://github.com/laxect/leetcode/blob/master/src/e268_missing_number.rs)

matrix67 上一个类似的[实例](http://www.matrix67.com/blog/archives/6584)

## 326 power of three

真正的水题...只需要一次 log...可能会有精度问题，提前判一下是不是 3 的倍数（注意*1*）就好了。

## 704 binary search

翻...翻翻书？

## 155 min stack

最小栈。用两个栈就好了，一个维护元素，一个维护最小值。最小队列同理可解。

[代码](https://github.com/laxect/leetcode/blob/master/src/e155_min_stack.rs)

## longest valid parenthess

基本就是基于栈的括号匹配...稍微扩展下就好了。在栈里一起存下括号序列长度就可以了。

[代码](https://github.com/laxect/leetcode/blob/master/src/h032_longest_valid_parentheses.rs)
