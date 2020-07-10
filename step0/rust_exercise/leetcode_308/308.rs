use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n=prices.len();
        let mut dp = Vec::new();
        for _ in 0..n{
            let mut temp=vec![0;3];
            dp.push(temp);
        }
        dp[0][0]=-prices[0];
        for i in 1..n{
            dp[i][0]=max(dp[i-1][0],dp[i][1]+prices[i]);
            dp[i][1]=dp[i-1][2]+prices[i];
            dp[i][2]=max(dp[i-1][0],dp[i-1][2]);
        }
        max(dp[n-1][0],dp[n-1][2])
    }
}