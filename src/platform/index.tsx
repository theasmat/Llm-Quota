import React, { useMemo } from 'react';
import MacMenuBarCard from './macos/MenuBarCard';
import WinSystemTrayCard from './windows/SystemTrayCard';

export const TrayUI: React.FC = () => {
    const isMac = useMemo(() => window.navigator.userAgent.includes('Mac OS'), []);
    
    // In the future, we can add linux or other specific versions
    if (isMac) {
        return <MacMenuBarCard />;
    }
    
    return <WinSystemTrayCard />;
};
