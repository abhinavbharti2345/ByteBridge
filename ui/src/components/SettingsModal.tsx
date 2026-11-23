import React, { useState } from 'react';
import type { AppSettings } from '../api';
import { X, Sliders, Shield, HardDrive, Cpu, Folder } from 'lucide-react';

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
    <div className="fixed inset-0 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 z-50 text-[#EDEDED] font-sans antialiased">
      <div className="bg-[#141415] border border-white/8 rounded-2xl p-6 w-full max-w-xl shadow-2xl space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between pb-3 border-b border-white/6">
          <div className="flex items-center gap-2">
            <Sliders className="w-4 h-4 text-blue-400" />
            <h2 className="text-sm font-medium text-[#EDEDED]">Preferences</h2>
          </div>
          <button onClick={onClose} className="text-[#85858B] hover:text-[#EDEDED] transition-colors">
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Performance Mode Selector (§7.1, §9) */}
        <div className="space-y-2">
          <label className="text-xs font-mono uppercase tracking-wider text-[#85858B]">
            Transfer performance mode
          </label>
          <div className="grid grid-cols-3 gap-3">
            {/* Minimal */}
            <div
              onClick={() => handleModeChange('minimal')}
              className={`p-3.5 rounded-xl border cursor-pointer transition-colors ${
                current.performance_mode === 'minimal'
                  ? 'border-blue-500/40 bg-blue-500/10'
                  : 'border-white/6 bg-[#0F0F10] hover:border-white/10'
              }`}
            >
              <Cpu className="w-4 h-4 text-[#85858B] mb-2" />
              <div className="font-medium text-xs text-[#EDEDED]">Minimal</div>
              <p className="text-[11px] text-[#85858B] mt-1 leading-tight">Low thermals & background mode</p>
            </div>

            {/* Adaptive */}
            <div
              onClick={() => handleModeChange('adaptive')}
              className={`p-3.5 rounded-xl border cursor-pointer transition-colors ${
                current.performance_mode === 'adaptive'
                  ? 'border-blue-500/40 bg-blue-500/10'
                  : 'border-white/6 bg-[#0F0F10] hover:border-white/10'
              }`}
            >
              <Sliders className="w-4 h-4 text-blue-400 mb-2" />
              <div className="font-medium text-xs text-[#EDEDED]">Adaptive</div>
              <p className="text-[11px] text-[#85858B] mt-1 leading-tight">Balanced backpressure auto-tuning</p>
            </div>

            {/* Maximum */}
            <div
              onClick={() => handleModeChange('maximum')}
              className={`p-3.5 rounded-xl border cursor-pointer transition-colors ${
                current.performance_mode === 'maximum'
                  ? 'border-blue-500/40 bg-blue-500/10'
                  : 'border-white/6 bg-[#0F0F10] hover:border-white/10'
              }`}
            >
              <HardDrive className="w-4 h-4 text-emerald-400 mb-2" />
              <div className="font-medium text-xs text-[#EDEDED]">Maximum</div>
              <p className="text-[11px] text-[#85858B] mt-1 leading-tight">Direct unbuffered I/O + 8MB chunks</p>
            </div>
          </div>
        </div>

        {/* Storage Path */}
        <div className="space-y-1.5">
          <label className="text-xs font-mono uppercase tracking-wider text-[#85858B]">
            Default download destination
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              value={current.default_save_path}
              onChange={(e) => setCurrent({ ...current, default_save_path: e.target.value })}
              className="w-full bg-[#0F0F10] border border-white/8 rounded-lg px-3 py-2 text-xs font-mono text-[#EDEDED] focus:outline-none focus:border-blue-500"
            />
            <button className="px-3 bg-[#19191B] hover:bg-[#232325] border border-white/8 rounded-lg text-[#85858B] hover:text-[#EDEDED] transition-colors">
              <Folder className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Security / Auto-Accept Toggle */}
        <div className="p-3 bg-[#0F0F10] rounded-xl border border-white/6 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Shield className="w-4 h-4 text-[#4CC38A]" />
            <div>
              <div className="text-xs font-medium text-[#EDEDED]">Auto-accept paired peers</div>
              <div className="text-[11px] text-[#85858B]">Accept transfers automatically from verified trust-store devices</div>
            </div>
          </div>
          <button
            onClick={() => setCurrent({ ...current, auto_accept_paired: !current.auto_accept_paired })}
            className={`w-9 h-5 rounded-full p-0.5 transition-colors ${
              current.auto_accept_paired ? 'bg-blue-600' : 'bg-[#232325]'
            }`}
          >
            <div
              className={`w-4 h-4 rounded-full bg-white transition-transform ${
                current.auto_accept_paired ? 'translate-x-4' : 'translate-x-0'
              }`}
            />
          </button>
        </div>

        <div className="flex justify-end gap-2.5 pt-3 border-t border-white/6">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-[#19191B] hover:bg-[#232325] rounded-lg text-xs font-medium text-[#A1A1A6] hover:text-[#EDEDED] transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={async () => {
              await onSave(current);
              onClose();
            }}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-500 rounded-lg text-xs font-medium text-white transition-colors"
          >
            Save changes
          </button>
        </div>
      </div>
    </div>
  );
};
