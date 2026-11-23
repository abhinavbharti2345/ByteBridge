import React, { useState } from 'react';
import type { TransferProgress } from '../api';
import { ArrowUpRight, HardDrive, Shield, Pause, Play } from 'lucide-react';

interface TransferScreenProps {
  progress: TransferProgress;
  onCancel: () => void;
  onBackToDashboard: () => void;
}

export const TransferScreen: React.FC<TransferScreenProps> = ({
  progress,
  onCancel,
  onBackToDashboard,
}) => {
  const [isPaused, setIsPaused] = useState(false);

  const percent =
    progress.total_bytes > 0
      ? Math.min(100, Math.round((progress.transferred_bytes / progress.total_bytes) * 100))
      : 0;

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const isCompleted = progress.status === 'completed';
  const isFailed = progress.status === 'failed' || progress.status === 'cancelled';

  // Format speed for maximum readability
  const speedDisplay =
    progress.speed_mbps >= 1000
      ? `${(progress.speed_mbps / 1024).toFixed(2)} GB/s`
      : `${progress.speed_mbps.toFixed(1)} MB/s`;

  return (
    <div className="flex flex-col items-center justify-center p-8 max-w-3xl mx-auto h-full text-[#EDEDED] font-sans antialiased">
      <div className="bg-[#141415] border border-white/8 rounded-2xl p-7 w-full shadow-xl space-y-6">
        
        {/* Top Header: Session Meta & Status */}
        <div className="flex items-center justify-between pb-5 border-b border-white/6">
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <span className="text-xs uppercase tracking-wider font-mono text-[#85858B]">Direct Link Session</span>
              <span className="text-[#85858B] text-xs">/</span>
              <span className="text-xs font-mono text-[#A1A1A6]">{progress.transfer_id}</span>
            </div>
            <h1 className="text-base font-medium text-[#EDEDED]">
              {isCompleted ? 'Transfer completed' : isFailed ? 'Transfer cancelled' : 'Transferring payload'}
            </h1>
          </div>

          <div className="flex items-center gap-2">
            <span
              className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-mono font-medium ${
                isCompleted
                  ? 'bg-[#4CC38A]/10 text-[#4CC38A] border border-[#4CC38A]/20'
                  : isFailed
                  ? 'bg-[#E5484D]/10 text-[#E5484D] border border-[#E5484D]/20'
                  : 'bg-blue-500/10 text-blue-400 border border-blue-500/20'
              }`}
            >
              <span
                className={`w-1.5 h-1.5 rounded-full ${
                  isCompleted ? 'bg-[#4CC38A]' : isFailed ? 'bg-[#E5484D]' : 'bg-blue-400 animate-pulse'
                }`}
              />
              {isCompleted ? 'Verified' : isFailed ? 'Stopped' : isPaused ? 'Paused' : 'Active'}
            </span>
          </div>
        </div>

        {/* Master File Progress Bar Area */}
        <div className="space-y-3 bg-[#19191B] border border-white/6 rounded-xl p-5">
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <div className="text-xs text-[#85858B]">Active file stream</div>
              <div className="text-sm font-medium text-[#EDEDED] font-mono truncate max-w-lg">
                {progress.active_file || 'Preparing payload...'}
              </div>
            </div>
            <div className="text-right">
              <div className="text-2xl font-mono font-semibold text-[#EDEDED] tabular-nums">
                {percent}%
              </div>
            </div>
          </div>

          {/* Precision Dual-Track Progress Bar */}
          <div className="relative w-full h-3 bg-[#0F0F10] rounded-full overflow-hidden border border-white/6">
            <div
              className={`h-full transition-all duration-300 ease-out rounded-full ${
                isCompleted
                  ? 'bg-[#4CC38A]'
                  : isFailed
                  ? 'bg-[#E5484D]'
                  : 'bg-blue-500'
              }`}
              style={{ width: `${percent}%` }}
            />
          </div>

          {/* Byte counts and file index metrics */}
          <div className="flex justify-between items-center text-xs text-[#A1A1A6] font-mono pt-1">
            <span>
              {formatBytes(progress.transferred_bytes)} of {formatBytes(progress.total_bytes)}
            </span>
            <span>
              {progress.total_files > 0 ? `Payload item 1 of ${progress.total_files}` : 'Verifying chunks'}
            </span>
          </div>
        </div>

        {/* Real-time Telemetry & Bottleneck Grid */}
        <div className="grid grid-cols-3 gap-3">
          {/* Speed Card */}
          <div className="bg-[#19191B] border border-white/6 rounded-xl p-4 flex flex-col justify-between">
            <div className="flex items-center justify-between text-xs text-[#85858B] mb-2">
              <span>Throughput</span>
              <ArrowUpRight className="w-3.5 h-3.5 text-[#A1A1A6]" />
            </div>
            <div className="text-xl font-mono font-semibold text-[#EDEDED] tabular-nums">
              {isCompleted ? '0 MB/s' : speedDisplay}
            </div>
            <div className="text-[11px] text-[#85858B] mt-1 font-mono">
              Unbuffered Direct I/O
            </div>
          </div>

          {/* Time Remaining Card */}
          <div className="bg-[#19191B] border border-white/6 rounded-xl p-4 flex flex-col justify-between">
            <div className="flex items-center justify-between text-xs text-[#85858B] mb-2">
              <span>Time Remaining</span>
              <span className="text-[10px] font-mono text-[#85858B]">ETA</span>
            </div>
            <div className="text-xl font-mono font-semibold text-[#EDEDED] tabular-nums">
              {isCompleted ? '0s' : `${progress.eta_seconds}s`}
            </div>
            <div className="text-[11px] text-[#85858B] mt-1 font-mono">
              Dynamic rate projection
            </div>
          </div>

          {/* Integrity Hashing Card */}
          <div className="bg-[#19191B] border border-white/6 rounded-xl p-4 flex flex-col justify-between">
            <div className="flex items-center justify-between text-xs text-[#85858B] mb-2">
              <span>Integrity</span>
              <Shield className="w-3.5 h-3.5 text-[#4CC38A]" />
            </div>
            <div className="text-sm font-mono font-semibold text-[#EDEDED] truncate">
              BLAKE3 + SHA-256
            </div>
            <div className="text-[11px] text-[#85858B] mt-1 font-mono">
              64 KiB chunk verification
            </div>
          </div>
        </div>

        {/* Real-time Bottleneck Diagnostic Bar (§7.2, §11) */}
        <div className="bg-[#19191B] border border-white/6 rounded-xl p-3.5 flex items-center justify-between text-xs">
          <div className="flex items-center gap-2.5">
            <HardDrive className="w-4 h-4 text-[#85858B]" />
            <span className="text-[#85858B]">Limiting factor:</span>
            <span className="text-[#EDEDED] font-medium">{progress.bottleneck}</span>
          </div>
          <span className="text-[11px] font-mono text-[#85858B]">Hysteresis filtered</span>
        </div>

        {/* Action Buttons */}
        <div className="flex items-center justify-end gap-3 pt-2">
          {!isCompleted && !isFailed ? (
            <>
              <button
                onClick={() => setIsPaused(!isPaused)}
                className="px-4 py-2.5 bg-[#232325] hover:bg-[#2A2A2D] text-[#EDEDED] text-xs font-medium rounded-lg border border-white/8 transition-colors flex items-center gap-2"
              >
                {isPaused ? <Play className="w-3.5 h-3.5" /> : <Pause className="w-3.5 h-3.5" />}
                <span>{isPaused ? 'Resume transfer' : 'Pause'}</span>
              </button>
              <button
                onClick={onCancel}
                className="px-4 py-2.5 bg-[#E5484D]/10 hover:bg-[#E5484D]/20 text-[#E5484D] text-xs font-medium rounded-lg border border-[#E5484D]/20 transition-colors"
              >
                Cancel transfer
              </button>
            </>
          ) : (
            <button
              onClick={onBackToDashboard}
              className="px-5 py-2.5 bg-blue-600 hover:bg-blue-500 text-white text-xs font-medium rounded-lg transition-colors shadow-sm"
            >
              Return to dashboard
            </button>
          )}
        </div>

      </div>
    </div>
  );
};
