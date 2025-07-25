"use client";

import { useState } from "react";

interface WalletConnectModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConnect: (method: string) => void;
  isConnecting: boolean;
}

const walletOptions = [
  {
    id: "google",
    name: "Google",
    icon: "🔍",
    description: "Connect with Google Account",
    isRecommended: true,
    isEnabled: true,
  },
  {
    id: "email",
    name: "Email",
    icon: "✉️",
    description: "Coming soon",
    isEnabled: false,
  },
];

export function WalletConnectModal({
  isOpen,
  onClose,
  onConnect,
  isConnecting,
}: WalletConnectModalProps) {
  const [selectedWallet, setSelectedWallet] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleWalletSelect = (walletId: string) => {
    setSelectedWallet(walletId);
    onConnect(walletId);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="relative bg-gray-900 rounded-2xl border border-gray-700 p-6 max-w-md w-full mx-4 shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold text-white">Connect Wallet</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* Description */}
        <p className="text-gray-300 mb-6 text-sm">
          By connecting a wallet, you agree to our{" "}
          <a href="#" className="text-purple-400 hover:underline">
            Terms of Service
          </a>{" "}
          and{" "}
          <a href="#" className="text-purple-400 hover:underline">
            Privacy Policy
          </a>
          .
        </p>

        {/* Wallet Options */}
        <div className="space-y-3">
          {walletOptions.map((wallet) => (
            <button
              key={wallet.id}
              onClick={() => wallet.isEnabled && handleWalletSelect(wallet.id)}
              disabled={isConnecting || !wallet.isEnabled}
              className={`w-full flex items-center justify-between p-4 rounded-lg border transition-all ${selectedWallet === wallet.id && isConnecting
                  ? "border-purple-500 bg-purple-500/10"
                  : wallet.isEnabled
                    ? "border-gray-600 hover:border-gray-500 bg-gray-800/50 hover:bg-gray-800"
                    : "border-gray-700 bg-gray-800/30"
                } ${isConnecting || !wallet.isEnabled
                  ? "opacity-50 cursor-not-allowed"
                  : "cursor-pointer"
                }`}
            >
              <div className="flex items-center space-x-3">
                <div className="text-2xl">{wallet.icon}</div>
                <div className="text-left">
                  <div className="flex items-center space-x-2">
                    <span
                      className={`font-semibold ${wallet.isEnabled ? "text-white" : "text-gray-500"}`}
                    >
                      {wallet.name}
                    </span>
                    {wallet.isRecommended && wallet.isEnabled && (
                      <span className="px-2 py-1 text-xs bg-purple-500 text-white rounded-full">
                        Recommended
                      </span>
                    )}
                  </div>
                  <p
                    className={`text-sm ${wallet.isEnabled ? "text-gray-400" : "text-gray-600"}`}
                  >
                    {wallet.description}
                  </p>
                </div>
              </div>

              {selectedWallet === wallet.id && isConnecting && (
                <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-purple-500"></div>
              )}
            </button>
          ))}
        </div>

        {/* Footer */}
        <div className="mt-6 pt-4 border-t border-gray-700">
          <p className="text-xs text-gray-400 text-center">
            New to wallets?{" "}
            <a href="#" className="text-purple-400 hover:underline">
              Learn more about wallets
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}
