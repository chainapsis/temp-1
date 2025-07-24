import type { Key, SettledResponse } from "@keplr-wallet/types";

import type { CosmosEWallet } from "@keplr-ewallet-sdk-cosmos/cosmos_ewallet";

export function getKeysSettled(
  this: CosmosEWallet,
  chainIds: string[],
): Promise<SettledResponse<Key>[]> {
  return Promise.allSettled(chainIds.map((chainId) => this.getKey(chainId)));
}
