import React, { useState } from 'react';
import type { PeerDevice } from '../api';
import { Usb, ShieldCheck, Laptop, Check, AlertTriangle, ArrowRight } from 'lucide-react';

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
  const [sasDigits, setSasDigits] = useState<string[]>(['', '', '', '', '', '']);
  const [error, setError] = useState<string | null>(null);

  const handleDigitChange = (index: number, val: string) => {
    const clean = val.replace(/\D/g, '').slice(-1);
    const next = [...sasDigits];
    next[index] = clean;
    setSasDigits(next);

    // Auto-advance
    if (clean && index < 5) {
      const nextInput = document.getElementById(`sas-digit-${index + 1}`);
      nextInput?.focus();
    }
  };

  const handleSasSubmit = async () => {
    if (!selectedPeer) return;
    const code = sasDigits.join('');
    if (code.length !== 6) return;

    try {
      setError(null);
      await onConfirmSas(selectedPeer.fingerprint, code);
      setStep(3);
    } catch {
      setError('Confirmation code mismatch. Verify both screens display matching 6 digits.');
    }
  };

  return (
    <div className="flex flex-col items-center justify-center p-8 max-w-xl mx-auto h-full text-[#EDEDED] font-sans antialiased">
      {/* Quiet Segmented Stepper */}
      <div className="flex items-center gap-2 mb-8 text-xs font-mono">
        <span className={step >= 1 ? 'text-[#EDEDED]' : 'text-[#85858B]'}>01 Physical Cable</span>
        <span className="text-white/10">/</span>
        <span className={step >= 2 ? 'text-[#EDEDED]' : 'text-[#85858B]'}>02 SAS Pairing</span>
        <span className="text-white/10">/</span>
        <span className={step === 3 ? 'text-[#4CC38A]' : 'text-[#85858B]'}>03 Verified</span>
      </div>

      {step === 1 && (
        <div className="bg-[#141415] border border-white/8 rounded-2xl p-7 w-full shadow-xl space-y-6 text-center">
          <div className="w-12 h-12 mx-auto rounded-xl bg-[#19191B] border border-white/6 flex items-center justify-center text-blue-400">
            <Usb className="w-5 h-5" />
          </div>
          <div className="space-y-1">
            <h2 className="text-base font-medium text-[#EDEDED]">Connect USB-C Physical Cable</h2>
            <p className="text-xs text-[#85858B] max-w-sm mx-auto leading-relaxed">
              Connect a high-speed USB-C, USB4, or Thunderbolt cable directly between both laptops. ByteBridge streams exclusively over hardware point-to-point links.
            </p>
          </div>

          <div className="bg-[#19191B] border border-white/6 text-amber-300 text-xs rounded-xl p-3.5 flex items-start gap-2.5 text-left">
            <AlertTriangle className="w-4 h-4 mt-0.5 shrink-0 text-amber-400" />
            <span className="text-[#A1A1A6] leading-relaxed">
              Standard phone charging cables lack high-speed data lines. Ensure you use a verified 10Gbps+ USB-C, USB4, or Thunderbolt cable.
            </span>
          </div>

          <button
            onClick={() => setStep(2)}
            className="w-full py-2.5 bg-blue-600 hover:bg-blue-500 font-medium text-xs text-white rounded-lg transition-colors flex items-center justify-center gap-2"
          >
            <span>Cable connected, proceed to pairing</span>
            <ArrowRight className="w-3.5 h-3.5" />
          </button>
        </div>
      )}

      {step === 2 && (
        <div className="bg-[#141415] border border-white/8 rounded-2xl p-7 w-full shadow-xl space-y-5">
          <div className="flex items-center gap-3 pb-3 border-b border-white/6">
            <div className="p-2 bg-[#19191B] border border-white/6 rounded-lg text-blue-400">
              <ShieldCheck className="w-4 h-4" />
            </div>
            <div>
              <h2 className="text-sm font-medium text-[#EDEDED]">Mutual SAS Peer Verification</h2>
              <p className="text-xs text-[#85858B]">Confirm 6-digit Short Authentication String to prevent MITM</p>
            </div>
          </div>

          <div className="space-y-2">
            <div className="text-xs text-[#85858B] font-mono">Discovered link peers:</div>
            {discoveredPeers.length === 0 ? (
              <div className="p-4 border border-white/6 rounded-xl bg-[#0F0F10] text-center text-[#85858B] text-xs">
                Searching for peers over direct IPv6 Link-Local...
              </div>
            ) : (
              discoveredPeers.map((peer) => (
                <div
                  key={peer.id}
                  onClick={() => setSelectedPeer(peer)}
                  className={`p-3 rounded-lg border cursor-pointer flex items-center justify-between transition-colors ${
                    selectedPeer?.id === peer.id
                      ? 'border-blue-500/40 bg-blue-500/10'
                      : 'border-white/6 bg-[#0F0F10] hover:border-white/12'
                  }`}
                >
                  <div className="flex items-center gap-2.5">
                    <Laptop className="w-4 h-4 text-blue-400" />
                    <div>
                      <div className="font-medium text-xs text-[#EDEDED]">{peer.name}</div>
                      <div className="text-[11px] font-mono text-[#85858B]">{peer.fingerprint.slice(0, 16)}...</div>
                    </div>
                  </div>
                  {peer.sas_code && (
                    <div className="px-2.5 py-1 bg-[#19191B] border border-white/8 text-blue-400 font-mono font-medium text-xs rounded">
                      SAS: {peer.sas_code}
                    </div>
                  )}
                </div>
              ))
            )}
          </div>

          {selectedPeer && (
            <div className="space-y-3 pt-3 border-t border-white/6">
              <label className="block text-xs text-[#85858B] text-center">
                Enter matching 6-digit SAS shown on peer screen:
              </label>

              {/* Segmented 6-Digit Monospace Input */}
              <div className="flex justify-center gap-2">
                {sasDigits.map((digit, idx) => (
                  <input
                    key={idx}
                    id={`sas-digit-${idx}`}
                    type="text"
                    inputMode="numeric"
                    maxLength={1}
                    value={digit}
                    onChange={(e) => handleDigitChange(idx, e.target.value)}
                    className="w-10 h-12 bg-[#0F0F10] border border-white/10 rounded-lg text-center font-mono text-lg font-semibold text-[#EDEDED] focus:outline-none focus:border-blue-500"
                  />
                ))}
              </div>

              {error && <p className="text-[#E5484D] text-xs text-center">{error}</p>}

              <button
                onClick={handleSasSubmit}
                disabled={sasDigits.join('').length !== 6}
                className="w-full py-2.5 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 font-medium text-xs text-white rounded-lg transition-colors mt-2"
              >
                Confirm pairing
              </button>
            </div>
          )}
        </div>
      )}

      {step === 3 && (
        <div className="bg-[#141415] border border-white/8 rounded-2xl p-7 w-full shadow-xl text-center space-y-5">
          <div className="w-12 h-12 mx-auto rounded-xl bg-[#4CC38A]/10 border border-[#4CC38A]/20 flex items-center justify-center text-[#4CC38A]">
            <Check className="w-5 h-5" />
          </div>
          <div className="space-y-1">
            <h2 className="text-base font-medium text-[#EDEDED]">Direct Link Authenticated</h2>
            <p className="text-xs text-[#85858B] max-w-sm mx-auto leading-relaxed">
              Encrypted session keys verified. Transfers can now execute securely at full hardware capability.
            </p>
          </div>
          <button
            onClick={onComplete}
            className="w-full py-2.5 bg-[#4CC38A] hover:bg-[#45B37D] font-medium text-xs text-[#0F0F10] font-semibold rounded-lg transition-colors"
          >
            Go to dashboard
          </button>
        </div>
      )}
    </div>
  );
};
