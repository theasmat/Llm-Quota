use crate::models::{Account, TokenData};
use std::fs;
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".gemini");
    path.push("antigravity");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn get_accounts_file() -> PathBuf {
    let mut path = get_data_dir();
    path.push("accounts.json");
    path
}

pub fn list_accounts() -> Result<Vec<Account>, String> {
    let path = get_accounts_file();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn save_accounts(accounts: &[Account]) -> Result<(), String> {
    let path = get_accounts_file();
    let json = serde_json::to_string_pretty(accounts).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

pub fn get_account(account_id: &str) -> Option<Account> {
    list_accounts().unwrap_or_default().into_iter().find(|a| a.id == account_id)
}

pub fn save_account(account: &Account) -> Result<(), String> {
    let mut accounts = list_accounts()?;
    if let Some(idx) = accounts.iter().position(|a| a.id == account.id) {
        accounts[idx] = account.clone();
    } else {
        accounts.push(account.clone());
    }
    save_accounts(&accounts)
}

pub fn upsert_account(email: String, name: Option<String>, token: TokenData) -> Result<Account, String> {
    let mut accounts = list_accounts()?;
    if let Some(existing) = accounts.iter_mut().find(|a| a.email == email) {
        existing.token = token.clone();
        if name.is_some() {
            existing.name = name;
        }
        let acc = existing.clone();
        save_accounts(&accounts)?;
        return Ok(acc);
    }
    let id = uuid::Uuid::new_v4().to_string();
    let mut new_acc = Account::new(id, email, token);
    new_acc.name = name;
    accounts.push(new_acc.clone());
    save_accounts(&accounts)?;
    Ok(new_acc)
}

pub fn delete_account(account_id: &str) -> Result<(), String> {
    let mut accounts = list_accounts()?;
    accounts.retain(|a| a.id != account_id);
    save_accounts(&accounts)
}

pub fn delete_accounts(account_ids: &[String]) -> Result<(), String> {
    let mut accounts = list_accounts()?;
    accounts.retain(|a| !account_ids.contains(&a.id));
    save_accounts(&accounts)
}

pub fn reorder_accounts(account_ids: &[String]) -> Result<(), String> {
    let mut accounts = list_accounts()?;
    let mut new_list = Vec::new();
    for id in account_ids {
        if let Some(idx) = accounts.iter().position(|a| &a.id == id) {
            new_list.push(accounts.remove(idx));
        }
    }
    new_list.extend(accounts);
    save_accounts(&new_list)
}

pub async fn refresh_all_quotas_logic() -> Result<(), String> {
    // simplified
    Ok(())
}
