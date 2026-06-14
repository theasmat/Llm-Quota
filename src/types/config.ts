export type Theme = 'light' | 'dark' | 'system';

export interface AppConfig {
    theme: Theme;
    language: string;
    auto_check_updates: boolean;
    oauth_client_id?: string;
    oauth_client_secret?: string;
    tray_mode?: boolean;
}
