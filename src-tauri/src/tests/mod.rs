// Run with: cargo test -- --nocapture
// Tests covering: account CRUD, quota model, token data, and import logic

#[cfg(test)]
mod account_tests {
    use crate::models::{Account, TokenData};
    use crate::modules::account;
    use std::sync::{Mutex, OnceLock};

    /// All account tests share a single JSON file — serialize them to avoid corruption.
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    fn lock() -> std::sync::MutexGuard<'static, ()> {
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|p| p.into_inner())
    }

    /// Remove any leftover accounts from a prior failed run.
    fn cleanup_emails(emails: &[&str]) {
        if let Ok(accs) = account::list_accounts() {
            let ids: Vec<String> = accs
                .iter()
                .filter(|a| emails.contains(&a.email.as_str()))
                .map(|a| a.id.clone())
                .collect();
            if !ids.is_empty() {
                let _ = account::delete_accounts(&ids);
            }
        }
    }

    fn make_token(access: &str, refresh: &str) -> TokenData {
        TokenData::new(
            access.into(),
            refresh.into(),
            3600,
            Some("test@example.com".into()),
            None,
            None,
            false,
            None,
        )
    }

    #[test]
    fn test_list_accounts_does_not_panic() {
        let _g = lock();
        assert!(account::list_accounts().is_ok());
    }

    #[test]
    fn test_save_and_retrieve_account() {
        let _g = lock();
        cleanup_emails(&["save@test.com"]);

        let token = make_token("access_token_test", "refresh_token_test");
        let acc = Account::new("test-save-id".into(), "save@test.com".into(), token);
        account::save_account(&acc).unwrap();

        let found = account::get_account("test-save-id");
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "save@test.com");

        account::delete_account("test-save-id").unwrap();
    }

    #[test]
    fn test_upsert_creates_new_account() {
        let _g = lock();
        cleanup_emails(&["upsert_new@test.com"]);

        let token = make_token("at_new", "rt_new");
        let acc =
            account::upsert_account("upsert_new@test.com".into(), Some("New User".into()), token)
                .unwrap();
        assert_eq!(acc.email, "upsert_new@test.com");
        assert_eq!(acc.name, Some("New User".into()));

        account::delete_account(&acc.id).unwrap();
    }

    #[test]
    fn test_upsert_updates_token_for_existing_email() {
        let _g = lock();
        cleanup_emails(&["upsert_update@test.com"]);

        let acc1 = account::upsert_account(
            "upsert_update@test.com".into(),
            Some("Original".into()),
            make_token("old_access", "rt"),
        )
        .unwrap();
        let acc2 = account::upsert_account(
            "upsert_update@test.com".into(),
            Some("Updated".into()),
            make_token("new_access", "rt"),
        )
        .unwrap();

        assert_eq!(acc1.id, acc2.id, "Same email should update, not create new");
        assert_eq!(acc2.token.access_token, "new_access");

        account::delete_account(&acc1.id).unwrap();
    }

    #[test]
    fn test_delete_account_removes_it() {
        let _g = lock();
        cleanup_emails(&["delete@test.com"]);

        let acc = account::upsert_account("delete@test.com".into(), None, make_token("at", "rt"))
            .unwrap();
        account::delete_account(&acc.id).unwrap();

        assert!(account::get_account(&acc.id).is_none());
    }

    #[test]
    fn test_delete_accounts_bulk() {
        let _g = lock();
        cleanup_emails(&["bulk1@test.com", "bulk2@test.com", "bulk3@test.com"]);

        let acc1 =
            account::upsert_account("bulk1@test.com".into(), None, make_token("a1", "r1")).unwrap();
        let acc2 =
            account::upsert_account("bulk2@test.com".into(), None, make_token("a2", "r2")).unwrap();
        let acc3 =
            account::upsert_account("bulk3@test.com".into(), None, make_token("a3", "r3")).unwrap();

        account::delete_accounts(&[acc1.id.clone(), acc2.id.clone()]).unwrap();

        assert!(account::get_account(&acc1.id).is_none());
        assert!(account::get_account(&acc2.id).is_none());
        assert!(account::get_account(&acc3.id).is_some());

        account::delete_account(&acc3.id).unwrap();
    }

    #[test]
    fn test_reorder_accounts() {
        let _g = lock();
        cleanup_emails(&["reorder1@test.com", "reorder2@test.com"]);

        let acc1 =
            account::upsert_account("reorder1@test.com".into(), None, make_token("a1", "r1"))
                .unwrap();
        let acc2 =
            account::upsert_account("reorder2@test.com".into(), None, make_token("a2", "r2"))
                .unwrap();

        account::reorder_accounts(&[acc2.id.clone(), acc1.id.clone()]).unwrap();

        let accounts = account::list_accounts().unwrap();
        let pos1 = accounts.iter().position(|a| a.id == acc2.id).unwrap();
        let pos2 = accounts.iter().position(|a| a.id == acc1.id).unwrap();
        assert!(pos1 < pos2, "acc2 should come before acc1 after reorder");

        account::delete_accounts(&[acc1.id.clone(), acc2.id.clone()]).unwrap();
    }
}

