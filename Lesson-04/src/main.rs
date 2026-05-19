use std::collections::HashMap;

enum AccountStatus {
    Active,
    Inactive,
    Suspended,
}

fn main() {
    let mut user_balances: HashMap<u32, f64> = HashMap::new();
    user_balances.insert(1, 1000.0);
    let mut user_accounts: HashMap<u32, AccountStatus> = HashMap::new();
    user_accounts.insert(1, AccountStatus::Active);
    match getUserBalance(1, &user_balances) {
        Ok(balance) => println!("User balance: {}", balance),
        Err(e) => println!("Error: {}", e),
    }
    match get_user_account_status(1, &user_accounts) {
        Ok(status) => println!("User account status: {:?}", status),
        Err(e) => println!("Error: {}", e),
    }
}


fn getUserBalance(user_id: u32, balance: &HashMap<u32, f64>) -> Result<f64, String> {
    match balance.get(&user_id) {
        Some(&value) => Ok(value),
        None => Err(String::from("User not found")),
    }
}

fn get_user_account_status(
    user_id: u32,
    accounts: &HashMap<u32, AccountStatus>,
) -> Result<String, String> {
    match accounts.get(&user_id) {
        Some(account_data) => match account_data {
            AccountStatus::Active => Ok(String::from("Active")),
            AccountStatus::Inactive => Ok(String::from("Inactive")),
            AccountStatus::Suspended => Ok(String::from("Suspended")),
        },
        None => Err(String::from("User not found")),
    }
}
