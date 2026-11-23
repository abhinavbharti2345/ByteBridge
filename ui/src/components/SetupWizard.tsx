import React, { useState } from 'react';
import type { PeerDevice } from '../api';
import { Usb, ShieldCheck, Laptop, CheckCircle2, AlertTriangle, ArrowRight } from 'lucide-react';

interface SetupWizardProps {
  discoveredPeers: PeerDevice[];
  onConfirmSas: (fingerprint: string, code: string) => Promise<void>;
  onComplete: () => void;
}

export const SetupWizard: React.FC<SetupWizardProps> = ({
  discoveredPeers,
  onConfirmSas,
  onComplete,
}) => {
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [selectedPeer, setSelectedPeer] = useState<PeerDevice | null>(null);
  const [enteredCode, setEnteredCode] = useState('');
  const [error, setError] = useState<string | null>(null);

  const handleSasSubmit = async () => {
    if (!selectedPeer) return;
    try {
      setError(null);
      await onConfirmSas(selectedPeer.fingerprint, enteredCode);
      setStep(3);
    } catch {
      setError('SAS confirmation code mismatch. Verify both screens display the exact same 6 digits.');
    }
  };

  return (
    <div className="flex flex-col items-center justify-center p-8 max-w-2xl mx-auto h-full text-slate-100">
      <div className="flex items-center gap-4 mb-8">
        <div className={`w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm ${step >= 1 ? 'bg-blue-600 text-white' : 'bg-slate-800 text-slate-500'}`}>1</div>
        <div className={`h-1 w-12 ${step >= 2 ? 'bg-blue-600' : 'bg-slate-800'}`} />
        <div className={`w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm ${step >= 2 ? 'bg-blue-600 text-white' : 'bg-slate-800 text-slate-500'}`}>2</div>
        <div className={`h-1 w-12 ${step >= 3 ? 'bg-blue-600' : 'bg-slate-800'}`} />
        <div className={`w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm ${step === 3 ? 'bg-emerald-600 text-white' : 'bg-slate-800 text-slate-500'}`}>3</div>
      </div>

      {step === 1 && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-8 w-full shadow-2xl flex flex-col items-center text-center">
          <div className="p-4 bg-blue-500/10 rounded-2xl mb-4 text-blue-400">
            <Usb className="w-12 h-12" />
          </div>
          <h2 className="text-xl font-bold mb-2">Connect Your USB-C Cable</h2>
          <p className="text-slate-400 text-sm mb-6 max-w-md">
            Plug a high-speed USB-C, USB4, or Thunderbolt cable directly between both laptops. ByteBridge operates exclusively over direct physical links.
          </p>
          <div className="bg-amber-500/10 border border-amber-500/20 text-amber-300 text-xs rounded-lg p-3 flex items-start gap-2 mb-6 text-left">
            <AlertTriangle className="w-4 h-4 mt-0.5 shrink-0" />
            <span>Note: Standard phone charging cables do not support high-speed data. Ensure you use a 10Gbps+ USB-C or Thunderbolt cable.</span>
          </div>
          <button
            onClick={() => setStep(2)}
            className="w-full py-3 bg-blue-600 hover:bg-blue-500 font-semibold rounded-lg transition-colors flex items-center justify-center gap-2"
          >
            <span>Cable Connected, Next</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      )}

      {step === 2 && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-8 w-full shadow-2xl">
          <div className="flex items-center gap-3 mb-4">
            <div className="p-3 bg-blue-500/10 rounded-xl text-blue-400">
              <ShieldCheck className="w-6 h-6" />
            </div>
            <div>
              <h2 className="text-xl font-bold">Mutual Peer Verification</h2>
              <p className="text-slate-400 text-xs">Verify the 6-digit Short Authentication String (SAS)</p>
            </div>
          </div>

          <p className="text-slate-300 text-sm mb-4">
            Select the discovered peer laptop on your direct link:
          </p>

          <div className="space-y-3 mb-6">
            {discoveredPeers.length === 0 ? (
              <div className="p-4 border border-slate-800 rounded-lg text-center text-slate-500 text-sm">
                Searching for peers over direct IPv6 Link-Local...
              </div>
            ) : (
              discoveredPeers.map((peer) => (
                <div
                  key={peer.id}
                  onClick={() => setSelectedPeer(peer)}
                  className={`p-4 rounded-lg border cursor-pointer flex items-center justify-between transition-all ${
                    selectedPeer?.id === peer.id
                      ? 'border-blue-500 bg-blue-500/10'
                      : 'border-slate-800 bg-slate-800/50 hover:border-slate-700'
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <Laptop className="w-5 h-5 text-blue-400" />
                    <div>
                      <div className="font-semibold text-sm">{peer.name}</div>
                      <div className="text-xs font-mono text-slate-400">{peer.fingerprint.slice(0, 16)}...</div>
                    </div>
                  </div>
                  {peer.sas_code && (
                    <div className="px-3 py-1 bg-blue-950 border border-blue-800 text-blue-300 font-mono font-bold text-sm rounded">
                      SAS: {peer.sas_code}
                    </div>
                  )}
                </div>
              ))
            )}
          </div>

          {selectedPeer && (
            <div className="space-y-4 pt-2 border-t border-slate-800">
              <div>
                <label className="block text-xs font-medium text-slate-400 mb-1">
                  Enter 6-digit code shown on peer screen:
                </label>
                <input
                  type="text"
                  maxLength={6}
                  value={enteredCode}
                  onChange={(e) => setEnteredCode(e.target.value.trim())}
                  placeholder="e.g. 849201"
                  className="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-center text-lg tracking-widest font-mono font-bold text-slate-100 focus:outline-none focus:border-blue-500"
                />
              </div>
              {error && <p className="text-red-400 text-xs">{error}</p>}
              <button
                onClick={handleSasSubmit}
                disabled={enteredCode.length !== 6}
                className="w-full py-3 bg-blue-600 hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed font-semibold rounded-lg transition-colors"
              >
                Confirm & Pair
              </button>
            </div>
          )}
        </div>
      )}

      {step === 3 && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-8 w-full shadow-2xl flex flex-col items-center text-center">
          <div className="p-4 bg-emerald-500/10 rounded-2xl mb-4 text-emerald-400">
            <CheckCircle2 className="w-12 h-12" />
          </div>
          <h2 className="text-xl font-bold mb-2">Direct Link Authenticated</h2>
          <p className="text-slate-400 text-sm mb-6 max-w-md">
            The encrypted session key has been verified against peer fingerprint. Files can now be transferred securely at full hardware capability.
          </p>
          <button
            onClick={onComplete}
            className="w-full py-3 bg-emerald-600 hover:bg-emerald-500 font-semibold rounded-lg transition-colors"
          >
            Go to Dashboard
          </button>
        </div>
      )}
    </div>
  );
};
