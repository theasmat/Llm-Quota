import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { isTauri } from './env';

/**
 * Enter Tray UI state
 * @param contentHeight The height of the content to fit
 */
export const enterTrayUIState = async (contentHeight: number) => {
    if (!isTauri()) return;
    try {
        const win = getCurrentWindow();
        
        // Remember if the window was visible before we start changing its properties
        const wasVisible = await win.isVisible();

        // Hide window decorations (title bar) first to ensure accurate sizing
        await win.setDecorations(false);

        // Set window size: width 300, height = content height 
        await win.setSize(new LogicalSize(300, contentHeight+2));

        await win.setAlwaysOnTop(true);
        // Enable window shadow
        await win.setShadow(true);
        // Disable resizing in tray mode
        await win.setResizable(false);
        
        // macOS NSWindow property changes (like setAlwaysOnTop or setDecorations) 
        // can implicitly unhide the window. If it was hidden, force it back into hiding!
        if (!wasVisible) {
            await win.hide();
        }
    } catch (error) {
        console.error('Failed to enter tray UI state:', error);
    }
};

/**
 * Exit Tray UI state and restore default window state
 */
export const exitTrayUIState = async () => {
    if (!isTauri()) return;
    try {
        const win = getCurrentWindow();
        // Restore to a reasonable default size
        await win.setSize(new LogicalSize(1200, 800));
        await win.setAlwaysOnTop(false);
        await win.center();
        // Restore window decorations (title bar)
        await win.setDecorations(true);
        // Re-enable resizing
        await win.setResizable(true);
    } catch (error) {
        console.error('Failed to exit tray UI state:', error);
    }
};

/**
 * Ensure window is in valid full view state (Self-healing)
 * Used on app startup to recover from improper shutdown in mini mode
 */
export const ensureFullViewState = async () => {
    if (!isTauri()) return;
    try {
        const win = getCurrentWindow();
        const size = await win.outerSize();
        // If window is suspiciously narrow (likely leftover from Mini View), restore default size
        if (size.width < 500) {
            await win.setSize(new LogicalSize(1200, 800));
            await win.center();
        }
        // Always enforce standard window properties for Full View
        await win.setDecorations(true);
        await win.setResizable(true);
        await win.setAlwaysOnTop(false);
    } catch (error) {
        console.error('Failed to ensure full view state:', error);
    }
};
