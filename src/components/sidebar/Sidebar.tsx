import { LayoutDashboard, Settings, Moon, Sun, LogOut, PanelLeftClose, PanelLeftOpen, Sparkles, PanelTop, AppWindow } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { useConfigStore } from '../../stores/useConfigStore';
import { isTauri, isLinux } from '../../utils/env';
import { SidebarItem } from './SidebarItem';
import { NavLogo } from '../navigation/NavLogo';
import { useViewStore } from '../../stores/useViewStore';

export function Sidebar() {
    const { t } = useTranslation();
    const { config, saveConfig } = useConfigStore();
    const { isSidebarCollapsed, toggleSidebar } = useViewStore();

    const navItems = [
        { path: '/', label: t('nav.dashboard', 'Dashboard'), icon: LayoutDashboard },
        { path: '/accounts', label: t('nav.accounts', 'Gemini'), icon: Sparkles },
        { path: '/settings', label: t('nav.settings', 'Settings'), icon: Settings },
    ];

    const toggleTheme = async (event: React.MouseEvent<HTMLButtonElement>) => {
        if (!config) return;
        const newTheme = config.theme === 'light' ? 'dark' : 'light';

        if ('startViewTransition' in document && !isLinux()) {
            const x = event.clientX;
            const y = event.clientY;
            const endRadius = Math.hypot(
                Math.max(x, window.innerWidth - x),
                Math.max(y, window.innerHeight - y)
            );

            // @ts-ignore
            const transition = document.startViewTransition(async () => {
                saveConfig({ ...config, theme: newTheme }, true);
            });

            transition.ready.then(() => {
                const isDarkMode = newTheme === 'dark';
                const clipPath = isDarkMode
                    ? [`circle(${endRadius}px at ${x}px ${y}px)`, `circle(0px at ${x}px ${y}px)`]
                    : [`circle(0px at ${x}px ${y}px)`, `circle(${endRadius}px at ${x}px ${y}px)`];

                document.documentElement.animate(
                    { clipPath },
                    {
                        duration: 500,
                        easing: 'ease-in-out',
                        fill: 'forwards',
                        pseudoElement: isDarkMode ? '::view-transition-old(root)' : '::view-transition-new(root)'
                    }
                );
            });
        } else {
            await saveConfig({ ...config, theme: newTheme }, true);
        }
    };

    const handleLogout = () => {
        sessionStorage.removeItem('abv_admin_api_key');
        localStorage.removeItem('abv_admin_api_key');
        window.location.reload();
    };

    const toggleTrayMode = async () => {
        if (!config) return;
        const newTrayMode = !config.tray_mode;
        
        if (newTrayMode && isTauri()) {
            // Hide the window immediately before the React state triggers the resize
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().hide();
        }
        
        await saveConfig({ ...config, tray_mode: newTrayMode }, false);
    };

    return (
        <aside className={`flex flex-col h-full bg-[#fafafa] dark:bg-[#1c1c1c] border-r border-[#dfdfdf] dark:border-[#2b2b2b] relative z-50 transition-all duration-300 ease-in-out ${isSidebarCollapsed ? 'w-[80px]' : 'w-[260px]'}`}>
            {/* Drag region for Tauri */}
            {isTauri() && (
                <div
                    className="absolute top-0 left-0 right-0 h-10 cursor-default"
                    style={{ zIndex: 10, WebkitAppRegion: 'drag' } as any}
                    data-tauri-drag-region
                />
            )}
            
            {/* Logo Section */}
            <div className={`pt-10 pb-6 relative z-20 ${isSidebarCollapsed ? 'px-0 flex justify-center' : 'px-6'}`}>
                <NavLogo isCollapsed={isSidebarCollapsed} />
            </div>

            <nav className={`flex-1 space-y-1 overflow-y-auto relative z-20 custom-scrollbar mt-2 ${isSidebarCollapsed ? 'px-2' : 'px-4'}`}>
                {navItems.map((item) => (
                    <SidebarItem
                        key={item.path}
                        path={item.path}
                        label={item.label}
                        icon={item.icon}
                        isCollapsed={isSidebarCollapsed}
                    />
                ))}
            </nav>

            {/* Bottom Actions */}
            <div className={`p-4 border-t border-[#dfdfdf] dark:border-[#2b2b2b] bg-[#ffffff] dark:bg-[#1c1c1c] relative z-20 flex flex-col ${isSidebarCollapsed ? 'items-center' : ''}`}>
                <div className={`flex items-center gap-2 mb-2 ${isSidebarCollapsed ? 'flex-col' : 'justify-around'}`}>

                    {/* Tray Mode Toggle */}
                    {isTauri() && (
                        <button
                            onClick={toggleTrayMode}
                            className="w-8 h-8 rounded-md flex items-center justify-center transition-colors duration-200 text-[#707070] hover:text-[#171717] hover:bg-gray-200 dark:text-[#9a9a9a] dark:hover:text-[#ffffff] dark:hover:bg-white/10"
                            title={config?.tray_mode ? t('nav.full_app_mode', 'Switch to Full App') : t('nav.menu_bar_mode', 'Switch to Menu Bar')}
                        >
                            {config?.tray_mode ? <AppWindow className="w-4 h-4" /> : <PanelTop className="w-4 h-4" />}
                        </button>
                    )}

                    {/* Theme Toggle */}
                    <button
                        onClick={toggleTheme}
                        className="w-8 h-8 rounded-md flex items-center justify-center transition-colors duration-200 text-[#707070] hover:text-[#171717] hover:bg-gray-200 dark:text-[#9a9a9a] dark:hover:text-[#ffffff] dark:hover:bg-white/10"
                        title={config?.theme === 'light' ? t('nav.theme_to_dark') : t('nav.theme_to_light')}
                    >
                        {config?.theme === 'light' ? (
                            <Moon className="w-4 h-4" />
                        ) : (
                            <Sun className="w-4 h-4" />
                        )}
                    </button>

                    {/* Language Toggle removed */}
                    
                    {/* Toggle Sidebar Button */}
                    <button
                        onClick={toggleSidebar}
                        className="w-8 h-8 rounded-md flex items-center justify-center transition-colors duration-200 text-[#707070] hover:text-[#171717] hover:bg-gray-200 dark:text-[#9a9a9a] dark:hover:text-[#ffffff] dark:hover:bg-white/10"
                        title={isSidebarCollapsed ? t('nav.expand_sidebar', 'Expand Sidebar') : t('nav.collapse_sidebar', 'Collapse Sidebar')}
                    >
                        {isSidebarCollapsed ? <PanelLeftOpen className="w-4 h-4" /> : <PanelLeftClose className="w-4 h-4" />}
                    </button>
                </div>

                {!isTauri() && (
                    <button
                        onClick={handleLogout}
                        className={`mt-2 py-1.5 px-3 rounded-md flex items-center justify-center gap-2 transition-colors duration-200 text-sm font-medium hover:bg-gray-200 dark:hover:bg-white/10 text-[#171717] dark:text-[#ffffff] ${isSidebarCollapsed ? 'w-8 h-8 p-0' : 'w-full'}`}
                        title={t('nav.logout', 'Logout')}
                    >
                        <LogOut className="w-4 h-4" />
                        {!isSidebarCollapsed && <span>{t('nav.logout', 'Logout')}</span>}
                    </button>
                )}
            </div>
        </aside>
    );
}
