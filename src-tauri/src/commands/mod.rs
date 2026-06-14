pub mod account;
pub mod config;
pub mod oauth;
pub mod system;
pub mod integration;

use account::*;
use config::*;
use oauth::*;
use system::*;
use integration::*;

/// Generates a handler for all commands in this module
pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        list_accounts,
        add_account,
        delete_account,
        delete_accounts,
        reorder_accounts,
        switch_account,
        export_accounts,
        get_current_account,
        fetch_account_quota,
        refresh_all_quotas,
        load_config,
        save_config,
        get_data_dir_path,
        start_oauth_login,
        read_text_file,
        save_text_file,
        check_proxy_latency,
        show_main_window,
        set_window_theme,
        prepare_oauth_url,
        complete_oauth_login,
        cancel_oauth_login,
        submit_oauth_code,
        list_oauth_clients,
        get_active_oauth_client,
        set_active_oauth_client,
        import_from_db,
        import_v1_accounts,
        import_custom_db,
        check_local_integrations,
    ]
}
