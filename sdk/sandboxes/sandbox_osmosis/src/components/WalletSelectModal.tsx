"use client";

import { useState, useEffect } from "react";
import { WalletConfig, getSelectableWallets, getAvailableWallets, getInstallableWallets, getEwalletConfigs, getExtensionWallets } from "@keplr-ewallet-sandbox-osmosis/config/wallet-registry";
import { useAccountStore } from "@keplr-ewallet-sandbox-osmosis/stores/account";

export interface WalletSelectModalProps {
  isOpen: boolean;
  onClose: () => void;
  onWalletSelect: (wallet: WalletConfig) => void;
  onEwalletConnect: (method: "google") => void;
  isConnecting: boolean;
  error?: string | null;
  onErrorDismiss?: () => void;
}

export const WalletSelectModal: React.FC<WalletSelectModalProps> = ({
  isOpen,
  onClose,
  onWalletSelect,
  onEwalletConnect,
  isConnecting,
  error,
  onErrorDismiss,
}) => {
  const [activeTab, setActiveTab] = useState<"wallets" | "ewallet">("wallets");
  const [availableWallets, setAvailableWallets] = useState<WalletConfig[]>([]);
  const [installableWallets, setInstallableWallets] = useState<WalletConfig[]>([]);
  const [ewalletConfigs, setEwalletConfigs] = useState<WalletConfig[]>([]);

  useEffect(() => {
    if (isOpen) {
      const available = getAvailableWallets();
      const installable = getInstallableWallets();
      const ewallets = getEwalletConfigs();
      
      // Separate extension wallets from eWallets
      setAvailableWallets(available.filter(w => w.mode === "extension"));
      setInstallableWallets(installable.filter(w => w.mode === "extension"));
      setEwalletConfigs(ewallets);
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleWalletClick = async (wallet: WalletConfig) => {
    if (isConnecting) return;
    
    try {
      onWalletSelect(wallet);
    } catch (error) {
      console.error("Failed to select wallet:", error);
    }
  };

  const handleInstallWallet = (wallet: WalletConfig) => {
    if (wallet.downloads.desktop) {
      window.open(wallet.downloads.desktop, "_blank");
    }
  };

  const renderWalletItem = (wallet: WalletConfig, isInstallable: boolean = false) => (
    <div
      key={wallet.id}
      className={`flex items-center justify-between p-4 border border-white/10 rounded-lg hover:bg-white/5 transition-colors ${
        isConnecting ? "opacity-50 cursor-not-allowed" : "cursor-pointer"
      }`}
      onClick={() => isInstallable ? handleInstallWallet(wallet) : handleWalletClick(wallet)}
    >
      <div className="flex items-center space-x-3">
        <div className="w-10 h-10 bg-white/10 rounded-full flex items-center justify-center">
          <span className="text-xl">🔗</span>
        </div>
        <div>
          <div className="text-white font-medium">{wallet.prettyName}</div>
          <div className="text-sm text-gray-400">
            {isInstallable ? "Install Extension" : "Available"}
          </div>
        </div>
      </div>
      <div className="text-right">
        {wallet.features?.notifications && (
          <div className="text-xs text-green-400 mb-1">✓ Notifications</div>
        )}
        {wallet.features?.staking && (
          <div className="text-xs text-blue-400 mb-1">✓ Staking</div>
        )}
        {wallet.features?.governance && (
          <div className="text-xs text-purple-400">✓ Governance</div>
        )}
      </div>
    </div>
  );

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-gray-900 rounded-2xl border border-white/10 w-full max-w-md max-h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="p-6 border-b border-white/10">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-bold text-white">Connect Wallet</h2>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-white p-1"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          
          {/* Tabs */}
          <div className="flex space-x-1 mt-4 bg-white/5 rounded-lg p-1">
            <button
              onClick={() => setActiveTab("wallets")}
              className={`flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                activeTab === "wallets"
                  ? "bg-purple-500 text-white"
                  : "text-gray-400 hover:text-white"
              }`}
            >
              Keplr Wallet
            </button>
            <button
              onClick={() => setActiveTab("ewallet")}
              className={`flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                activeTab === "ewallet"
                  ? "bg-purple-500 text-white"
                  : "text-gray-400 hover:text-white"
              }`}
            >
              eWallet
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-6 max-h-96 overflow-y-auto">
          {error && (
            <div className="mb-4 p-3 bg-red-500/20 border border-red-500/50 rounded-lg">
              <div className="flex items-center justify-between">
                <div className="text-red-400 text-sm">{error}</div>
                {onErrorDismiss && (
                  <button
                    onClick={onErrorDismiss}
                    className="text-red-400 hover:text-red-300 text-sm"
                  >
                    ×
                  </button>
                )}
              </div>
            </div>
          )}

          {activeTab === "wallets" && (
            <div className="space-y-4">
              {availableWallets.length > 0 && (
                <div>
                  <h3 className="text-sm font-medium text-gray-400 mb-2">Available</h3>
                  <div className="space-y-2">
                    {availableWallets.map(wallet => renderWalletItem(wallet, false))}
                  </div>
                </div>
              )}
              
              {installableWallets.length > 0 && (
                <div>
                  <h3 className="text-sm font-medium text-gray-400 mb-2">Install Extension</h3>
                  <div className="space-y-2">
                    {installableWallets.map(wallet => renderWalletItem(wallet, true))}
                  </div>
                </div>
              )}
              
              {availableWallets.length === 0 && installableWallets.length === 0 && (
                <div className="text-center py-8">
                  <div className="text-gray-400 mb-2">Keplr not detected</div>
                  <div className="text-sm text-gray-500">
                    Please install Keplr extension or use eWallet
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === "ewallet" && (
            <div className="space-y-3">
              <div className="text-center mb-6">
                <div className="text-white font-medium mb-2">Social Login</div>
                <div className="text-sm text-gray-400">
                  Connect with your social account using eWallet
                </div>
              </div>
              
              {ewalletConfigs.map(wallet => (
                <div key={wallet.id} className="space-y-2">
                  {wallet.socialMethods?.map(method => (
                    <button
                      key={method}
                      onClick={() => onWalletSelect(wallet)}
                      disabled={isConnecting}
                      className={`w-full flex items-center justify-center space-x-3 p-4 border border-white/10 rounded-lg transition-colors ${
                        isConnecting
                          ? "opacity-50 cursor-not-allowed"
                          : "hover:bg-white/5 cursor-pointer"
                      }`}
                    >
                      <div className="w-8 h-8 bg-white rounded-full flex items-center justify-center">
                        <span className="text-lg">G</span>
                      </div>
                      <div className="text-white font-medium">
                        {isConnecting ? "Connecting..." : `Continue with ${method.charAt(0).toUpperCase() + method.slice(1)}`}
                      </div>
                    </button>
                  ))}
                </div>
              ))}

              <div className="mt-4 p-3 bg-blue-500/20 border border-blue-500/50 rounded-lg">
                <div className="text-blue-400 text-sm">
                  <div className="font-medium mb-1">✓ No Extension Required</div>
                  <div>Connect instantly with your social account</div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-white/10">
          <div className="text-xs text-gray-500 text-center">
            By connecting, you agree to our Terms of Service and Privacy Policy
          </div>
        </div>
      </div>
    </div>
  );
};