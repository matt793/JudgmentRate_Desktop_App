import React, { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { StateRate } from '../types';
import { AgGridReact } from 'ag-grid-react';
import { ColDef } from 'ag-grid-community';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';

const DbManagerTab: React.FC = () => {
  const [stateRates, setStateRates] = useState<StateRate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string>('');
  const [editingRate, setEditingRate] = useState<StateRate | null>(null);

  useEffect(() => {
    loadStateRates();
  }, []);

  const loadStateRates = async () => {
    try {
      const rates = await invoke<StateRate[]>('get_all_state_rates');
      setStateRates(rates);
      setError('');
    } catch (err) {
      setError('Failed to load state rates: ' + err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdate = async (stateRate: StateRate) => {
    try {
      await invoke('update_state_rate_command', { stateRate });
      await loadStateRates();
      setEditingRate(null);
    } catch (err) {
      setError('Failed to update state rate: ' + err);
    }
  };

  const handleDelete = async (id: number) => {
    if (window.confirm('Are you sure you want to delete this state rate?')) {
      try {
        await invoke('delete_state_rate_command', { id });
        await loadStateRates();
      } catch (err) {
        setError('Failed to delete state rate: ' + err);
      }
    }
  };

  const columnDefs: ColDef<StateRate>[] = useMemo(() => [
    { 
      field: 'state', 
      headerName: 'State',
      sortable: true,
      filter: true,
      width: 200
    },
    { 
      field: 'rate', 
      headerName: 'Rate (%)',
      sortable: true,
      width: 100,
      valueFormatter: params => params.value.toFixed(2) + '%'
    },
    { 
      field: 'is_variable', 
      headerName: 'Variable',
      width: 100,
      cellRenderer: (params: any) => params.value ? '✓' : ''
    },
    { 
      field: 'plus_percentage', 
      headerName: 'Plus %',
      width: 100,
      valueFormatter: params => params.value ? '+' + params.value.toFixed(2) + '%' : ''
    },
    { 
      field: 'update_frequency', 
      headerName: 'Update Freq',
      width: 120
    },
    { 
      field: 'last_update', 
      headerName: 'Last Updated',
      width: 120
    },
    { 
      field: 'notes', 
      headerName: 'Notes',
      flex: 1,
      minWidth: 200
    },
    {
      headerName: 'Actions',
      width: 120,
      cellRenderer: (params: any) => (
        <div className="flex gap-2">
          <button
            onClick={() => setEditingRate(params.data)}
            className="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Edit
          </button>
          <button
            onClick={() => handleDelete(params.data.id)}
            className="px-2 py-1 text-xs bg-red-500 text-white rounded hover:bg-red-600"
          >
            Delete
          </button>
        </div>
      )
    }
  ], []);

  if (loading) {
    return <div className="text-center py-8">Loading...</div>;
  }

  if (error) {
    return (
      <div className="p-4 bg-red-100 border border-red-400 rounded-lg">
        <p className="text-red-700">{error}</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold text-gray-800">State Interest Rates Database</h2>
        <div className="text-sm text-gray-600">
          Total States: {stateRates.length}
        </div>
      </div>

      <div className="ag-theme-alpine" style={{ height: 500 }}>
        <AgGridReact
          rowData={stateRates}
          columnDefs={columnDefs}
          defaultColDef={{
            resizable: true,
          }}
          animateRows={true}
          rowHeight={40}
        />
      </div>

      {editingRate && (
        <EditModal
          stateRate={editingRate}
          onSave={handleUpdate}
          onCancel={() => setEditingRate(null)}
        />
      )}
    </div>
  );
};

interface EditModalProps {
  stateRate: StateRate;
  onSave: (stateRate: StateRate) => void;
  onCancel: () => void;
}

const EditModal: React.FC<EditModalProps> = ({ stateRate, onSave, onCancel }) => {
  const [formData, setFormData] = useState<StateRate>(stateRate);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave({
      ...formData,
      last_update: new Date().toISOString().split('T')[0]
    });
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-md w-full">
        <h3 className="text-lg font-semibold mb-4">Edit State Rate: {formData.state}</h3>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Rate (%)
            </label>
            <input
              type="number"
              value={formData.rate}
              onChange={(e) => setFormData(prev => ({ ...prev, rate: parseFloat(e.target.value) || 0 }))}
              step="0.01"
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={formData.is_variable}
                onChange={(e) => setFormData(prev => ({ ...prev, is_variable: e.target.checked }))}
                className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm font-medium text-gray-700">Variable Rate</span>
            </label>
          </div>

          {formData.is_variable && (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Plus Percentage (%)
              </label>
              <input
                type="number"
                value={formData.plus_percentage}
                onChange={(e) => setFormData(prev => ({ ...prev, plus_percentage: parseFloat(e.target.value) || 0 }))}
                step="0.01"
                min="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Update Frequency
            </label>
            <input
              type="text"
              value={formData.update_frequency}
              onChange={(e) => setFormData(prev => ({ ...prev, update_frequency: e.target.value }))}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Notes
            </label>
            <textarea
              value={formData.notes}
              onChange={(e) => setFormData(prev => ({ ...prev, notes: e.target.value }))}
              rows={3}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex justify-end space-x-2 pt-4">
            <button
              type="button"
              onClick={onCancel}
              className="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              Save Changes
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default DbManagerTab;
