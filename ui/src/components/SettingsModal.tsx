import React, { useState } from 'react';
import type { AppSettings } from '../api';
import { X, Sliders, Shield, HardDrive, Cpu } from 'lucide-react';

interface SettingsModalProps {
  settings: AppSettings;
  onClose: () => void;
  onSave: (settings: AppSettings) => Promise<void>;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({
  settings,
  onClose,
  onSave,
}) => {
  const [current, setCurrent] = useState<AppSettings>(settings);

  const handleModeChange = (mode: 'minimal' | 'adaptive' | 'maximum') => {
    setCurrent({ ...current, performance_mode: mode });
  };

  return (
    <div className="fixed inset-0 bg-black/70 backdrop-blur-xs flex items-center justify-center p-4 z-50 text-slate-100">
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 w-full max-w-xl shadow-2xl space-y-6">
        <div className="flex items-center justify-between pb-4 border-b border-slate-800">
          <div className="flex items-center gap-2">
            <Sliders className="w-5 h-5 text-blue-400" />
            <h2 className="text-lg font-bold">ByteBridge Settings</h2>
          </div>
          <button onClick={onClose} className="text-slate-400 hover:text-slate-100">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Performance Mode Selector (§9 Specification) */}
        <div className="space-y-3">
          <label className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
            Transfer Performance Mode
          </label>
          <div className="grid grid-cols-3 gap-3">
            {/* Minimal */}
            <div
              onClick={() => handleModeChange('minimal')}
              className={`p-3 rounded-xl border cursor-pointer transition-all ${
                current.performance_mode === 'minimal'
                  ? 'border-blue-500 bg-blue-500/10'
                  : 'border-slate-800 bg-slate-950 hover:border-slate-700'
              }`}
            >
              <Cpu className="w-4 h-4 text-slate-400 mb-1" />
              <div className="font-bold text-xs">Minimal</div>
              <p className="text-[10px] text-slate-400 mt-0.5 leading-tight">Lowest CPU & thermals, ~50MB/s</p>
            </div>

            {/* Adaptive */}
            <div
              onClick={() => handleModeChange('adaptive')}
              className={`p-3 rounded-xl border cursor-pointer transition-all ${
                current.performance_mode === 'adaptive'
                  ? 'border-blue-500 bg-blue-500/10'
                  : 'border-slate-800 bg-slate-950 hover:border-slate-700'
              }`}
            >
              <Sliders className="w-4 h-4 text-blue-400 mb-1" />
              <div className="font-bold text-xs">Adaptive</div>
              <p className="text-[10px] text-slate-400 mt-0.5 leading-tight">Balanced 256KB-1MB buffer auto-scaling</p>
            </div>

            {/* Maximum */}
            <div
              onClick={() => handleModeChange('maximum')}
              className={`p-3 rounded-xl border cursor-pointer transition-all ${
                current.performance_mode === 'maximum'
                  ? 'border-blue-500 bg-blue-500/10'
                  : 'border-slate-800 bg-slate-950 hover:border-slate-700'
              }`}
            >
              <HardDrive className="w-4 h-4 text-purple-400 mb-1" />
              <div className="font-bold text-xs">Maximum</div>
              <p className="text-[10px] text-slate-400 mt-0.5 leading-tight">Direct unbuffered I/O + 4MB chunks</p>
            </div>
          </div>
        </div>

        {/* Storage Path */}
        <div className="space-y-2">
          <label className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
            Default Download Location
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              value={current.default_save_path}
              onChange={(e) => setCurrent({ ...current, default_save_path: e.target.value })}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-xs font-mono text-slate-200 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        {/* Security / Auto Accept */}
        <div className="p-3 bg-slate-950 rounded-xl border border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Shield className="w-4 h-4 text-emerald-400" />
            <div>
              <div className="text-xs font-semibold">Auto-accept paired peers</div>
              <div className="text-[10px] text-slate-400">Skip confirmation prompt for verified trust-store peers</div>
            </div>
          </div>
          <input
            type="checkbox"
            checked={current.auto_accept_paired}
            onChange={(e) => setCurrent({ ...current, auto_accept_paired: e.target.checked })}
            className="w-4 h-4 rounded text-blue-600 bg-slate-900 border-slate-700 focus:ring-0"
          />
        </div>

        <div className="flex justify-end gap-3 pt-4 border-t border-slate-800">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-slate-800 hover:bg-slate-700 rounded-lg text-xs font-semibold transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={async () => {
              await onSave(current);
              onClose();
            }}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-500 rounded-lg text-xs font-semibold transition-colors"
          >
            Save Changes
          </button>
        </div>
      </div>
    </div>
  );
};
