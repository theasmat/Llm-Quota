export type Theme = 'light' | 'dark' | 'system';

export interface AppConfig {
    theme: Theme;
    language: string;
    auto_check_updates: boolean;
}
