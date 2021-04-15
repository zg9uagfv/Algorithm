//! dp: dynamic plan
//!
//! 解题思路:
//! 求解动态规划的核心问题是穷举。因为要求最值，肯定要把所有可行的答案穷举出来，然后在其中找最值
//!
//! 1. 动态规划的穷举有点特别，因为这类问题存在「重叠子问题」，如果暴力穷举的话效率会极其低下，
//! 所以需要「备忘录」或者「DP table」来优化穷举过程，避免不必要的计算
//!
//! 2. 动态规划问题一定会具备「最优子结构」，才能通过子问题的最值得到原问题的最值
//!
//! 3. 虽然动态规划的核心思想就是穷举求最值，但是问题可以千变万化，穷举所有可行解其实并不是一件
//! 容易的事，只有列出正确的「状态转移方程」才能正确地穷举
//!
//! 以上提到的重叠子问题、最优子结构、状态转移方程就是动态规划三要素
//!
//! 但凡遇到需要递归的问题，最好都画出递归树，这对你分析算法的复杂度，寻找算法低效的原因都有巨大帮助

mod coin;
mod fib;
