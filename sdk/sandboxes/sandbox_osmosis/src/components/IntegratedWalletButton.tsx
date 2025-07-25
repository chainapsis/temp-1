"use client";

import { useState, useRef, useEffect } from "react";
import { createPortal } from "react-dom";
import { AccountData } from "@cosmjs/amino";
import type { Key } from "@keplr-wallet/types";

import { useWalletSelect } from "@keplr-ewallet-sandbox-osmosis/contexts/WalletSelectProvider";
import { useAccountStore } from "@keplr-ewallet-sandbox-osmosis/stores/account";

interface IntegratedWalletButtonProps {
  isEwalletAuthenticated: boolean;
  ewalletAccounts: AccountData[];
  osmosisAccount: Key | null;
  onConnect: () => void;
  onDisconnect: () => void;
  isConnecting: boolean;
}

export function IntegratedWalletButton({
  isEwalletAuthenticated,
  ewalletAccounts,
  osmosisAccount,
  onConnect,
  onDisconnect,
  isConnecting,
}: IntegratedWalletButtonProps) {
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [dropdownPosition, setDropdownPosition] = useState({
    top: 0,
    right: 0,
  });
  const dropdownRef = useRef<HTMLDivElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);

  const { onOpenWalletSelect } = useWalletSelect();
  const [osmosisWallet, setOsmosisWallet] = useState<any>({
    isWalletConnected: false,
    address: null,
    disconnect: () => Promise.resolve(),
  });

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node) &&
        buttonRef.current &&
        !buttonRef.current.contains(event.target as Node)
      ) {
        setIsDropdownOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  useEffect(() => {
    // Subscribe to Zustand store state changes
    const unsubscribe = useAccountStore.subscribe((state) => {
      setOsmosisWallet({
        address: state.address,
        isWalletConnected: state.isConnected,
        walletStatus: state.isConnected ? "Connected" : "Disconnected",
        walletName: state.currentWallet?.name || null,
        disconnect: () => state.disconnect(),
        connect: (walletName?: string) => state.connect(walletName),
      });
    });

    // Set initial state
    const initialState = useAccountStore.getState();
    setOsmosisWallet({
      address: initialState.address,
      isWalletConnected: initialState.isConnected,
      walletStatus: initialState.isConnected ? "Connected" : "Disconnected",
      walletName: initialState.currentWallet?.name || null,
      disconnect: () => initialState.disconnect(),
      connect: (walletName?: string) => initialState.connect(walletName),
    });

    return unsubscribe;
  }, []);

  useEffect(() => {
    if (isDropdownOpen && buttonRef.current) {
      const rect = buttonRef.current.getBoundingClientRect();
      setDropdownPosition({
        top: rect.bottom + 8,
        right: window.innerWidth - rect.right,
      });
    }
  }, [isDropdownOpen]);

  const formatAddress = (address: string) => {
    return `${address.slice(0, 8)}...${address.slice(-6)}`;
  };

  const handleDisconnectAll = async () => {
    if (isEwalletAuthenticated) {
      onDisconnect();
    }
    if (osmosisWallet.isWalletConnected) {
      await osmosisWallet.disconnect();
    }
    setIsDropdownOpen(false);
  };

  const handleCosmosConnect = () => {
    onOpenWalletSelect({
      walletOptions: [
        {
          walletType: "cosmos",
          chainId: useAccountStore.getState().osmosisChainId,
        },
      ],
    });
  };

  const isAnyWalletConnected =
    isEwalletAuthenticated || osmosisWallet.isWalletConnected;

  if (isConnecting) {
    return (
      <div className="flex items-center gap-2">
        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-purple-500"></div>
        <span className="text-sm">Connecting...</span>
      </div>
    );
  }

  if (!isAnyWalletConnected) {
    return (
      <button
        onClick={onConnect}
        className="px-4 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition-colors"
      >
        Connect Wallet
      </button>
    );
  }

  const DropdownContent = () => (
    <div
      ref={dropdownRef}
      className="fixed w-80 bg-white rounded-lg shadow-2xl border border-gray-200 backdrop-blur-sm z-[99999]"
      style={{
        top: dropdownPosition.top,
        right: dropdownPosition.right,
      }}
    >
      <div className="p-4">
        <h3 className="font-semibold text-black mb-3">Connected Wallet</h3>

        {/* Show only connected wallet */}
        {isEwalletAuthenticated && osmosisAccount && (
          <div className="mb-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-700">eWallet</span>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-green-500"></div>
                <span className="text-xs text-gray-500">Connected</span>
              </div>
            </div>
            <div className="bg-gray-50 p-3 rounded-lg">
              <div className="text-xs text-gray-600 mb-1">Address:</div>
              <div className="text-sm text-black font-mono break-all">
                {osmosisAccount.bech32Address}
              </div>
            </div>
          </div>
        )}

        {osmosisWallet.isWalletConnected && osmosisWallet.address && (
          <div className="mb-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-700">
                Cosmos Wallet
              </span>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-green-500"></div>
                <span className="text-xs text-gray-500">Connected</span>
              </div>
            </div>
            <div className="bg-gray-50 p-3 rounded-lg">
              <div className="text-xs text-gray-600 mb-1">Address:</div>
              <div className="text-sm text-black font-mono break-all">
                {osmosisWallet.address}
              </div>
            </div>
          </div>
        )}

        {!isEwalletAuthenticated && !osmosisWallet.isWalletConnected && (
          <div className="mb-4 text-center text-gray-500">
            <p className="text-sm">No wallet connected</p>
          </div>
        )}

        <div className="flex flex-col gap-2">
          {!isEwalletAuthenticated && !osmosisWallet.isWalletConnected && (
            <>
              <button
                onClick={() => {
                  onConnect();
                  setIsDropdownOpen(false);
                }}
                className="w-full px-3 py-2 text-sm bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition-colors"
              >
                Connect eWallet
              </button>
              <button
                onClick={() => {
                  handleCosmosConnect();
                  setIsDropdownOpen(false);
                }}
                className="w-full px-3 py-2 text-sm bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
              >
                Connect Cosmos Wallet
              </button>
            </>
          )}

          {(isEwalletAuthenticated || osmosisWallet.isWalletConnected) && (
            <button
              onClick={handleDisconnectAll}
              className="w-full px-3 py-2 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
            >
              Disconnect Wallet
            </button>
          )}
        </div>
      </div>
    </div>
  );

  return (
    <div className="relative">
      <button
        ref={buttonRef}
        onClick={() => setIsDropdownOpen(!isDropdownOpen)}
        className="flex items-center gap-2 px-4 py-2 bg-gray-800 rounded-lg hover:bg-gray-900 transition-colors"
      >
        <div className="w-2 h-2 bg-green-500 rounded-full"></div>
        <span className="text-sm font-medium text-white">
          {isEwalletAuthenticated && osmosisAccount
            ? formatAddress(osmosisAccount.bech32Address)
            : osmosisWallet.isWalletConnected && osmosisWallet.address
              ? formatAddress(osmosisWallet.address)
              : "Connected"}
        </span>
        <svg
          className={`w-4 h-4 transition-transform text-white ${
            isDropdownOpen ? "rotate-180" : ""
          }`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 9l-7 7-7-7"
          />
        </svg>
      </button>

      {isDropdownOpen &&
        typeof window !== "undefined" &&
        createPortal(<DropdownContent />, document.body)}
    </div>
  );
}