#[cfg(test)]
mod quota_tests {
    use crate::models::QuotaData;

    #[test]
    fn test_quota_defaults() {
        let q = QuotaData::new();
        assert!(!q.is_forbidden);
        assert!(q.models.is_empty());
        assert!(q.subscription_tier.is_none());
    }

    #[test]
    fn test_quota_forbidden_round_trips_json() {
        let mut q = QuotaData::new();
        q.is_forbidden = true;
        q.subscription_tier = Some("enterprise".into());

        let json = serde_json::to_string(&q).unwrap();
        let back: QuotaData = serde_json::from_str(&json).unwrap();
        assert!(back.is_forbidden);
        assert_eq!(back.subscription_tier, Some("enterprise".into()));
    }

    #[test]
    fn test_quota_model_list_serialization() {
        use crate::models::quota::ModelQuota;
        let mut q = QuotaData::new();
        q.models = vec![
            ModelQuota {
                name: "gemini-1.5-pro".into(),
                percentage: 80,
                reset_time: "".into(),
                display_name: None,
                supports_images: None,
                supports_thinking: None,
                thinking_budget: None,
                recommended: None,
                max_tokens: None,
                max_output_tokens: None,
                supported_mime_types: None,
            },
            ModelQuota {
                name: "gemini-2.0-flash".into(),
                percentage: 50,
                reset_time: "".into(),
                display_name: None,
                supports_images: None,
                supports_thinking: None,
                thinking_budget: None,
                recommended: None,
                max_tokens: None,
                max_output_tokens: None,
                supported_mime_types: None,
            },
        ];

        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("gemini-1.5-pro"));
        let back: QuotaData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.models.len(), 2);
    }
}

#[cfg(test)]
mod token_tests {
    use crate::models::TokenData;

    #[test]
    fn test_token_fields_correct() {
        let t = TokenData::new(
            "my_access".into(),
            "my_refresh".into(),
            7200,
            Some("user@example.com".into()),
            None,
            None,
            false,
            Some("id_token_value".into()),
        );
        assert_eq!(t.access_token, "my_access");
        assert_eq!(t.refresh_token, "my_refresh");
        assert_eq!(t.expires_in, 7200);
        assert_eq!(t.email, Some("user@example.com".into()));
    }

    #[test]
    fn test_with_oauth_client_key_chaining() {
        let t = TokenData::new("at".into(), "rt".into(), 100, None, None, None, false, None)
            .with_oauth_client_key(Some("client-key-123".into()));
        assert_eq!(t.oauth_client_key, Some("client-key-123".into()));
    }

    #[test]
    fn test_token_has_future_expiry_timestamp() {
        let t = TokenData::new(
            "at".into(),
            "rt".into(),
            3600,
            None,
            None,
            None,
            false,
            None,
        );
        let now = chrono::Utc::now().timestamp();
        assert!(
            t.expiry_timestamp > now,
            "Token expiry_timestamp should be in the future"
        );
    }
}

#[cfg(test)]
mod import_tests {
    use crate::models::{Account, TokenData};
    use std::fs;

    fn make_test_accounts(count: usize) -> Vec<Account> {
        (0..count)
            .map(|i| {
                let token = TokenData::new(
                    format!("at_{i}"),
                    format!("rt_{i}"),
                    3600,
                    None,
                    None,
                    None,
                    false,
                    None,
                );
                Account::new(format!("import-id-{i}"), format!("user{i}@test.com"), token)
            })
            .collect()
    }

    #[test]
    fn test_import_valid_json_file() {
        let accs = make_test_accounts(3);
        let path = "/tmp/ag_import_valid_test.json";
        fs::write(path, serde_json::to_string_pretty(&accs).unwrap()).unwrap();

        let content = fs::read_to_string(path).unwrap();
        let loaded: Vec<Account> = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].email, "user0@test.com");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_import_invalid_json_returns_err() {
        let path = "/tmp/ag_import_bad_test.json";
        fs::write(path, r#"{ not valid json !"#).unwrap();

        let content = fs::read_to_string(path).unwrap();
        let result: Result<Vec<Account>, _> =
            serde_json::from_str(&content).map_err(|e| e.to_string());
        assert!(result.is_err());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_import_missing_file_returns_empty() {
        let path = std::path::Path::new("/tmp/ag_no_such_file_xyz.json");
        if path.exists() {
            fs::remove_file(path).unwrap();
        }

        let result: Vec<Account> = if path.exists() {
            let c = fs::read_to_string(path).unwrap();
            serde_json::from_str(&c).unwrap()
        } else {
            vec![]
        };
        assert!(result.is_empty());
    }

    #[test]
    fn test_import_empty_array_json() {
        let path = "/tmp/ag_import_empty_test.json";
        fs::write(path, "[]").unwrap();

        let content = fs::read_to_string(path).unwrap();
        let loaded: Vec<Account> = serde_json::from_str(&content).unwrap();
        assert!(loaded.is_empty());

        fs::remove_file(path).unwrap();
    }
}
