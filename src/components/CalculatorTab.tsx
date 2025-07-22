import React, { useState, useEffect } from 'react';
import DatePicker from 'react-datepicker';
import 'react-datepicker/dist/react-datepicker.css';
import { invoke } from '@tauri-apps/api/core';
import { CalcRequest, CalcResponse, StateRate } from '../types';

const CalculatorTab: React.FC = () => {
  const [formData, setFormData] = useState<CalcRequest>({
    judgment_date: new Date().toISOString().split('T')[0],
    is_federal: true,
    state: '',
    amount: 0,
    from_date: new Date().toISOString().split('T')[0],
    to_date: new Date().toISOString().split('T')[0],
  });

  const [states, setStates] = useState<StateRate[]>([]);
  const [result, setResult] = useState<CalcResponse | null>(null);
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadStates();
  }, []);

  const loadStates = async () => {
    try {
      const stateRates = await invoke<StateRate[]>('get_all_state_rates');
      setStates(stateRates.filter(s => s.state !== 'Federal'));
      // Set default state
      if (stateRates.length > 0 && !formData.state) {
        setFormData(prev => ({ ...prev, state: stateRates[0].state }));
      }
    } catch (err) {
      console.error('Failed to load states:', err);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setResult(null);
    setLoading(true);

    try {
      const response = await invoke<CalcResponse>('calculate', {
        request: formData,
      });
      setResult(response);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleDateChange = (field: 'judgment_date' | 'from_date' | 'to_date', date: Date | null) => {
    if (date) {
      setFormData(prev => ({
        ...prev,
        [field]: date.toISOString().split('T')[0],
      }));
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount);
  };

  return (
    <div className="space-y-6">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Jurisdiction Type
            </label>
            <select
              value={formData.is_federal ? 'federal' : 'state'}
              onChange={(e) => setFormData(prev => ({ 
                ...prev, 
                is_federal: e.target.value === 'federal' 
              }))}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="federal">Federal</option>
              <option value="state">State</option>
            </select>
          </div>

          {!formData.is_federal && (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                State
              </label>
              <select
                value={formData.state}
                onChange={(e) => setFormData(prev => ({ ...prev, state: e.target.value }))}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {states.map(state => (
                  <option key={state.id} value={state.state}>
                    {state.state}
                  </option>
                ))}
              </select>
            </div>
          )}
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Judgment Date
            </label>
            <DatePicker
              selected={new Date(formData.judgment_date)}
              onChange={(date) => handleDateChange('judgment_date', date)}
              dateFormat="yyyy-MM-dd"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Interest Start Date
            </label>
            <DatePicker
              selected={new Date(formData.from_date)}
              onChange={(date) => handleDateChange('from_date', date)}
              dateFormat="yyyy-MM-dd"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Interest End Date
            </label>
            <DatePicker
              selected={new Date(formData.to_date)}
              onChange={(date) => handleDateChange('to_date', date)}
              dateFormat="yyyy-MM-dd"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Judgment Amount
          </label>
          <div className="relative">
            <span className="absolute inset-y-0 left-0 pl-3 flex items-center text-gray-500">
              $
            </span>
            <input
              type="number"
              value={formData.amount || ''}
              onChange={(e) => setFormData(prev => ({ 
                ...prev, 
                amount: parseFloat(e.target.value) || 0 
              }))}
              step="0.01"
              min="0"
              className="w-full pl-8 pr-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="0.00"
            />
          </div>
        </div>

        <div className="pt-4">
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? 'Calculating...' : 'Calculate Interest'}
          </button>
        </div>
      </form>

      {error && (
        <div className="p-4 bg-red-100 border border-red-400 rounded-lg">
          <p className="text-red-700">{error}</p>
        </div>
      )}

      {result && (
        <div className="bg-gray-50 p-6 rounded-lg space-y-4">
          <h3 className="text-lg font-semibold text-gray-800">Calculation Result</h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <p className="text-sm text-gray-500">Interest Rate</p>
              <p className="text-lg font-medium">{result.rate.toFixed(2)}%</p>
              <p className="text-xs text-gray-500">{result.rate_source}</p>
            </div>
            
            <div>
              <p className="text-sm text-gray-500">Days</p>
              <p className="text-lg font-medium">{result.days}</p>
            </div>
            
            <div>
              <p className="text-sm text-gray-500">Interest Amount</p>
              <p className="text-lg font-medium">{formatCurrency(result.interest_amount)}</p>
            </div>
            
            <div>
              <p className="text-sm text-gray-500">Total Amount</p>
              <p className="text-lg font-medium text-green-600">
                {formatCurrency(result.total_amount)}
              </p>
            </div>
          </div>

          <div className="pt-4 border-t border-gray-200">
            <p className="text-sm text-gray-600 italic">
              {result.disclaimer}
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

export default CalculatorTab;
