"use client";

import { RainbowKitProvider, Wallet } from "@rainbow-me/rainbowkit";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import { WagmiProvider } from "wagmi";

import { Footer } from "@keplr-ewallet-sandbox-evm/components/Footer";
import { Header } from "@keplr-ewallet-sandbox-evm/components/Header";
import { BlockieAvatar } from "@keplr-ewallet-sandbox-evm/components/scaffold-eth";
import { useInitializeNativeCurrencyPrice } from "@keplr-ewallet-sandbox-evm/hooks/scaffold-eth";
import {
  keplrEWallet,
  wagmiConfigWithKeplr,
} from "@keplr-ewallet-sandbox-evm/services/web3/wagmiConfig";
import {
  KeplrEwalletProvider,
  useKeplrEwallet,
} from "@keplr-ewallet-sandbox-evm/contexts/KeplrEwalletProvider";

const ScaffoldEthApp = ({ children }: { children: React.ReactNode }) => {
  useInitializeNativeCurrencyPrice();

  return (
    <>
      <div className={`flex flex-col min-h-screen `}>
        <Header />
        <main className="relative flex flex-col flex-1">{children}</main>
        <Footer />
      </div>
      <Toaster />
    </>
  );
};

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
});

const WagmiWithKeplr = ({ children }: { children: React.ReactNode }) => {
  const { eWallet } = useKeplrEwallet();

  const wagmiConfig = (() => {
    if (!eWallet) {
      return wagmiConfigWithKeplr();
    }

    return wagmiConfigWithKeplr(keplrEWallet(eWallet));
  })();

  return <WagmiProvider config={wagmiConfig}>{children}</WagmiProvider>;
};

export const ScaffoldEthAppWithProviders = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <KeplrEwalletProvider initialLoginMethods={["email", "google"]}>
      <WagmiWithKeplr>
        <QueryClientProvider client={queryClient}>
          <RainbowKitProvider avatar={BlockieAvatar}>
            <ScaffoldEthApp>{children}</ScaffoldEthApp>
          </RainbowKitProvider>
        </QueryClientProvider>
      </WagmiWithKeplr>
    </KeplrEwalletProvider>
  );
};
