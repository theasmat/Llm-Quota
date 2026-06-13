import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import LogoIcon from '../../../src-tauri/icons/icon.png';

export function NavLogo({ isCollapsed }: { isCollapsed?: boolean }) {
    const { t } = useTranslation();

    return (
        <Link to="/" draggable="false" className={`flex w-full min-w-0 items-center gap-2 text-lg font-medium tracking-tight text-[#171717] dark:text-[#ffffff] ${isCollapsed ? 'justify-center ml-0' : ''}`}>
            <div className="relative flex items-center justify-center">
                <img
                    src={LogoIcon}
                    alt="Logo"
                    className="w-8 h-8 cursor-pointer active:scale-95 transition-transform relative z-10"
                    draggable="false"
                />
            </div>

            {/* Collapsed Hide */}
            {!isCollapsed && <span className="hidden sm:inline text-nowrap">{t('common.app_name', 'Llm Quota')}</span>}
        </Link>
    );
}
