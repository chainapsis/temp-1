"use client";

import { WalletStatus } from "@cosmos-kit/core";
import { chains } from "chain-registry";
import { create } from "zustand";
import type { CosmosEWallet } from "@keplr-ewallet/ewallet-sdk-cosmos";
import type { KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";
import {
  WalletConfig,
  getWalletByName,
} from "@keplr-ewallet-sandbox-osmosis/config/wallet-registry";
import type { OfflineDirectSigner } from "@cosmjs/proto-signing";

// Keplr types
declare global {
  interface Window {
    keplr?: {
      enable: (chainId: string) => Promise<void>;
      getOfflineSignerOnlyAmino: (chainId: string) => any;
      getKey: (chainId: string) => Promise<any>;
      getOfflineSigner: (chainId: string) => any;
    };
  }
}

interface AccountState {
  wallets: any[];
  currentWallet: any;
  isConnected: boolean;
  address: string | null;
  refreshRequests: number;
  // eWallet related state
  eWallet: KeplrEWallet | null;
  cosmosWallet: CosmosEWallet | null;
  isEwalletConnected: boolean;
  ewalletAddress: string | null;
  ewalletAccounts: any[];
  osmosisEwalletAccount: any;
  ewalletSigner: OfflineDirectSigner | null;
}

interface AccountActions {
  addWallet: (wallet: any) => Promise<any>;
  setCurrentWallet: (wallet: any) => void;
  connect: (walletName?: string) => Promise<void>;
  connectWallet: (walletConfig: WalletConfig) => Promise<void>;
  disconnect: () => Promise<void>;
  getWalletRepo: (chainId: string) => any;
  onMounted: () => Promise<void>;
  onUnmounted: () => void;
  refresh: () => void;
  // eWallet related actions
  setEWallet: (eWallet: KeplrEWallet | null) => void;
  setCosmosWallet: (cosmosWallet: CosmosEWallet | null) => void;
  connectEWallet: (method: string) => Promise<void>;
  disconnectEWallet: () => Promise<void>;
}

interface AccountStoreType extends AccountState, AccountActions {
  osmosisChainId: string;
  osmosisWallet: {
    address: string | null;
    isWalletConnected: boolean;
    walletStatus: WalletStatus;
    walletName: string | null;
    disconnect: () => Promise<void>;
    connect: (walletName?: string) => Promise<void>;
  };
}

// Osmosis chain configuration
const osmosisChain = chains.find(
  (chain: any) => chain.chain_name === "osmosis",
);

export const useAccountStore = create<AccountStoreType>((set, get) => ({
  // State
  wallets: [],
  currentWallet: null,
  isConnected: false,
  address: null,
  refreshRequests: 0,
  // eWallet related initial state
  eWallet: null,
  cosmosWallet: null,
  isEwalletConnected: false,
  ewalletAddress: null,
  ewalletAccounts: [],
  osmosisEwalletAccount: null,
  ewalletSigner: null,

  // Computed values
  get osmosisChainId() {
    return osmosisChain?.chain_id || "osmosis-1";
  },

  get osmosisWallet() {
    const state = get();
    return {
      address: state.address,
      isWalletConnected: state.isConnected,
      walletStatus: state.isConnected
        ? WalletStatus.Connected
        : WalletStatus.Disconnected,
      walletName: state.currentWallet?.name || null,
      disconnect: () => state.disconnect(),
      connect: (walletName?: string) => state.connect(walletName),
    };
  },

  // Actions
  refresh: () => {
    set((state) => ({ refreshRequests: state.refreshRequests + 1 }));
  },

  addWallet: async (wallet: any) => {
    set((state) => ({
      wallets: [...state.wallets, wallet],
      currentWallet: wallet,
    }));
    get().refresh();
    return wallet;
  },

  setCurrentWallet: (wallet: any) => {
    set({ currentWallet: wallet });
    get().refresh();
  },

  connect: async (walletName?: string) => {
    if (!walletName) {
      throw new Error("Wallet name is required");
    }

    const walletConfig = getWalletByName(walletName);
    if (!walletConfig) {
      throw new Error(`Wallet ${walletName} not found in registry`);
    }

    try {
      const chainId = osmosisChain?.chain_id || "osmosis-1";

      // Use the wallet config to get the wallet instance
      const signer = await walletConfig.getWallet?.(chainId);

      if (!signer) {
        throw new Error(`Failed to get wallet instance for ${walletName}`);
      }

      // Get accounts
      const accounts = await signer.getAccounts();

      if (accounts.length > 0) {
        set({
          isConnected: true,
          address: accounts[0].address,
          currentWallet: {
            name: walletConfig.name,
            prettyName: walletConfig.prettyName,
            signer: signer,
            accounts: accounts,
            config: walletConfig,
          },
        });
      } else {
        throw new Error("No accounts found");
      }
    } catch (error) {
      console.error(`Failed to connect to ${walletName}:`, error);
      throw error;
    }
    get().refresh();
  },

  connectWallet: async (walletConfig: WalletConfig) => {
    try {
      const chainId = osmosisChain?.chain_id || "osmosis-1";

      // Handle eWallet differently
      if (walletConfig.isEwallet) {
        // For eWallet, we use the existing connectEWallet method
        await get().connectEWallet("google");
        return;
      }

      // Use the wallet config to get the wallet instance
      const signer = await walletConfig.getWallet?.(chainId);

      if (!signer) {
        throw new Error(
          `Failed to get wallet instance for ${walletConfig.name}`,
        );
      }

      // Get accounts
      const accounts = await signer.getAccounts();

      if (accounts.length > 0) {
        set({
          isConnected: true,
          address: accounts[0].address,
          currentWallet: {
            name: walletConfig.name,
            prettyName: walletConfig.prettyName,
            signer: signer,
            accounts: accounts,
            config: walletConfig,
          },
        });
      } else {
        throw new Error("No accounts found");
      }
    } catch (error) {
      console.error(`Failed to connect to ${walletConfig.name}:`, error);
      throw error;
    }
    get().refresh();
  },

  disconnect: async () => {
    set({
      isConnected: false,
      address: null,
      currentWallet: null,
    });
    get().refresh();
  },

  // eWallet related actions
  setEWallet: (eWallet: KeplrEWallet | null) => {
    set({ eWallet });
  },

  setCosmosWallet: (cosmosWallet: CosmosEWallet | null) => {
    set({ cosmosWallet });
  },

  connectEWallet: async (method: string) => {
    const { eWallet, cosmosWallet } = get();
    if (!cosmosWallet) {
      throw new Error("CosmosWallet is not initialized");
    }

    try {
      await eWallet?.signIn(method as "google");
      // Get account information
      const accounts = await cosmosWallet.getAccounts();
      const osmosisAccount = await cosmosWallet.getKey("osmosis-1");

      // Create eWallet signer using cosmos variant
      const ewalletSigner = cosmosWallet.getOfflineSigner("osmosis-1");

      set({
        isEwalletConnected: true,
        ewalletAddress: osmosisAccount?.bech32Address || null,
        ewalletAccounts: [...accounts],
        osmosisEwalletAccount: osmosisAccount,
        ewalletSigner: ewalletSigner,
      });

      get().refresh();
    } catch (error) {
      console.error("Failed to connect eWallet:", error);
      throw error;
    }
  },

  disconnectEWallet: async () => {
    const { eWallet } = get();

    try {
      if (eWallet) {
        await eWallet.signOut();
      }

      set({
        isEwalletConnected: false,
        ewalletAddress: null,
        ewalletAccounts: [],
        osmosisEwalletAccount: null,
        ewalletSigner: null,
      });

      get().refresh();
    } catch (error) {
      console.error("Failed to disconnect eWallet:", error);
      throw error;
    }
  },

  getWalletRepo: (chainId: string) => {
    return get().currentWallet;
  },

  onMounted: async () => {
    // Initialize store
    get().refresh();
  },

  onUnmounted: () => {
    // Cleanup
    set({
      wallets: [],
      currentWallet: null,
      isConnected: false,
      address: null,
    });
  },
}));

// Legacy AccountStore class for compatibility
export class AccountStore {
  constructor() { }

  get osmosisChainId() {
    return useAccountStore.getState().osmosisChainId;
  }

  get osmosisWallet() {
    return useAccountStore.getState().osmosisWallet;
  }

  get wallets() {
    return useAccountStore.getState().wallets;
  }

  async addWallet(wallet: any) {
    return useAccountStore.getState().addWallet(wallet);
  }

  setCurrentWallet(wallet: any) {
    useAccountStore.getState().setCurrentWallet(wallet);
  }

  async connect(walletName?: string) {
    return useAccountStore.getState().connect(walletName);
  }

  async connectWallet(walletConfig: WalletConfig) {
    return useAccountStore.getState().connectWallet(walletConfig);
  }

  async disconnect() {
    return useAccountStore.getState().disconnect();
  }

  getWalletRepo(chainId: string) {
    return useAccountStore.getState().getWalletRepo(chainId);
  }

  async onMounted() {
    return useAccountStore.getState().onMounted();
  }

  onUnmounted() {
    useAccountStore.getState().onUnmounted();
  }
}
