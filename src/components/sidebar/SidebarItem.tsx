import React from 'react';
import { NavLink } from 'react-router-dom';

interface SidebarItemProps {
    path: string;
    label: string;
    icon: React.ElementType;
    isCollapsed?: boolean;
}

export function SidebarItem({ path, label, icon: Icon, isCollapsed }: SidebarItemProps) {
    return (
        <NavLink
            to={path}
            title={isCollapsed ? label : undefined}
            className={({ isActive }) =>
                `group flex items-center ${isCollapsed ? 'justify-center px-0' : 'gap-3 px-3'} py-1.5 my-1 rounded-md transition-all duration-200 ${
                    isActive
                        ? 'bg-[#3ecf8e] dark:bg-[#3ecf8e] text-[#171717] font-medium'
                        : 'text-[#707070] dark:text-[#9a9a9a] hover:bg-[#ededed] dark:hover:bg-[#202020] hover:text-[#171717] dark:hover:text-[#ffffff] font-normal'
                }`
            }
        >
            {({ isActive }) => (
                <>
                    {/* Icon */}
                    <div className="flex items-center justify-center">
                        <Icon className={`w-4 h-4 ${isActive ? 'text-[#171717]' : ''}`} />
                    </div>

                    {/* Label */}
                    {!isCollapsed && (
                        <span className="text-sm tracking-normal transition-all duration-200 truncate">
                            {label}
                        </span>
                    )}
                </>
            )}
        </NavLink>
    );
}
