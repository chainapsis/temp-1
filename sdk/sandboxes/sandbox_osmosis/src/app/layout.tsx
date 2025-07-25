import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";

import { WalletSelectProvider } from "@keplr-ewallet-sandbox-osmosis/contexts/WalletSelectProvider";

import "./globals.scss";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Osmosis + eWallet",
  description: "Osmosis-style interface with eWallet integration",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable}`}
        suppressHydrationWarning
      >
        <WalletSelectProvider>{children}</WalletSelectProvider>
      </body>
    </html>
  );
}
