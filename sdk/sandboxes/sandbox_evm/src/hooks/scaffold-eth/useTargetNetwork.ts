import { useEffect, useMemo } from "react";
import { useAccount } from "wagmi";

import scaffoldConfig from "@keplr-ewallet-sandbox-evm/../scaffold.config";
import { useGlobalState } from "@keplr-ewallet-sandbox-evm/services/store/store";
import { ChainWithAttributes } from "@keplr-ewallet-sandbox-evm/utils/scaffold-eth";
import { NETWORKS_EXTRA_DATA } from "@keplr-ewallet-sandbox-evm/utils/scaffold-eth";

/**
 * Retrieves the connected wallet's network from scaffold.config or defaults to the 0th network in the list if the wallet is not connected.
 */
export function useTargetNetwork(): { targetNetwork: ChainWithAttributes } {
  const { chain } = useAccount();
  const targetNetwork = useGlobalState(({ targetNetwork }) => targetNetwork);
  const setTargetNetwork = useGlobalState(
    ({ setTargetNetwork }) => setTargetNetwork,
  );

  useEffect(() => {
    const newSelectedNetwork = scaffoldConfig.targetNetworks.find(
      (targetNetwork) => targetNetwork.id === chain?.id,
    );
    if (newSelectedNetwork && newSelectedNetwork.id !== targetNetwork.id) {
      setTargetNetwork({
        ...newSelectedNetwork,
        ...NETWORKS_EXTRA_DATA[newSelectedNetwork.id],
      });
    }
  }, [chain?.id, setTargetNetwork, targetNetwork.id]);

  return useMemo(() => ({ targetNetwork }), [targetNetwork]);
}
