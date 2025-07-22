import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SettingsTabProps {
  onApiKeySet: () => void;
}

const SettingsTab: React.FC<SettingsTabProps> = ({ onApiKeySet }) => {
  const [apiKey, setApiKey] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    setSuccess('');

    try {
      // Validate the API key first
      const isValid = await invoke<boolean>('validate_api_key_command', { apiKey });
      
      if (!isValid) {
        setError('Invalid API key. Please check your key and try again.');
        return;
      }

      // Save the API key
      await invoke('set_api_key', { apiKey });
      setSuccess('API key saved successfully!');
      onApiKeySet();
      
      // Clear the input after successful save
      setTimeout(() => {
        setApiKey('');
      }, 2000);
    } catch (err) {
      setError('Failed to save API key: ' + err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-semibold text-gray-800 mb-6">Settings</h2>
      
      <div className="space-y-6">
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h3 className="font-medium text-blue-900 mb-2">FRED API Key Configuration</h3>
          <p className="text-sm text-blue-700 mb-3">
            To fetch federal interest rates, you need a free API key from the Federal Reserve Economic Data (FRED) service.
          </p>
          <ol className="text-sm text-blue-700 space-y-1 list-decimal list-inside">
            <li>Visit <a href="https://fred.stlouisfed.org/docs/api/api_key.html" className="underline hover:text-blue-800" target="_blank" rel="noopener noreferrer">FRED API Key Registration</a></li>
            <li>Create a free account or sign in</li>
            <li>Request an API key (instant approval)</li>
            <li>Copy your API key and paste it below</li>
          </ol>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="apiKey" className="block text-sm font-medium text-gray-700 mb-1">
              FRED API Key
            </label>
            <input
              id="apiKey"
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter your FRED API key"
              required
            />
            <p className="mt-1 text-xs text-gray-500">
              Your API key is stored locally and never shared.
            </p>
          </div>

          <button
            type="submit"
            disabled={loading || !apiKey}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? 'Validating...' : 'Save API Key'}
          </button>
        </form>

        {error && (
          <div className="p-4 bg-red-100 border border-red-400 rounded-lg">
            <p className="text-red-700">{error}</p>
          </div>
        )}

        {success && (
          <div className="p-4 bg-green-100 border border-green-400 rounded-lg">
            <p className="text-green-700">{success}</p>
          </div>
        )}

        <div className="pt-6 border-t border-gray-200">
          <h3 className="font-medium text-gray-800 mb-3">About This Application</h3>
          <div className="text-sm text-gray-600 space-y-2">
            <p>
              <strong>Version:</strong> 0.1.0
            </p>
            <p>
              <strong>Purpose:</strong> Calculate post-judgment interest for federal and state jurisdictions.
            </p>
            <p>
              <strong>Data Sources:</strong>
            </p>
            <ul className="list-disc list-inside ml-4 space-y-1">
              <li>Federal rates: FRED API (1-Year Treasury Constant Maturity Rate)</li>
              <li>State rates: Local SQLite database (editable via Database Manager)</li>
            </ul>
            <p className="text-xs text-gray-500 mt-4">
              Note: State interest rates may change periodically. Please verify rates with official sources
              and update the database as needed to ensure accuracy.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsTab;
