import { type KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";

import { CosmosEWallet } from "./cosmos_ewallet";

export interface CosmosEWalletArgs {
  eWallet: KeplrEWallet | null;
}

export async function initCosmosEWallet({
  eWallet,
}: CosmosEWalletArgs): Promise<CosmosEWallet | null> {
  if (eWallet === null) {
    console.error("eWallet is not provided");

    return null;
  }

  return new CosmosEWallet(eWallet);
}
