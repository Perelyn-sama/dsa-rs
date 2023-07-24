use std::cmp;

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter()
        .fold(0, |max_balance, account| cmp::max(max_balance, account.iter().sum()))
}