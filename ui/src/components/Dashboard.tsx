import React, { useState } from 'react';
import type { DirectLinkStatus, PeerDevice, DeviceInfo } from '../api';
import { Laptop, Send, Shield, Zap, UploadCloud, Check, X, File, HardDrive } from 'lucide-react';

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
    <div className="flex flex-col h-full p-8 max-w-5xl mx-auto space-y-6 text-[#EDEDED] font-sans antialiased">
      {/* Top Telemetry Header */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
        {/* Local Device Identity */}
        <div className="bg-[#141415] border border-white/8 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-[#19191B] border border-white/6 rounded-lg text-[#EDEDED]">
              <Laptop className="w-4 h-4 text-blue-400" />
            </div>
            <div>
              <div className="text-[11px] text-[#85858B] uppercase tracking-wider font-mono">This Host</div>
              <div className="font-medium text-sm text-[#EDEDED]">{deviceInfo?.name || 'Local Laptop'}</div>
            </div>
          </div>
          <span className="text-[11px] px-2 py-0.5 rounded bg-[#19191B] border border-white/6 text-[#A1A1A6] font-mono">
            {deviceInfo?.os || 'OS'}
          </span>
        </div>

        {/* Physical Direct Link */}
        <div className="bg-[#141415] border border-white/8 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className={`p-2 rounded-lg border ${
              linkStatus?.connected 
                ? 'bg-[#4CC38A]/10 text-[#4CC38A] border-[#4CC38A]/20' 
                : 'bg-amber-500/10 text-amber-400 border-amber-500/20'
            }`}>
              <Zap className="w-4 h-4" />
            </div>
            <div>
              <div className="text-[11px] text-[#85858B] uppercase tracking-wider font-mono">Direct Cable Link</div>
              <div className="font-medium text-sm text-[#EDEDED] truncate max-w-[140px]">
                {linkStatus?.connected ? linkStatus.adapter_name : 'No Link'}
              </div>
            </div>
          </div>
          <div className="text-right">
            <div className={`text-[11px] font-mono font-medium ${linkStatus?.connected ? 'text-[#4CC38A]' : 'text-amber-400'}`}>
              {linkStatus?.connected ? 'CONNECTED' : 'STANDBY'}
            </div>
            <div className="text-[11px] font-mono text-[#85858B]">
              {linkStatus?.link_speed_gbps || 'Unknown'}
            </div>
          </div>
        </div>

        {/* Noise IK Channel State */}
        <div className="bg-[#141415] border border-white/8 rounded-xl p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-[#19191B] border border-white/6 rounded-lg text-[#EDEDED]">
              <Shield className="w-4 h-4 text-emerald-400" />
            </div>
            <div>
              <div className="text-[11px] text-[#85858B] uppercase tracking-wider font-mono">Channel Security</div>
              <div className="font-medium text-sm text-[#EDEDED]">Noise_IK Mutual</div>
            </div>
          </div>
          <button
            onClick={onOpenSettings}
            className="text-xs px-3 py-1.5 bg-[#19191B] hover:bg-[#232325] text-[#EDEDED] border border-white/8 rounded-lg font-medium transition-colors"
          >
            Settings
          </button>
        </div>
      </div>

      {/* Main Staging & Selection Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 flex-1">
        {/* Peer Selection (Left Col) */}
        <div className="bg-[#141415] border border-white/8 rounded-xl p-5 flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-xs font-mono uppercase tracking-wider text-[#85858B]">Target Peer</h2>
              <span className="text-[11px] font-mono text-[#85858B]">{peers.length} discovered</span>
            </div>

            <div className="space-y-2">
              {peers.length === 0 ? (
                <div className="p-4 rounded-lg bg-[#0F0F10] border border-white/6 text-xs text-[#85858B] text-center">
                  Searching for peers over direct IPv6 Link-Local...
                </div>
              ) : (
                peers.map((peer) => (
                  <div
                    key={peer.id}
                    onClick={() => setSelectedPeerId(peer.id)}
                    className={`p-3 rounded-lg border cursor-pointer transition-colors ${
                      selectedPeerId === peer.id
                        ? 'border-blue-500/40 bg-blue-500/10'
                        : 'border-white/6 bg-[#0F0F10] hover:border-white/12'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium text-sm text-[#EDEDED]">{peer.name}</span>
                      {peer.paired && <Check className="w-3.5 h-3.5 text-[#4CC38A]" />}
                    </div>
                    <div className="text-[11px] font-mono text-[#85858B] mt-1">
                      {peer.fingerprint.slice(0, 16)}...
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>

          <div className="pt-4 border-t border-white/6 space-y-2">
            <button
              onClick={onRunMockDemo}
              className="w-full py-2.5 bg-[#19191B] hover:bg-[#232325] text-xs font-mono text-[#A1A1A6] hover:text-[#EDEDED] rounded-lg transition-colors border border-white/6 flex items-center justify-center gap-2"
            >
              <HardDrive className="w-3.5 h-3.5 text-[#85858B]" />
              <span>Run mock benchmark (50 MB)</span>
            </button>
          </div>
        </div>

        {/* File Queue & Drag Drop Area (Right 2 Cols) */}
        <div className="md:col-span-2 bg-[#141415] border border-white/8 rounded-xl p-5 flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-xs font-mono uppercase tracking-wider text-[#85858B]">Staged payload</h2>
              <span className="text-[11px] font-mono text-[#85858B]">{simulatedFiles.length} items ready</span>
            </div>

            {/* Recessed Desktop Drop Tray */}
            <div className="border border-white/8 rounded-xl p-6 flex flex-col items-center justify-center text-center cursor-pointer transition-colors bg-[#0F0F10] hover:border-white/16">
              <UploadCloud className="w-8 h-8 text-[#85858B] mb-2" />
              <div className="text-sm font-medium text-[#EDEDED]">Drop files or directory trees here</div>
              <p className="text-xs text-[#85858B] mt-0.5 font-mono">Direct unbuffered stream over link</p>
            </div>

            {/* File List */}
            <div className="mt-4 space-y-2 max-h-48 overflow-y-auto">
              {simulatedFiles.map((file, idx) => (
                <div key={idx} className="flex items-center justify-between p-2.5 rounded-lg bg-[#0F0F10] border border-white/6 text-xs">
                  <div className="flex items-center gap-2 truncate">
                    <File className="w-3.5 h-3.5 text-[#85858B] shrink-0" />
                    <span className="font-mono text-[#EDEDED] truncate">{file}</span>
                  </div>
                  <button
                    onClick={() => setSimulatedFiles(simulatedFiles.filter((_, i) => i !== idx))}
                    className="text-[#85858B] hover:text-[#E5484D] p-1 transition-colors"
                  >
                    <X className="w-3.5 h-3.5" />
                  </button>
                </div>
              ))}
            </div>
          </div>

          <div className="pt-4 border-t border-white/6 flex justify-end">
            <button
              onClick={() => onStartTransfer(selectedPeerId, simulatedFiles)}
              disabled={!selectedPeerId || simulatedFiles.length === 0}
              className="px-5 py-2.5 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 disabled:cursor-not-allowed font-medium text-xs rounded-lg flex items-center gap-2 transition-colors text-white shadow-sm"
            >
              <Send className="w-3.5 h-3.5" />
              <span>Initiate direct transfer</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
