import { useState, useEffect } from 'react';
import CalculatorTab from './components/CalculatorTab';
import DbManagerTab from './components/DbManagerTab';
import SettingsTab from './components/SettingsTab';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [activeTab, setActiveTab] = useState<'calculator' | 'database' | 'settings'>('calculator');
  const [apiKeyConfigured, setApiKeyConfigured] = useState(false);

  useEffect(() => {
    // Check if API key is configured
    checkApiKey();
  }, []);

  const checkApiKey = async () => {
    try {
      const configured = await invoke<boolean>('get_api_key_configured');
      setApiKeyConfigured(configured);
    } catch (error) {
      console.error('Failed to check API key:', error);
    }
  };

  const handleApiKeySet = () => {
    setApiKeyConfigured(true);
    setActiveTab('calculator');
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <div className="container mx-auto px-4 py-6">
        <h1 className="text-3xl font-bold text-gray-800 mb-6">
          Post-Judgment Interest Calculator
        </h1>

        {!apiKeyConfigured && (
          <div className="mb-6 p-4 bg-yellow-100 border border-yellow-400 rounded-lg">
            <p className="text-sm text-yellow-800">
              ⚠️ Please configure your FRED API key in the Settings tab to enable federal rate calculations.
            </p>
          </div>
        )}

        <div className="bg-white rounded-lg shadow-md">
          <div className="border-b border-gray-200">
            <nav className="flex">
              <button
                className={`px-6 py-3 text-sm font-medium ${
                  activeTab === 'calculator'
                    ? 'text-blue-600 border-b-2 border-blue-600'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
                onClick={() => setActiveTab('calculator')}
              >
                Calculator
              </button>
              <button
                className={`px-6 py-3 text-sm font-medium ${
                  activeTab === 'database'
                    ? 'text-blue-600 border-b-2 border-blue-600'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
                onClick={() => setActiveTab('database')}
              >
                Database Manager
              </button>
              <button
                className={`px-6 py-3 text-sm font-medium ${
                  activeTab === 'settings'
                    ? 'text-blue-600 border-b-2 border-blue-600'
                    : 'text-gray-500 hover:text-gray-700'
                }`}
                onClick={() => setActiveTab('settings')}
              >
                Settings
              </button>
            </nav>
          </div>

          <div className="p-6">
            {activeTab === 'calculator' && <CalculatorTab />}
            {activeTab === 'database' && <DbManagerTab />}
            {activeTab === 'settings' && (
              <SettingsTab onApiKeySet={handleApiKeySet} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
