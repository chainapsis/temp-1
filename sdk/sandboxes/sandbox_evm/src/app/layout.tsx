import "@rainbow-me/rainbowkit/styles.css";
import "@keplr-ewallet-sandbox-evm/styles/globals.css";

import { ScaffoldEthAppWithProviders } from "@keplr-ewallet-sandbox-evm/components/ScaffoldEthAppWithProviders";
import { getMetadata } from "@keplr-ewallet-sandbox-evm/utils/scaffold-eth/getMetadata";

export const metadata = getMetadata({
  title: "Keplr E-Wallet Sandbox EVM",
  description: "Keplr E-Wallet Sandbox EVM",
});

const ScaffoldEthApp = ({ children }: { children: React.ReactNode }) => {
  return (
    <html suppressHydrationWarning className={``}>
      <body>
        <ScaffoldEthAppWithProviders>{children}</ScaffoldEthAppWithProviders>
      </body>
    </html>
  );
};

export default ScaffoldEthApp;
