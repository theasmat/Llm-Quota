/**
 * 
 * 
 * ：（ HTTPS  localhost），navigator.clipboard  undefined。
 *  execCommand('copy') ， HTTP （ Docker IP ）。
 */
export async function copyToClipboard(text: string): Promise<boolean> {
    // 1.  Clipboard API
    if (navigator.clipboard && window.isSecureContext) {
        try {
            await navigator.clipboard.writeText(text);
            return true;
        } catch (err) {
            console.error('Clipboard API :', err);
        }
    }

    // 2.  execCommand('copy') 
    try {
        const textArea = document.createElement('textarea');
        textArea.value = text;

        //  textarea ， DOM  copy
        textArea.style.position = 'fixed';
        textArea.style.left = '-9999px';
        textArea.style.top = '0';
        document.body.appendChild(textArea);

        textArea.focus();
        textArea.select();

        const successful = document.execCommand('copy');
        document.body.removeChild(textArea);

        return successful;
    } catch (err) {
        console.error('execCommand :', err);
        return false;
    }
}
