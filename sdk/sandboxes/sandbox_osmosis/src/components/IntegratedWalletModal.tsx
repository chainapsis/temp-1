"use client";

import { useState } from "react";

import { useWalletSelect } from "@keplr-ewallet-sandbox-osmosis/contexts/WalletSelectProvider";
import { useAccountStore } from "@keplr-ewallet-sandbox-osmosis/stores/account";

export type LoginMethod = "google" | "email" | "keplr";

interface IntegratedWalletModalProps {
  isOpen: boolean;
  onClose: () => void;
  onEwalletConnect: (method: LoginMethod) => void;
  isConnecting: boolean;
  error?: string | null;
  onErrorDismiss?: () => void;
  successMessage?: string | null;
}

// eWallet options
const ewalletOptions = [
  {
    id: "google" as LoginMethod,
    name: "Google",
    icon: "🔍",
    description: "Connect with Google Account",
    isRecommended: true,
    isEnabled: true,
    type: "ewallet" as const,
  },
  {
    id: "email" as LoginMethod,
    name: "Email",
    icon: "✉️",
    description: "Coming soon",
    isEnabled: false,
    type: "ewallet" as const,
  },
];

// Cosmos-kit wallet options
const cosmosKitOptions = [
  {
    id: "keplr-extension",
    name: "Keplr",
    icon: "🔓",
    description: "Connect with Keplr Extension",
    isEnabled: true,
    type: "cosmos-kit" as const,
  },
];

export function IntegratedWalletModal({
  isOpen,
  onClose,
  onEwalletConnect,
  isConnecting,
  error,
  onErrorDismiss,
  successMessage,
}: IntegratedWalletModalProps) {
  const [selectedWallet, setSelectedWallet] = useState<string | null>(null);
  const [connectingType, setConnectingType] = useState<
    "ewallet" | "cosmos-kit" | null
  >(null);

  const { accountStore } = useWalletSelect();
  const connectKeplr = () => useAccountStore.getState().connect("keplr");
  const isWalletConnected = accountStore.osmosisWallet.isWalletConnected;

  if (!isOpen) return null;

  const handleEwalletSelect = (walletId: LoginMethod) => {
    setSelectedWallet(walletId);
    setConnectingType("ewallet");
    onEwalletConnect(walletId);
  };

  const handleCosmosKitSelect = async (walletId: string) => {
    setSelectedWallet(walletId);
    setConnectingType("cosmos-kit");

    try {
      await connectKeplr();
      onClose();
    } catch (error) {
      console.error("Failed to connect with cosmos-kit:", error);
      setConnectingType(null);
      setSelectedWallet(null);
    }
  };

  const allWalletOptions = [...ewalletOptions, ...cosmosKitOptions];

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

        {/* Error Message */}
        {error && (
          <div className="mb-4 p-4 bg-red-900/30 border border-red-500/50 rounded-lg">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <svg
                  className="w-5 h-5 text-red-400 mr-2"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                    clipRule="evenodd"
                  />
                </svg>
                <p className="text-red-300 text-sm font-medium">{error}</p>
              </div>
              {onErrorDismiss && (
                <button
                  onClick={onErrorDismiss}
                  className="text-red-400 hover:text-red-300 transition-colors"
                >
                  <svg
                    className="w-4 h-4"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                  >
                    <path
                      fillRule="evenodd"
                      d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                      clipRule="evenodd"
                    />
                  </svg>
                </button>
              )}
            </div>
          </div>
        )}

        {/* Success Message */}
        {successMessage && (
          <div className="mb-4 p-4 bg-green-900/30 border border-green-500/50 rounded-lg">
            <div className="flex items-center">
              <svg
                className="w-5 h-5 text-green-400 mr-2"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              <p className="text-green-300 text-sm font-medium">
                {successMessage}
              </p>
            </div>
          </div>
        )}

        {/* Description */}
        <p className="text-gray-300 mb-6 text-sm">
          Connect with eWallet for social login or use your existing Cosmos
          wallet.
        </p>

        {/* eWallet Section */}
        <div className="mb-6">
          <h3 className="text-lg font-semibold text-white mb-3">
            🔐 eWallet (Social Login)
          </h3>
          <div className="space-y-3">
            {ewalletOptions.map((wallet) => (
              <button
                key={wallet.id}
                onClick={() =>
                  wallet.isEnabled && handleEwalletSelect(wallet.id)
                }
                disabled={isConnecting || !wallet.isEnabled}
                className={`w-full flex items-center justify-between p-4 rounded-lg border transition-all ${selectedWallet === wallet.id &&
                    connectingType === "ewallet" &&
                    isConnecting
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

                {selectedWallet === wallet.id &&
                  connectingType === "ewallet" &&
                  isConnecting && (
                    <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-purple-500"></div>
                  )}
              </button>
            ))}
          </div>
        </div>

        {/* Divider */}
        <div className="flex items-center my-6">
          <div className="flex-1 border-t border-gray-700"></div>
          <span className="px-3 text-gray-500 text-sm">or</span>
          <div className="flex-1 border-t border-gray-700"></div>
        </div>

        {/* Cosmos Kit Section */}
        <div>
          <h3 className="text-lg font-semibold text-white mb-3">
            🌌 Cosmos Wallets
          </h3>
          <div className="space-y-3">
            {cosmosKitOptions.map((wallet) => (
              <button
                key={wallet.id}
                onClick={() =>
                  wallet.isEnabled && handleCosmosKitSelect(wallet.id)
                }
                disabled={isConnecting || !wallet.isEnabled}
                className={`w-full flex items-center justify-between p-4 rounded-lg border transition-all ${selectedWallet === wallet.id &&
                    connectingType === "cosmos-kit" &&
                    isConnecting
                    ? "border-blue-500 bg-blue-500/10"
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
                    </div>
                    <p
                      className={`text-sm ${wallet.isEnabled ? "text-gray-400" : "text-gray-600"}`}
                    >
                      {wallet.description}
                    </p>
                  </div>
                </div>

                {selectedWallet === wallet.id &&
                  connectingType === "cosmos-kit" &&
                  isConnecting && (
                    <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-500"></div>
                  )}
              </button>
            ))}
          </div>
        </div>

        {/* Footer */}
        <div className="mt-6 pt-4 border-t border-gray-700">
          <p className="text-xs text-gray-400 text-center">
            New to wallets?{" "}
            <a
              href="https://docs.osmosis.zone/overview/getting-started"
              className="text-purple-400 hover:underline"
            >
              Learn more about wallets
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}
