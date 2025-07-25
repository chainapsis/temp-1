import type { KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";
import {
  isAddress,
  toHex,
  hashMessage,
  keccak256,
  serializeTransaction,
  hashTypedData,
  parseSignature,
} from "viem";
import type { Address, Hex, TypedDataDefinition } from "viem";
import { privateKeyToAccount, publicKeyToAddress } from "viem/accounts";
import { base, mainnet, optimism } from "viem/chains";

import { makeGetPublicKey, makeSign } from "./api";
import type {
  EthEWallet,
  EthSigner,
  EWalletAccount,
  SignFunctionParams,
  SignFunctionResult,
  EthSignMethod,
} from "./types";
import { initEWalletEIP1193Provider } from "./provider";

const SUPPORTED_CHAINS = [mainnet, base, optimism];

export interface InitEthEWalletArgs {
  eWallet: KeplrEWallet | null;
}

export async function initEthEWallet({
  eWallet,
}: InitEthEWalletArgs): Promise<EthEWallet | null> {
  if (eWallet === null) {
    return null;
  }

  const getPublicKey = makeGetPublicKey(eWallet);

  const publicKey = await getPublicKey();
  const address = publicKeyToAddress(publicKey as `0x${string}`);

  const sign = makeSign(eWallet);

  // TODO: add supported chains list for the wallet
  // how to get chain list? hardcoding or get from sdk?
  let activeChainId = 1; // ethereum mainnet
  const chains = SUPPORTED_CHAINS;

  const obj: EthEWallet = {
    type: "ethereum",
    chainId: `eip155:${activeChainId}`,
    address,
    getEthereumProvider: async () => {
      // initial chain should be added first, as first chain is active by default on init provider
      const activeChain =
        chains.find((chain) => chain.id === activeChainId) ?? chains[0];

      const addEthereumChainParameters = [
        activeChain,
        ...chains.filter((chain) => chain.id !== activeChainId),
      ].map((chain) => ({
        chainId: toHex(chain.id),
        chainName: chain.name,
        rpcUrls: chain.rpcUrls.default.http,
        nativeCurrency: chain.nativeCurrency,
        blockExplorerUrls: chain.blockExplorers?.default.url
          ? [chain.blockExplorers.default.url]
          : [],
      }));

      const providerId = `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
      const hasSigner = isAddress(address);

      if (hasSigner) {
        return await initEWalletEIP1193Provider({
          id: providerId,
          signer: {
            sign,
            address,
          },
          chains: addEthereumChainParameters,
        });
      }

      // if signer is not available, only handle public rpc requests
      return await initEWalletEIP1193Provider({
        id: providerId,
        chains: addEthereumChainParameters,
      });
    },
    sign: async (message: string): Promise<Hex> => {
      const result = await sign<"personal_sign">({
        type: "personal_sign",
        data: {
          address,
          message,
        },
      });

      return result.signature;
    },
    switchChain: async (chainId: `0x${string}` | number): Promise<void> => {
      const chainIdNumber =
        typeof chainId === "string" ? parseInt(chainId, 16) : chainId;

      // TODO: send request to ewallet to switch chain
      const chain = chains.find((chain) => chain.id === chainIdNumber);
      if (!chain) {
        throw new Error(`Chain with id ${chainId} not found`);
      }

      activeChainId = chainIdNumber;
    },
  };

  return obj;
}

export async function toViemAccount(
  eWallet: KeplrEWallet,
): Promise<EWalletAccount<"ewallet", Hex>> {
  // TODO: check if eWallet is initialized
  // eWallet.isInitialized()

  const getPublicKey = makeGetPublicKey(eWallet);

  const publicKey = await getPublicKey();
  const address = publicKeyToAddress(publicKey as `0x${string}`);

  if (!address || !isAddress(address)) {
    throw new Error("Invalid address format");
  }

  const sign = makeSign(eWallet);

  const account: EWalletAccount<"ewallet", Address> = {
    address,
    type: "local",
    source: "ewallet",
    publicKey,

    signMessage: async ({ message }) => {
      const { signature } = await sign<"personal_sign">({
        type: "personal_sign",
        data: {
          address,
          message,
        },
      });

      return signature;
    },

    signTransaction: async (transaction) => {
      const signableTransaction = (() => {
        // For EIP-4844 Transactions, we want to sign the transaction payload body (tx_payload_body) without the sidecars (ie. without the network wrapper).
        // See: https://github.com/ethereum/EIPs/blob/e00f4daa66bd56e2dbd5f1d36d09fd613811a48b/EIPS/eip-4844.md#networking
        if (transaction.type === "eip4844")
          return {
            ...transaction,
            sidecars: false,
          };
        return transaction;
      })();

      const { signedTransaction } = await sign<"sign_transaction">({
        type: "sign_transaction",
        data: {
          address,
          transaction: signableTransaction,
        },
      });

      return signedTransaction;
    },

    signTypedData: async (typedData) => {
      const { signature } = await sign<"sign_typedData_v4">({
        type: "sign_typedData_v4",
        data: {
          address,
          message: typedData as TypedDataDefinition,
        },
      });

      return signature;
    },
  };

  return account satisfies EWalletAccount<"ewallet", Address>;
}

/**
 * Create an Ethereum local signer for testing purposes
 * TODO: remove this function after testing
 * @param privateKey - The private key to use for signing
 * @returns An Ethereum signer that can sign transactions, personal messages, and typed data
 * @dev signHash function is not part of the EthSigner interface, but is added for testing purposes
 */
export const createEthLocalSigner = (
  privateKey: Hex,
): EthSigner & { signHash: ({ hash }: { hash: Hex }) => Promise<Hex> } => {
  const account = privateKeyToAccount(privateKey);
  return {
    address: account.address,
    sign: async function <M extends EthSignMethod>(
      parameters: SignFunctionParams<M>,
    ): Promise<SignFunctionResult<M>> {
      switch (parameters.type) {
        case "sign_transaction": {
          const { transaction } = parameters.data;
          const serializedTx = serializeTransaction(transaction);
          const hash = keccak256(serializedTx);
          const signature = await account.sign({ hash });
          const signedTransaction = serializeTransaction(
            transaction,
            parseSignature(signature),
          );
          return {
            type: "signed_transaction",
            signedTransaction,
          };
        }
        case "personal_sign": {
          const { message } = parameters.data;
          const hash = hashMessage(message);
          const signature = await account.sign({ hash });
          return {
            type: "signature",
            signature,
          };
        }
        case "sign_typedData_v4": {
          const { message } = parameters.data;
          const hash = hashTypedData(message);
          const signature = await account.sign({ hash });
          return {
            type: "signature",
            signature,
          };
        }
        default:
          throw new Error(`Unknown sign type: ${(parameters as any).type}`);
      }
    },
    signHash: async ({ hash }: { hash: Hex }): Promise<Hex> => {
      return await account.sign({ hash });
    },
  };
};
