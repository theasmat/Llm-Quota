import { useEffect, useRef } from 'react';
import { useAccountStore } from '../../stores/useAccountStore';
import { isTauri } from '../../utils/env';
import { enterTrayUIState } from '../../utils/windowManager';
import { useTranslation } from 'react-i18next';
import clsx from 'clsx';
import { Maximize2, Sparkles, User, Database, Cpu, Zap, Power } from 'lucide-react';

import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export default function WinSystemTrayCard() {
    const { currentAccount, refreshQuota, fetchCurrentAccount } = useAccountStore();
    const { t } = useTranslation();
    const containerRef = useRef<HTMLDivElement>(null);

    // Fetch initial account state for the tray window
    useEffect(() => {
        fetchCurrentAccount();
    }, [fetchCurrentAccount]);

    // Enter tray mode & Auto-resize based on content
    useEffect(() => {
        const adjustSize = async () => {
            if (isTauri() && containerRef.current) {
                const height = containerRef.current.scrollHeight;
                await enterTrayUIState(height);
            }
        };

        const timer = setTimeout(adjustSize, 50);
        return () => clearTimeout(timer);
    }, [currentAccount]);

    // Focus lost & Inactivity timer (10s)
    useEffect(() => {
        if (!isTauri()) return;
        const win = getCurrentWindow();
        
        // Hide on blur
        const unlistenFocus = win.onFocusChanged(({ payload: focused }) => {
            if (!focused) {
                win.hide();
            }
        });

        // 10s inactivity timer
        let timeout: ReturnType<typeof setTimeout>;
        const resetTimer = () => {
            clearTimeout(timeout);
            timeout = setTimeout(() => {
                win.hide();
            }, 10000);
        };

        resetTimer();
        window.addEventListener('mousemove', resetTimer);
        window.addEventListener('keydown', resetTimer);

        return () => {
            unlistenFocus.then(f => f());
            clearTimeout(timeout);
            window.removeEventListener('mousemove', resetTimer);
            window.removeEventListener('keydown', resetTimer);
        };
    }, []);

    // Auto-refresh when tray icon is clicked
    useEffect(() => {
        if (!isTauri()) return;
        const unlisten = listen('tray-opened', async () => {
            if (currentAccount?.id) {
                refreshQuota(currentAccount.id).catch(console.error);
            }
        });
        
        return () => {
            unlisten.then(f => f());
        };
    }, [currentAccount?.id, refreshQuota]);

    const handleMaximize = async () => {
        if (!isTauri()) return;
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('show_main_window');
        
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().hide();
        
        const { useConfigStore } = await import('../../stores/useConfigStore');
        const configStore = useConfigStore.getState();
        if (configStore.config) {
            await configStore.saveConfig({ ...configStore.config, tray_mode: false }, false);
        }
    };

    // Extract specific models
    const geminiProModel = currentAccount?.quota?.models
        .filter(m =>
            m.name.toLowerCase() === 'gemini-3-pro-high'
            || m.name.toLowerCase() === 'gemini-3-pro-low'
            || m.name.toLowerCase() === 'gemini-3.1-pro-high'
            || m.name.toLowerCase() === 'gemini-3.1-pro-low'
        )
        .sort((a, b) => (a.percentage || 0) - (b.percentage || 0))[0];

    const geminiFlashModel = currentAccount?.quota?.models.find(m => m.name.toLowerCase() === 'gemini-3-flash');

    const claudeGroupNames = [
        'claude-opus-4-6-thinking',
        'claude'
    ];
    const claudeModel = currentAccount?.quota?.models
        .filter(m => claudeGroupNames.includes(m.name.toLowerCase()))
        .sort((a, b) => (a.percentage || 0) - (b.percentage || 0))[0];

    const renderModelRow = (model: any, displayName: string, colorClass: string, Icon: any) => {
        if (!model) return null;
        const getStatusColor = (p: number) => {
            if (p >= 50) return 'text-emerald-500';
            if (p >= 20) return 'text-amber-500';
            return 'text-rose-500';
        };
        const getBarColor = (p: number) => {
            if (p >= 50) return colorClass === 'cyan' ? 'bg-gradient-to-r from-cyan-400 to-blue-500' : 'bg-gradient-to-r from-emerald-400 to-teal-500';
            if (p >= 20) return colorClass === 'cyan' ? 'bg-gradient-to-r from-orange-400 to-amber-500' : 'bg-gradient-to-r from-amber-400 to-yellow-500';
            return 'bg-gradient-to-r from-rose-400 to-red-500';
        };
        const getGlow = (p: number) => {
            if (p >= 50) return colorClass === 'cyan' ? 'shadow-[0_0_10px_rgba(34,211,238,0.5)]' : 'shadow-[0_0_10px_rgba(52,211,153,0.5)]';
            if (p >= 20) return colorClass === 'cyan' ? 'shadow-[0_0_10px_rgba(251,146,60,0.5)]' : 'shadow-[0_0_10px_rgba(251,191,36,0.5)]';
            return 'shadow-[0_0_10px_rgba(244,63,94,0.5)]';
        };

        return (
            <div className="space-y-2 py-2 px-1">
                <div className="flex justify-between items-center">
                    <div className="flex items-center gap-2">
                        <div className={clsx("p-1.5 rounded-md bg-gray-100 dark:bg-white/5", colorClass === 'cyan' ? 'text-cyan-500' : 'text-emerald-500')}>
                            <Icon className="w-3.5 h-3.5" />
                        </div>
                        <span className="text-xs font-semibold text-gray-700 dark:text-gray-200">{displayName}</span>
                    </div>
                    <span className={clsx("text-xs font-bold font-mono", getStatusColor(model.percentage))}>
                        {model.percentage}%
                    </span>
                </div>
                <div className="w-full bg-gray-100 dark:bg-white/5 rounded-full h-1.5 overflow-hidden">
                    <div 
                        className={clsx("h-full rounded-full transition-all duration-1000 ease-out", getBarColor(model.percentage), getGlow(model.percentage))} 
                        style={{ width: `${model.percentage}%` }} 
                    />
                </div>
            </div>
        );
    };

    const handleQuit = async () => {
        if (!isTauri()) return;
        const { exit } = await import('@tauri-apps/plugin-process');
        await exit(0);
    };

    return (
        <div className="w-full h-screen bg-transparent select-none pb-2">
            <div
                ref={containerRef}
                className="w-[300px] flex flex-col bg-white/95 dark:bg-[#111111]/95 backdrop-blur-xl shadow-2xl overflow-hidden rounded-xl border border-black/5 dark:border-white/10"
            >
                {/* Header */}
                <div className="p-4 pb-3 border-b border-gray-200/50 dark:border-white/5">
                    <div className="flex items-center gap-3">
                        <div className="relative">
                            <div className="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white shadow-inner">
                                <User className="w-4 h-4" />
                            </div>
                            <div className="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full bg-green-500 border-2 border-white dark:border-[#111111]"></div>
                        </div>
                        <div className="flex flex-col">
                            <span className="text-[10px] font-medium text-gray-400 uppercase tracking-wider">Active Account</span>
                            <span className="text-sm font-bold text-gray-900 dark:text-white truncate max-w-[200px]">
                                {currentAccount?.email || 'No Account'}
                            </span>
                        </div>
                    </div>
                </div>
                
                {/* Quotas */}
                <div className="p-4 space-y-1">
                    {renderModelRow(geminiProModel, 'Gemini 3.1 Pro', 'emerald', Sparkles)}
                    {renderModelRow(geminiFlashModel, 'Gemini 3 Flash', 'emerald', Zap)}
                    {renderModelRow(claudeModel, t('common.claude_series', 'Claude Opus/Sonnet'), 'cyan', Cpu)}
                    
                    {!geminiProModel && !geminiFlashModel && !claudeModel && (
                        <div className="flex flex-col items-center justify-center py-6 text-gray-400 gap-2">
                            <Database className="w-6 h-6 opacity-50" />
                            <span className="text-xs">No active quota data</span>
                        </div>
                    )}
                </div>
                
                {/* Footer Actions */}
                <div className="p-2 bg-gray-50/50 dark:bg-black/20 border-t border-gray-200/50 dark:border-white/5 space-y-1">
                    <button 
                        onClick={handleMaximize}
                        className="group w-full flex items-center justify-between px-3 py-2 text-xs font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-white dark:hover:bg-white/10 rounded-lg transition-all duration-200"
                    >
                        <span>{t('nav.full_app_mode', 'Open Full App')}</span>
                        <Maximize2 className="w-3.5 h-3.5 opacity-50 group-hover:opacity-100 group-hover:scale-110 transition-all" />
                    </button>
                    <button 
                        onClick={handleQuit}
                        className="group w-full flex items-center justify-between px-3 py-2 text-xs font-medium text-gray-600 dark:text-gray-300 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-500/10 rounded-lg transition-all duration-200"
                    >
                        <span>{t('common.quit', 'Quit')}</span>
                        <Power className="w-3.5 h-3.5 opacity-50 group-hover:opacity-100 group-hover:scale-110 transition-all" />
                    </button>
                </div>
            </div>
        </div>
    );
}
