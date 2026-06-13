import type { LucideIcon } from 'lucide-react';

// 
export interface NavItem {
    path: string;
    label: string;
    icon: LucideIcon;
    priority: 'high' | 'medium' | 'low';
}

export interface Language {
    code: string;
    label: string;
    short: string;
}

// 
export const LANGUAGES: Language[] = [
    { code: 'zh', label: '', short: 'ZH' },
    { code: 'zh-TW', label: '', short: 'TW' },
    { code: 'en', label: 'English', short: 'EN' },
    { code: 'ja', label: '', short: 'JA' },
    { code: 'tr', label: 'Türkçe', short: 'TR' },
    { code: 'vi', label: 'Tiếng Việt', short: 'VI' },
    { code: 'pt', label: 'Português', short: 'PT' },
    { code: 'ko', label: '한국어', short: 'KO' },
    { code: 'ru', label: 'Русский', short: 'RU' },
    { code: 'ar', label: 'العربية', short: 'AR' },
    { code: 'es', label: 'Español', short: 'ES' },
    { code: 'my', label: 'Bahasa Melayu', short: 'MY' },
];

// 
export const isActive = (pathname: string, itemPath: string): boolean => {
    if (itemPath === '/') {
        return pathname === '/';
    }
    return pathname.startsWith(itemPath);
};

export const getCurrentNavItem = (pathname: string, navItems: NavItem[]): NavItem => {
    return navItems.find(item => isActive(pathname, item.path)) || navItems[0];
};
