import { invoke } from '@tauri-apps/api/core';

export interface DeviceInfo {
  name: string;
  fingerprint: string;
  os: string;
}

export interface DirectLinkStatus {
  connected: boolean;
  adapter_name: string;
  link_speed_gbps: string;
  local_ip: string;
  peer_ip: string | null;
}

export interface PeerDevice {
  id: string;
  name: string;
  fingerprint: string;
  paired: boolean;
  sas_code: string | null;
}

export interface TransferProgress {
  transfer_id: string;
  total_files: number;
  total_bytes: number;
  transferred_bytes: number;
  speed_mbps: number;
  eta_seconds: number;
  status: 'idle' | 'transferring' | 'completed' | 'failed' | 'cancelled';
  active_file: string;
  bottleneck: string;
}

export interface AppSettings {
  performance_mode: 'minimal' | 'adaptive' | 'maximum';
  default_save_path: string;
  auto_accept_paired: boolean;
  port: number;
}

export const api = {
  getDirectLinkStatus: (): Promise<DirectLinkStatus> => invoke('get_direct_link_status'),
  getLocalDeviceInfo: (): Promise<DeviceInfo> => invoke('get_local_device_info'),
  discoverPeers: (): Promise<PeerDevice[]> => invoke('discover_peers'),
  confirmSasCode: (peerFingerprint: string, code: string): Promise<boolean> =>
    invoke('confirm_sas_code', { peerFingerprint, code }),
  startTransfer: (targetPeerId: string, filePaths: string[]): Promise<string> =>
    invoke('start_transfer', { targetPeerId, filePaths }),
  getTransferProgress: (transferId: string): Promise<TransferProgress> =>
    invoke('get_transfer_progress', { transferId }),
  cancelTransfer: (transferId: string): Promise<boolean> =>
    invoke('cancel_transfer', { transferId }),
  getSettings: (): Promise<AppSettings> => invoke('get_settings'),
  saveSettings: (settings: AppSettings): Promise<boolean> =>
    invoke('save_settings', { settings }),
  runMockTransfer: (sizeMb: number): Promise<string> =>
    invoke('run_mock_transfer', { sizeMb }),
};
