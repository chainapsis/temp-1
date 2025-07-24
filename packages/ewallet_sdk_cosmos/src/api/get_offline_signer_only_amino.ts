import type { OfflineAminoSigner } from "@cosmjs/amino";
import type { KeplrSignOptions } from "@keplr-wallet/types";

import { CosmosEWallet } from "@keplr-ewallet-sdk-cosmos/cosmos_ewallet";

export function getOfflineSignerOnlyAmino(
  this: CosmosEWallet,
  chainId: string,
  signOptions?: KeplrSignOptions,
): OfflineAminoSigner {
  return {
    getAccounts: this.getAccounts,
    signAmino: (signerAddress, signDoc) =>
      this.signAmino(chainId, signerAddress, signDoc, signOptions),
  };
}
