import type { OfflineAminoSigner } from "@cosmjs/amino";
import type { OfflineDirectSigner } from "@cosmjs/proto-signing";
import type { KeplrSignOptions } from "@keplr-wallet/types";

import type { CosmosEWallet } from "@keplr-ewallet-sdk-cosmos/cosmos_ewallet";

export async function getOfflineSignerAuto(
  this: CosmosEWallet,
  chainId: string,
  signOptions?: KeplrSignOptions,
): Promise<OfflineDirectSigner | OfflineAminoSigner> {
  const key = await this.getKey(chainId);
  if (key.isNanoLedger) {
    return this.getOfflineSignerOnlyAmino(chainId, signOptions);
  }
  return this.getOfflineSigner(chainId, "", signOptions);
}
