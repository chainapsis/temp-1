import type {
  PublicRpcMethod,
  UnsupportedPublicRpcMethod,
  UnsupportedWalletRpcMethod,
  WalletRpcMethod,
} from "./types";

export const PUBLIC_RPC_METHODS: ReadonlySet<PublicRpcMethod> = new Set([
  "web3_clientVersion",
  "eth_blobBaseFee",
  "eth_blockNumber",
  "eth_chainId",
  "eth_call",
  "eth_coinbase",
  "eth_feeHistory",
  "eth_estimateGas",
  "eth_gasPrice",
  "eth_getBalance",
  "eth_getBlockByHash",
  "eth_getBlockByNumber",
  "eth_getBlockTransactionCountByHash",
  "eth_getBlockTransactionCountByNumber",
  "eth_getCode",
  "eth_getProof",
  "eth_getFilterChanges",
  "eth_getFilterLogs",
  "eth_getLogs",
  "eth_newBlockFilter",
  "eth_newFilter",
  "eth_newPendingTransactionFilter",
  "eth_uninstallFilter",
  "eth_getStorageAt",
  "eth_getTransactionByBlockHashAndIndex",
  "eth_getTransactionByBlockNumberAndIndex",
  "eth_getTransactionByHash",
  "eth_getTransactionCount",
  "eth_getTransactionReceipt",
  "eth_getUncleByBlockHashAndIndex",
  "eth_getUncleByBlockNumberAndIndex",
  "eth_getUncleCountByBlockHash",
  "eth_getUncleCountByBlockNumber",
  "eth_maxPriorityFeePerGas",
  "eth_protocolVersion",
  "eth_sendRawTransaction",
  "eth_syncing",
]);

export const WALLET_RPC_METHODS: ReadonlySet<WalletRpcMethod> = new Set([
  "eth_accounts",
  "eth_requestAccounts",
  "eth_sendTransaction",
  "eth_signTransaction",
  "eth_signTypedData_v4",
  "personal_sign",
  "wallet_addEthereumChain",
  "wallet_switchEthereumChain",
]);

export const UNSUPPORTED_PUBLIC_RPC_METHODS: ReadonlySet<UnsupportedPublicRpcMethod> =
  new Set([
    "eth_subscribe",
    "eth_unsubscribe",
    "eth_simulateV1",
    "eth_createAccessList",
  ]);

// EIP-7846, EIP-5792, EIP-7715, EIP-2255
export const UNSUPPORTED_WALLET_RPC_METHODS: ReadonlySet<UnsupportedWalletRpcMethod> =
  new Set([
    // https://docs.metamask.io/wallet/concepts/signing-methods/#eth_sign
    "eth_sign",
    "wallet_sendTransaction",
    "eth_signTypedData_v3",
    "wallet_getCallsStatus",
    "wallet_getCapabilities",
    "wallet_getPermissions",
    "wallet_grantPermissions",
    "wallet_requestPermissions",
    "wallet_revokePermissions",
    "wallet_sendCalls",
    "wallet_showCallsStatus",
    "wallet_addSubAccount",
    "wallet_connect",
    "wallet_disconnect",
    "wallet_watchAsset",
  ]);

export const UNSUPPORTED_RPC_METHODS: ReadonlySet<UnsupportedWalletRpcMethod> =
  new Set([
    ...UNSUPPORTED_PUBLIC_RPC_METHODS,
    ...UNSUPPORTED_WALLET_RPC_METHODS,
  ]);
