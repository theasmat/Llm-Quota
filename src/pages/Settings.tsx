import { useState, useEffect } from 'react';
import { Save } from 'lucide-react';
import { useConfigStore } from '../stores/useConfigStore';
import { AppConfig } from '../types/config';
import { showToast } from '../components/common/ToastContainer';
import { useTranslation } from 'react-i18next';

function Settings() {
    const { t } = useTranslation();
    const { config, loadConfig, saveConfig, updateTheme } = useConfigStore();
    const [activeTab, setActiveTab] = useState<'general' | 'account'>('general');
    const [formData, setFormData] = useState<AppConfig>({
    theme: 'system',
    language: 'zh-CN',
    auto_check_updates: true,
  });

    useEffect(() => {
        loadConfig();
    }, [loadConfig]);

    useEffect(() => {
        if (config) {
            setFormData(config);
        }
    }, [config]);

    const handleSave = async () => {
        try {
            await saveConfig(formData);
            showToast(t('common.saved'), 'success');
        } catch (error) {
            showToast(`${t('common.error')}: ${error}`, 'error');
        }
    };

    return (
        <div className="h-full w-full overflow-y-auto">
            <div className="p-5 space-y-4 max-w-7xl mx-auto">
                <div className="flex justify-between items-center">
                    <div className="flex items-center gap-1 bg-gray-100 dark:bg-base-200 rounded-full p-1 w-fit">
                        <button
                            className={`px-6 py-2 rounded-full text-sm font-medium transition-all ${activeTab === 'general'
                                ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 shadow-sm'
                                : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                                }`}
                            onClick={() => setActiveTab('general')}
                        >
                            {t('settings.tabs.general')}
                        </button>
                        <button
                            className={`px-6 py-2 rounded-full text-sm font-medium transition-all ${activeTab === 'general'
                                ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 shadow-sm'
                                : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                                }`}
                            onClick={() => setActiveTab('general')}
                        >
                            {t('settings.tabs.general')}
                        </button>
                    </div>

                    <button
                        className="px-4 py-2 bg-blue-500 text-white text-sm rounded-lg hover:bg-blue-600 transition-colors flex items-center gap-2 shadow-sm"
                        onClick={handleSave}
                    >
                        <Save className="w-4 h-4" />
                        {t('settings.save')}
                    </button>
                </div>

                <div className="bg-white dark:bg-base-100 rounded-2xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
                    {activeTab === 'general' && (
                        <div className="space-y-6">
                            <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">{t('settings.general.title')}</h2>

                            <div>
                                <label className="block text-sm font-medium text-gray-900 dark:text-base-content mb-2">{t('settings.general.theme')}</label>
                                <select
                                    className="w-full px-4 py-4 border border-gray-200 dark:border-base-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 dark:text-base-content bg-gray-50 dark:bg-base-200"
                                    value={formData.theme}
                                    onChange={(e) => {
                                        const newTheme = e.target.value;
                                        setFormData({ ...formData, theme: newTheme as any });
                                        updateTheme(newTheme);
                                    }}
                                >
                                    <option value="light">{t('settings.general.theme_light')}</option>
                                    <option value="dark">{t('settings.general.theme_dark')}</option>
                                    <option value="system">{t('settings.general.theme_system')}</option>
                                </select>
                            </div>
                        </div>
                    )}


                </div>
            </div>
        </div>
    );
}

export default Settings;
