import { create } from 'zustand';

interface ViewState {
    isSidebarCollapsed: boolean;
    toggleSidebar: () => void;
}

export const useViewStore = create<ViewState>((set) => ({
    isSidebarCollapsed: false,
    toggleSidebar: () => set((state) => ({ isSidebarCollapsed: !state.isSidebarCollapsed })),
}));
