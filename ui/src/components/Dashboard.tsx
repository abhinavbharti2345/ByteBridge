import React, { useState } from 'react';
import type { DirectLinkStatus, PeerDevice, DeviceInfo } from '../api';
import { Laptop, Send, Shield, Zap, UploadCloud, CheckCircle } from 'lucide-react';

interface DashboardProps {
  deviceInfo: DeviceInfo | null;
  linkStatus: DirectLinkStatus | null;
  peers: PeerDevice[];
  onStartTransfer: (peerId: string, files: string[]) => void;
  onOpenSettings: () => void;
  onRunMockDemo: () => void;
}

export const Dashboard: React.FC<DashboardProps> = ({
  deviceInfo,
  linkStatus,
  peers,
  onStartTransfer,
  onOpenSettings,
  onRunMockDemo,
}) => {
  const [selectedPeerId, setSelectedPeerId] = useState<string>(peers[0]?.id || '');
  const [simulatedFiles, setSimulatedFiles] = useState<string[]>([
    'dataset-large-2026.iso (4.2 GB)',
    'video-render-4k.mov (1.8 GB)',
  ]);

  return (
    <div className="flex flex-col h-full p-6 text-slate-100 max-w-5xl mx-auto space-y-6">
      {/* Top Bar: Device & Link Status */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* Local Device */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2.5 bg-blue-500/10 rounded-lg text-blue-400">
              <Laptop className="w-5 h-5" />
            </div>
            <div>
              <div className="text-xs text-slate-400">This Device</div>
              <div className="font-semibold text-sm">{deviceInfo?.name || 'Local Laptop'}</div>
            </div>
          </div>
          <span className="text-xs px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono uppercase">
            {deviceInfo?.os || 'OS'}
          </span>
        </div>

        {/* Physical Link Status */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className={`p-2.5 rounded-lg ${linkStatus?.connected ? 'bg-emerald-500/10 text-emerald-400' : 'bg-amber-500/10 text-amber-400'}`}>
              <Zap className="w-5 h-5" />
            </div>
            <div>
              <div className="text-xs text-slate-400">Direct Link</div>
              <div className="font-semibold text-sm">
                {linkStatus?.connected ? linkStatus.adapter_name : 'No Link Detected'}
              </div>
            </div>
          </div>
          <div className="text-right">
            <div className={`text-xs font-bold ${linkStatus?.connected ? 'text-emerald-400' : 'text-amber-400'}`}>
              {linkStatus?.connected ? 'CONNECTED' : 'STANDBY'}
            </div>
            <div className="text-xs font-mono text-slate-400">
              {linkStatus?.link_speed_gbps || 'Unknown / Not exposed by hardware or OS'}
            </div>
          </div>
        </div>

        {/* Security & Settings */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2.5 bg-indigo-500/10 rounded-lg text-indigo-400">
              <Shield className="w-5 h-5" />
            </div>
            <div>
              <div className="text-xs text-slate-400">Security State</div>
              <div className="font-semibold text-sm">Noise_IK Encrypted</div>
            </div>
          </div>
          <button
            onClick={onOpenSettings}
            className="text-xs px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg font-medium transition-colors"
          >
            Preferences
          </button>
        </div>
      </div>

      {/* Main Transfer Workspace */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 flex-1">
        {/* Peer Selection (Left Col) */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 flex flex-col justify-between">
          <div>
            <h3 className="font-semibold text-sm text-slate-300 mb-3 flex items-center gap-2">
              <Laptop className="w-4 h-4 text-blue-400" />
              Target Direct Peer
            </h3>
            <div className="space-y-2">
              {peers.length === 0 ? (
                <div className="p-4 rounded-lg bg-slate-950 border border-slate-800 text-xs text-slate-500 text-center">
                  No direct peer connected. Connect cable and verify SAS.
                </div>
              ) : (
                peers.map((peer) => (
                  <div
                    key={peer.id}
                    onClick={() => setSelectedPeerId(peer.id)}
                    className={`p-3 rounded-lg border cursor-pointer transition-all ${
                      selectedPeerId === peer.id
                        ? 'border-blue-500 bg-blue-500/10'
                        : 'border-slate-800 bg-slate-950 hover:border-slate-700'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium text-sm">{peer.name}</span>
                      {peer.paired && <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />}
                    </div>
                    <div className="text-xs font-mono text-slate-400 mt-1">
                      {peer.fingerprint.slice(0, 12)}...
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>

          <div className="pt-4 border-t border-slate-800 space-y-2">
            <button
              onClick={onRunMockDemo}
              className="w-full py-2 bg-slate-800 hover:bg-slate-700 text-blue-300 text-xs font-semibold rounded-lg transition-colors border border-blue-900/40"
            >
              Run In-Process Benchmark (50 MB)
            </button>
          </div>
        </div>

        {/* File Queue & Drag Drop (Right 2 Cols) */}
        <div className="md:col-span-2 bg-slate-900 border border-slate-800 rounded-xl p-5 flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold text-sm text-slate-300 flex items-center gap-2">
                <UploadCloud className="w-4 h-4 text-blue-400" />
                Files to Transfer
              </h3>
              <span className="text-xs text-slate-400">{simulatedFiles.length} files queued</span>
            </div>

            {/* Drop Zone Box */}
            <div className="border-2 border-dashed border-slate-700 hover:border-blue-500 rounded-xl p-8 flex flex-col items-center justify-center text-center cursor-pointer transition-colors bg-slate-950/50">
              <UploadCloud className="w-10 h-10 text-slate-500 mb-2" />
              <p className="text-sm font-medium text-slate-300">Drag and drop files or folders here</p>
              <p className="text-xs text-slate-500 mt-1">Direct unbuffered disk streaming over direct link</p>
            </div>

            {/* Selected File List */}
            <div className="mt-4 space-y-2">
              {simulatedFiles.map((file, idx) => (
                <div key={idx} className="flex items-center justify-between p-2.5 rounded-lg bg-slate-950 border border-slate-800 text-xs">
                  <span className="font-medium text-slate-300 truncate max-w-sm">{file}</span>
                  <button
                    onClick={() => setSimulatedFiles(simulatedFiles.filter((_, i) => i !== idx))}
                    className="text-slate-500 hover:text-red-400 font-bold ml-2"
                  >
                    ×
                  </button>
                </div>
              ))}
            </div>
          </div>

          <div className="pt-4 border-t border-slate-800 flex justify-end">
            <button
              onClick={() => onStartTransfer(selectedPeerId, simulatedFiles)}
              disabled={!selectedPeerId || simulatedFiles.length === 0}
              className="px-6 py-3 bg-blue-600 hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed font-semibold text-sm rounded-lg flex items-center gap-2 shadow-lg shadow-blue-900/30 transition-all"
            >
              <Send className="w-4 h-4" />
              <span>Initiate Direct Transfer</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
