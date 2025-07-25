"use client";

import { useState, useEffect } from "react";
import type { AccountData } from "@cosmjs/amino";
import type { Key } from "@keplr-wallet/types";

import { IntegratedWalletModal } from "@keplr-ewallet-sandbox-osmosis/components/IntegratedWalletModal";
import { IntegratedWalletButton } from "@keplr-ewallet-sandbox-osmosis/components/IntegratedWalletButton";
import { useWalletSelect } from "@keplr-ewallet-sandbox-osmosis/contexts/WalletSelectProvider";
import { useAccountStore } from "@keplr-ewallet-sandbox-osmosis/stores/account";
import type { LoginMethod } from "@keplr-ewallet-sandbox-osmosis/components/IntegratedWalletModal";
import {
  executeSwap,
  executeSwapWithEWallet,
  executeSwapWithEWalletSigner,
  calculateSwapAmount,
  TOKENS,
  OSMOSIS_POOLS,
} from "@keplr-ewallet-sandbox-osmosis/utils/swap";

export const Home: React.FC = () => {
  const [activeTab, setActiveTab] = useState<"swap" | "pools">("swap");
  const [isConnecting, setIsConnecting] = useState(false);
  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);
  const [walletError, setWalletError] = useState<string | null>(null);

  // eWallet state managed by Zustand store
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [accounts, setAccounts] = useState<AccountData[]>([]);
  const [osmosisAccount, setOsmosisAccount] = useState<Key | null>(null);

  // Swap related state
  const [fromToken, setFromToken] = useState("OSMO");
  const [toToken, setToToken] = useState("ATOM");
  const [fromAmount, setFromAmount] = useState("");
  const [toAmount, setToAmount] = useState("");
  const [slippage, setSlippage] = useState(0.5);
  const [isSwapping, setIsSwapping] = useState(false);
  const [swapResult, setSwapResult] = useState<string | null>(null);

  // Get wallet select context
  const { onOpenWalletSelect } = useWalletSelect();

  // Get wallet state directly from Zustand store
  const [osmosisWallet, setOsmosisWallet] = useState<any>({
    isWalletConnected: false,
    address: null,
    disconnect: () => Promise.resolve(),
  });

  // Subscribe to Zustand store changes for both Keplr and eWallet
  useEffect(() => {
    const unsubscribe = useAccountStore.subscribe((state) => {
      setOsmosisWallet({
        address: state.address,
        isWalletConnected: state.isConnected,
        walletStatus: state.isConnected ? "Connected" : "Disconnected",
        walletName: state.currentWallet?.name || null,
        disconnect: () => state.disconnect(),
        connect: (walletName?: string) => state.connect(walletName),
      });

      // Update eWallet state
      if (state.cosmosWallet) {
        setIsAuthenticated(state.isEwalletConnected);
        setAccounts([...state.ewalletAccounts]);
        setOsmosisAccount(state.osmosisEwalletAccount);
      }
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

    // Set initial eWallet state
    if (initialState.cosmosWallet) {
      setIsAuthenticated(initialState.isEwalletConnected);
      setAccounts([...initialState.ewalletAccounts]);
      setOsmosisAccount(initialState.osmosisEwalletAccount);
    }

    return unsubscribe;
  }, []);

  // Already initialized in WalletSelectProvider, so removed and managed by AccountStore

  const handleConnect = async () => {
    setIsWalletModalOpen(true);
  };

  const handleDisconnect = async () => {
    try {
      await useAccountStore.getState().disconnectEWallet();
    } catch (error) {
      console.error("Failed to disconnect:", error);
    }
  };

  const handleLoginMethod = async (method: LoginMethod) => {
    setIsConnecting(true);
    setWalletError(null); // Reset error state

    try {
      if (method === "google") {
        // Use AccountStore's connectEWallet method
        await useAccountStore.getState().connectEWallet("google");
        setIsWalletModalOpen(false); // Close modal only on success
      } else if (method === "keplr") {
        onOpenWalletSelect({
          walletOptions: [
            {
              walletType: "cosmos",
              chainId: useAccountStore.getState().osmosisChainId,
            },
          ],
          onConnect: () => {
            setIsWalletModalOpen(false);
          },
          onError: (error) => {
            setWalletError(error.message);
          },
        });
        setIsWalletModalOpen(false); // Keplr moves to WalletSelect modal
      }
    } catch (error) {
      console.error("Login failed:", error);
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error occurred";
      setWalletError(errorMessage);
    } finally {
      setIsConnecting(false);
    }
  };

  const isAnyWalletConnected =
    isAuthenticated || osmosisWallet.isWalletConnected;

  // Swap related functions
  const calculateEstimatedOutput = (inputAmount: string) => {
    if (!inputAmount || inputAmount === "0") {
      setToAmount("");
      return;
    }

    const fromTokenInfo = TOKENS[fromToken as keyof typeof TOKENS];
    const toTokenInfo = TOKENS[toToken as keyof typeof TOKENS];

    if (!fromTokenInfo || !toTokenInfo) return;

    const pool = OSMOSIS_POOLS.find(
      (p) =>
        (p.token1 === fromTokenInfo.denom && p.token2 === toTokenInfo.denom) ||
        (p.token1 === toTokenInfo.denom && p.token2 === fromTokenInfo.denom),
    );

    if (!pool) return;

    const isToken1ToToken2 = pool.token1 === fromTokenInfo.denom;
    const inputReserve = isToken1ToToken2
      ? pool.token1Reserve
      : pool.token2Reserve;
    const outputReserve = isToken1ToToken2
      ? pool.token2Reserve
      : pool.token1Reserve;

    const inputInBaseUnit = (
      parseFloat(inputAmount) * Math.pow(10, fromTokenInfo.decimals)
    ).toString();

    try {
      const { outputAmount } = calculateSwapAmount(
        inputInBaseUnit,
        inputReserve,
        outputReserve,
        slippage,
      );

      const outputInDisplayUnit = (
        parseInt(outputAmount) / Math.pow(10, toTokenInfo.decimals)
      ).toFixed(6);
      setToAmount(outputInDisplayUnit);
    } catch (error) {
      console.error("Failed to calculate swap amount:", error);
      setToAmount("");
    }
  };

  const handleSwap = async () => {
    if (!fromAmount || (!isAuthenticated && !osmosisWallet.isWalletConnected))
      return;

    setIsSwapping(true);
    setSwapResult(null);

    try {
      const fromTokenInfo = TOKENS[fromToken as keyof typeof TOKENS];
      const toTokenInfo = TOKENS[toToken as keyof typeof TOKENS];

      if (!fromTokenInfo || !toTokenInfo) {
        throw new Error("Invalid token selection");
      }

      const inputAmount = (
        parseFloat(fromAmount) * Math.pow(10, fromTokenInfo.decimals)
      ).toString();

      let txHash: string;

      // When using eWallet (Google social login)
      const accountStoreState = useAccountStore.getState();
      if (
        isAuthenticated &&
        accountStoreState.ewalletSigner &&
        osmosisAccount
      ) {
        console.log("Using eWallet signer for swap transaction");
        txHash = await executeSwapWithEWalletSigner(
          {
            fromToken: fromTokenInfo.denom,
            toToken: toTokenInfo.denom,
            amount: inputAmount,
            slippage,
          },
          accountStoreState.ewalletSigner,
          osmosisAccount,
        );
      }
      // When using Keplr wallet connection
      else if (osmosisWallet.isWalletConnected && osmosisWallet.address) {
        console.log("Using Keplr wallet for swap transaction");
        txHash = await executeSwap(
          {
            fromToken: fromTokenInfo.denom,
            toToken: toTokenInfo.denom,
            amount: inputAmount,
            slippage,
          },
          osmosisWallet.address,
        );
      } else {
        throw new Error("No wallet connected");
      }

      setSwapResult(txHash);
      setFromAmount("");
      setToAmount("");
    } catch (error) {
      console.error("Swap failed:", error);
      alert(
        `Swap failed: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
    } finally {
      setIsSwapping(false);
    }
  };

  const handleTokenSwitch = () => {
    setFromToken(toToken);
    setToToken(fromToken);
    setFromAmount(toAmount);
    setToAmount(fromAmount);
  };

  // Calculate toAmount whenever fromAmount changes
  useEffect(() => {
    if (fromAmount) {
      calculateEstimatedOutput(fromAmount);
    }
  }, [fromAmount, fromToken, toToken, slippage]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-indigo-900 to-blue-900">
      {/* Navigation */}
      <nav className="border-b border-white/10 bg-black/20 backdrop-blur-md">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center space-x-8">
              <div className="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                Osmosis
              </div>
              <div className="flex space-x-6">
                <button
                  onClick={() => setActiveTab("swap")}
                  className={`px-4 py-2 rounded-lg font-medium transition-all ${activeTab === "swap"
                      ? "bg-purple-500 text-white"
                      : "text-gray-300 hover:text-white hover:bg-white/5"
                    }`}
                >
                  Swap
                </button>
                <button
                  onClick={() => setActiveTab("pools")}
                  className={`px-4 py-2 rounded-lg font-medium transition-all ${activeTab === "pools"
                      ? "bg-purple-500 text-white"
                      : "text-gray-300 hover:text-white hover:bg-white/5"
                    }`}
                >
                  Pools
                </button>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <IntegratedWalletButton
                isEwalletAuthenticated={isAuthenticated}
                ewalletAccounts={accounts}
                osmosisAccount={osmosisAccount}
                onConnect={handleConnect}
                onDisconnect={handleDisconnect}
                isConnecting={isConnecting}
              />
            </div>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {!isAnyWalletConnected ? (
          <div className="text-center py-20">
            <div className="mb-8">
              <div className="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full mb-4">
                <svg
                  className="w-8 h-8 text-white"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"
                  />
                </svg>
              </div>
              <h2 className="text-3xl font-bold text-white mb-4">
                Welcome to Osmosis
              </h2>
              <p className="text-lg text-gray-300 max-w-2xl mx-auto">
                Connect your wallet to access the leading decentralized exchange
                for Cosmos. Swap tokens, provide liquidity, and earn rewards.
              </p>
            </div>
            <button
              onClick={handleConnect}
              className="bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white px-8 py-4 rounded-lg font-semibold text-lg transition-all transform hover:scale-105"
            >
              Connect Wallet
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            {/* Main Trading Interface */}
            <div className="lg:col-span-2">
              <div className="bg-black/20 backdrop-blur-md rounded-2xl border border-white/10 p-6">
                {activeTab === "swap" ? (
                  <div>
                    <h3 className="text-xl font-bold text-white mb-6">Swap</h3>
                    <div className="space-y-4">
                      {/* From Token */}
                      <div className="bg-white/5 rounded-xl p-4">
                        <div className="flex justify-between items-center mb-2">
                          <span className="text-sm text-gray-400">From</span>
                          <span className="text-sm text-gray-400">
                            Balance: 0.00
                          </span>
                        </div>
                        <div className="flex items-center space-x-3">
                          <input
                            type="number"
                            placeholder="0.00"
                            value={fromAmount}
                            onChange={(e) => setFromAmount(e.target.value)}
                            className="flex-1 bg-transparent text-white text-2xl font-bold outline-none"
                          />
                          <div className="flex items-center space-x-2 bg-white/10 rounded-lg px-3 py-2">
                            <span className="text-2xl">
                              {TOKENS[fromToken as keyof typeof TOKENS]?.icon ||
                                "🔗"}
                            </span>
                            <span className="text-white font-medium">
                              {fromToken}
                            </span>
                          </div>
                        </div>
                      </div>

                      {/* Swap Button */}
                      <div className="flex justify-center">
                        <button
                          onClick={handleTokenSwitch}
                          className="bg-white/10 hover:bg-white/20 rounded-full p-2 transition-colors"
                        >
                          <svg
                            className="w-5 h-5 text-white"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"
                            />
                          </svg>
                        </button>
                      </div>

                      {/* To Token */}
                      <div className="bg-white/5 rounded-xl p-4">
                        <div className="flex justify-between items-center mb-2">
                          <span className="text-sm text-gray-400">To</span>
                          <span className="text-sm text-gray-400">
                            Balance: 0.00
                          </span>
                        </div>
                        <div className="flex items-center space-x-3">
                          <input
                            type="number"
                            placeholder="0.00"
                            value={toAmount}
                            readOnly
                            className="flex-1 bg-transparent text-white text-2xl font-bold outline-none"
                          />
                          <div className="flex items-center space-x-2 bg-white/10 rounded-lg px-3 py-2">
                            <span className="text-2xl">
                              {TOKENS[toToken as keyof typeof TOKENS]?.icon ||
                                "🔗"}
                            </span>
                            <span className="text-white font-medium">
                              {toToken}
                            </span>
                          </div>
                        </div>
                      </div>

                      {/* Slippage Setting */}
                      <div className="bg-white/5 rounded-xl p-4">
                        <div className="flex justify-between items-center mb-2">
                          <span className="text-sm text-gray-400">
                            Slippage Tolerance
                          </span>
                          <span className="text-sm text-white">
                            {slippage}%
                          </span>
                        </div>
                        <input
                          type="range"
                          min="0.1"
                          max="5"
                          step="0.1"
                          value={slippage}
                          onChange={(e) =>
                            setSlippage(parseFloat(e.target.value))
                          }
                          className="w-full"
                        />
                      </div>

                      {/* Swap Button */}
                      <button
                        onClick={handleSwap}
                        disabled={
                          !fromAmount ||
                          (!isAuthenticated &&
                            !osmosisWallet.isWalletConnected) ||
                          isSwapping
                        }
                        className="w-full bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 disabled:from-gray-500 disabled:to-gray-500 disabled:cursor-not-allowed text-white py-4 rounded-xl font-semibold text-lg transition-all"
                      >
                        {isSwapping ? "Swapping..." : "Swap"}
                      </button>

                      {/* Swap Result */}
                      {swapResult && (
                        <div className="bg-green-500/20 border border-green-500/50 rounded-xl p-4">
                          <div className="text-green-400 font-semibold mb-2">
                            Swap Successful!
                          </div>
                          <div className="text-sm text-green-300">
                            Transaction Hash: {swapResult.slice(0, 10)}...
                            {swapResult.slice(-10)}
                          </div>
                          <a
                            href={`https://mintscan.io/osmosis/txs/${swapResult}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-400 hover:underline text-sm"
                          >
                            View on Mintscan
                          </a>
                        </div>
                      )}
                    </div>
                  </div>
                ) : (
                  <div>
                    <h3 className="text-xl font-bold text-white mb-6">
                      Liquidity Pools
                    </h3>
                    <div className="text-center py-20">
                      <div className="text-gray-400 mb-4">
                        <svg
                          className="w-16 h-16 mx-auto mb-4"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={1}
                            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                          />
                        </svg>
                      </div>
                      <h4 className="text-white font-semibold mb-2">
                        Pools Coming Soon
                      </h4>
                      <p className="text-gray-400">
                        Liquidity pool functionality will be available soon.
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Sidebar */}
            <div className="space-y-6">
              {/* Portfolio */}
              <div className="bg-black/20 backdrop-blur-md rounded-2xl border border-white/10 p-6">
                <h3 className="text-lg font-bold text-white mb-4">Portfolio</h3>
                <div className="text-center py-8">
                  <div className="text-3xl font-bold text-white mb-2">
                    $0.00
                  </div>
                  <div className="text-gray-400">Total Balance</div>
                </div>
              </div>
            </div>
          </div>
        )}
      </main>

      {/* Wallet Modal */}
      <IntegratedWalletModal
        isOpen={isWalletModalOpen}
        onClose={() => {
          setIsWalletModalOpen(false);
          setWalletError(null); // Reset error when closing modal
        }}
        onEwalletConnect={handleLoginMethod}
        isConnecting={isConnecting}
        error={walletError}
        onErrorDismiss={() => setWalletError(null)}
      />
    </div>
  );
};
