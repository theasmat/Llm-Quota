import { Outlet } from 'react-router-dom';

import { useEffect } from 'react';
import { isTauri } from '../../utils/env';
import { ensureFullViewState } from '../../utils/windowManager';
import { useConfigStore } from '../../stores/useConfigStore';
import { TrayUI } from '../../platform';
import { Sidebar } from '../sidebar/Sidebar';
import BackgroundTaskRunner from '../common/BackgroundTaskRunner';
import ToastContainer from '../common/ToastContainer';

function Layout() {
    const { config } = useConfigStore();

    // Ensure correct window state when in Full View (not Menu Bar mode)
    // This handles the case where the app was closed in Menu Bar mode (small size, no decorations)
    // and restarted (defaults to Full View state but keeps last window properties)
    useEffect(() => {
        if (!config?.tray_mode && isTauri()) {
            ensureFullViewState();
        }
    }, [config?.tray_mode]);

    if (config?.tray_mode) {
        return (
            <>
                <BackgroundTaskRunner />
                <ToastContainer />
                <TrayUI />
            </>
        );
    }

    return (
        <div className="h-screen flex flex-row bg-white dark:bg-[#1c1c1c]">
            <BackgroundTaskRunner />
            <ToastContainer />
            <Sidebar />
            
            <main className="flex-1 overflow-y-auto overflow-x-hidden flex flex-col relative bg-transparent">
                {/* Global window drag region - only at the very top of main content */}
                {isTauri() && (
                    <div
                        className="sticky top-0 left-0 right-0 h-9 z-40"
                        style={{
                            backgroundColor: 'transparent',
                            cursor: 'default',
                            WebkitAppRegion: 'drag'
                        } as any}
                        data-tauri-drag-region
                    />
                )}
                
                <div className="flex-1 p-6 z-10 relative">
                    <Outlet />
                </div>
            </main>
        </div>
    );
}

export default Layout;

