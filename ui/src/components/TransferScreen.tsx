import React from 'react';
import type { TransferProgress } from '../api';
import { ArrowUpRight, Activity, XCircle, CheckCircle2, Clock } from 'lucide-react';

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
  const percent = progress.total_bytes > 0
    ? Math.min(100, Math.round((progress.transferred_bytes / progress.total_bytes) * 100))
    : 0;

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const isFinished = progress.status === 'completed';
  const isFailed = progress.status === 'failed' || progress.status === 'cancelled';

  return (
    <div className="flex flex-col items-center justify-center p-8 max-w-2xl mx-auto h-full text-slate-100">
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-8 w-full shadow-2xl">
        {/* Status Header */}
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className={`p-3 rounded-xl ${isFinished ? 'bg-emerald-500/10 text-emerald-400' : isFailed ? 'bg-red-500/10 text-red-400' : 'bg-blue-500/10 text-blue-400'}`}>
              {isFinished ? <CheckCircle2 className="w-6 h-6" /> : isFailed ? <XCircle className="w-6 h-6" /> : <Activity className="w-6 h-6 animate-pulse" />}
            </div>
            <div>
              <h2 className="text-lg font-bold">
                {isFinished ? 'Transfer Completed' : isFailed ? 'Transfer Interrupted' : 'Direct Link Transferring'}
              </h2>
              <div className="text-xs text-slate-400 font-mono">ID: {progress.transfer_id}</div>
            </div>
          </div>
          <span className={`text-xs px-3 py-1 rounded-full font-semibold uppercase ${
            isFinished ? 'bg-emerald-950 text-emerald-400 border border-emerald-800' :
            isFailed ? 'bg-red-950 text-red-400 border border-red-800' :
            'bg-blue-950 text-blue-400 border border-blue-800'
          }`}>
            {progress.status}
          </span>
        </div>

        {/* Current Active File */}
        <div className="mb-6 p-3 bg-slate-950 rounded-lg border border-slate-800/80">
          <div className="text-xs text-slate-500 mb-1">Active File Stream:</div>
          <div className="text-sm font-medium text-slate-200 truncate">{progress.active_file || 'Preparing payload...'}</div>
        </div>

        {/* Progress Bar */}
        <div className="mb-6 space-y-2">
          <div className="flex justify-between text-xs font-semibold">
            <span className="text-slate-400">{formatBytes(progress.transferred_bytes)} / {formatBytes(progress.total_bytes)}</span>
            <span className="text-blue-400 font-mono text-sm">{percent}%</span>
          </div>
          <div className="w-full bg-slate-950 h-3 rounded-full overflow-hidden border border-slate-800">
            <div
              className={`h-full transition-all duration-300 rounded-full ${
                isFinished ? 'bg-emerald-500' : isFailed ? 'bg-red-500' : 'bg-gradient-to-r from-blue-600 to-indigo-500'
              }`}
              style={{ width: `${percent}%` }}
            />
          </div>
        </div>

        {/* Real-time Metrics Grid */}
        <div className="grid grid-cols-2 gap-4 mb-6">
          <div className="p-4 bg-slate-950 rounded-xl border border-slate-800 flex items-center gap-3">
            <div className="p-2 bg-blue-500/10 text-blue-400 rounded-lg">
              <ArrowUpRight className="w-5 h-5" />
            </div>
            <div>
              <div className="text-xs text-slate-400">Current Speed</div>
              <div className="text-base font-bold font-mono text-slate-100">
                {progress.speed_mbps.toFixed(1)} MB/s
              </div>
            </div>
          </div>

          <div className="p-4 bg-slate-950 rounded-xl border border-slate-800 flex items-center gap-3">
            <div className="p-2 bg-purple-500/10 text-purple-400 rounded-lg">
              <Clock className="w-5 h-5" />
            </div>
            <div>
              <div className="text-xs text-slate-400">Est. Time Remaining</div>
              <div className="text-base font-bold font-mono text-slate-100">
                {progress.eta_seconds > 0 ? `${progress.eta_seconds}s` : '0s'}
              </div>
            </div>
          </div>
        </div>

        {/* Bottleneck Indicator Line */}
        <div className="text-xs text-slate-400 bg-slate-950/70 border border-slate-800/80 rounded-lg p-2.5 mb-6 flex items-center justify-between">
          <span className="text-slate-500">Limiting Factor:</span>
          <span className="font-semibold text-slate-300">{progress.bottleneck}</span>
        </div>

        {/* Actions */}
        <div className="flex gap-3">
          {!isFinished && !isFailed ? (
            <button
              onClick={onCancel}
              className="w-full py-3 bg-red-950 hover:bg-red-900 text-red-300 border border-red-800 font-semibold rounded-lg transition-colors"
            >
              Cancel Transfer
            </button>
          ) : (
            <button
              onClick={onBackToDashboard}
              className="w-full py-3 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg transition-colors"
            >
              Return to Dashboard
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
