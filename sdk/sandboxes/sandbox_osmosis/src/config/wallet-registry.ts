import { ChainRecord } from "@cosmos-kit/core";

export interface WalletConfig {
  id: string;
  name: string;
  prettyName: string;
  logo: string;
  mode: "extension" | "wallet-connect" | "ewallet";
  downloads: {
    desktop?: string;
    mobile?: string;
  };
  features?: {
    notifications?: boolean;
    staking?: boolean;
    governance?: boolean;
  };
  mobileDisabled?: boolean;
  rejectMessage?: {
    source: string;
    target: string;
  };
  connectEventNamesOnWindow?: string[];
  isLazyInstall?: boolean;
  lazyInstall?: () => Promise<void>;
  getWallet?: (chainId: string) => Promise<any>;
  // eWallet specific properties
  isEwallet?: boolean;
  socialMethods?: string[];
}

export const CosmosWalletRegistry: WalletConfig[] = [
  {
    id: "keplr-extension",
    name: "keplr",
    prettyName: "Keplr",
    logo: "/images/wallets/keplr.png",
    mode: "extension",
    downloads: {
      desktop: "https://chrome.google.com/webstore/detail/keplr/dmkamcknogkgcdfhhbddcghachkejeap",
    },
    features: {
      notifications: true,
      staking: true,
      governance: true,
    },
    connectEventNamesOnWindow: ["keplr_keystorechange"],
    getWallet: async (chainId: string) => {
      if (typeof window === "undefined" || !window.keplr) {
        throw new Error("Keplr extension not found");
      }
      
      await window.keplr.enable(chainId);
      return window.keplr.getOfflineSignerOnlyAmino(chainId);
    },
  },
  {
    id: "ewallet-social",
    name: "ewallet",
    prettyName: "eWallet",
    logo: "/images/wallets/ewallet.png",
    mode: "ewallet",
    downloads: {}, // No downloads needed for eWallet
    features: {
      notifications: true,
      staking: true,
      governance: true,
    },
    isEwallet: true,
    socialMethods: ["google"],
    getWallet: async (chainId: string) => {
      // eWallet doesn't use traditional wallet interface
      // Return a placeholder that indicates it's an eWallet
      return {
        isEwallet: true,
        chainId,
        type: "ewallet"
      };
    },
  },
];

export const getWalletById = (id: string): WalletConfig | undefined => {
  return CosmosWalletRegistry.find(wallet => wallet.id === id);
};

export const getWalletByName = (name: string): WalletConfig | undefined => {
  return CosmosWalletRegistry.find(wallet => wallet.name === name);
};

export const getAvailableWallets = (): WalletConfig[] => {
  return CosmosWalletRegistry.filter(wallet => {
    if (typeof window === "undefined") return false;
    
    switch (wallet.name) {
      case "keplr":
        return !!(window as any).keplr;
      case "ewallet":
        return true; // eWallet is always available (no extension needed)
      default:
        return false;
    }
  });
};

export const getInstallableWallets = (): WalletConfig[] => {
  return CosmosWalletRegistry.filter(wallet => {
    if (typeof window === "undefined") return true;
    
    switch (wallet.name) {
      case "keplr":
        return !(window as any).keplr;
      case "ewallet":
        return false; // eWallet doesn't need installation
      default:
        return true;
    }
  });
};

export const getEwalletConfigs = (): WalletConfig[] => {
  return CosmosWalletRegistry.filter(wallet => wallet.isEwallet);
};

export const getExtensionWallets = (): WalletConfig[] => {
  return CosmosWalletRegistry.filter(wallet => wallet.mode === "extension");
};

// Helper function to detect mobile environment
export const isMobile = (): boolean => {
  if (typeof window === "undefined") return false;
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
};

// Helper function to detect in-app browser
export const isInAppBrowser = (): boolean => {
  if (typeof window === "undefined") return false;
  const ua = navigator.userAgent.toLowerCase();
  return ua.includes("wv") || ua.includes("webview") || ua.includes("version/") && ua.includes("chrome");
};

// Filter wallets based on current environment
export const getSelectableWallets = (): WalletConfig[] => {
  const mobile = isMobile();
  const inApp = isInAppBrowser();
  
  return CosmosWalletRegistry.filter(wallet => {
    if (mobile && wallet.mobileDisabled) return false;
    if (inApp && wallet.mode === "extension") return false;
    return true;
  });
};