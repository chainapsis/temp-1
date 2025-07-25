import { useTargetNetwork } from "@keplr-ewallet-sandbox-evm/hooks/scaffold-eth";
import {
  GenericContractsDeclaration,
  contracts,
} from "@keplr-ewallet-sandbox-evm/utils/scaffold-eth/contract";

const DEFAULT_ALL_CONTRACTS: GenericContractsDeclaration[number] = {};

export function useAllContracts() {
  const { targetNetwork } = useTargetNetwork();
  const contractsData = contracts?.[targetNetwork.id];
  // using constant to avoid creating a new object on every call
  return contractsData || DEFAULT_ALL_CONTRACTS;
}
