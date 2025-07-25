import type {
  AddEthereumChainParameter as Chain,
  RpcTransactionRequest,
  TransactionSerializable,
} from "viem";

export const isValidChainId = (chainId: unknown): chainId is string =>
  Boolean(chainId) && typeof chainId === "string" && chainId.startsWith("0x");

/**
 * Check if a URL string is valid
 */
export const isValidUrl = (url: string): boolean => {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
};

/**
 * Validate chain ID format and value
 * @param chainId - The chain ID to validate
 * @returns Object with validation result and decimal value
 */
export const validateChainIdFormat = (
  chainId: string,
): {
  isValid: boolean;
  decimalValue?: number;
  error?: string;
} => {
  try {
    const decimalChainId = parseInt(chainId, 16);

    // Ensure the chain ID is a 0x-prefixed hexadecimal string and can be parsed to an integer
    if (!/^0x[0-9a-fA-F]+$/.test(chainId) || isNaN(decimalChainId)) {
      return { isValid: false, error: "Invalid chain ID format" };
    }

    // Validate chain ID value is not greater than max safe integer value
    if (decimalChainId > Number.MAX_SAFE_INTEGER) {
      return {
        isValid: false,
        error: "Chain ID value exceeds maximum safe integer",
      };
    }

    return { isValid: true, decimalValue: decimalChainId };
  } catch (error) {
    return { isValid: false, error: "Invalid chain ID format" };
  }
};

/**
 * Validate RPC URLs array
 * @param rpcUrls - Array of RPC URLs to validate
 * @returns Validation result
 */
export const validateRpcUrls = (
  rpcUrls: readonly string[],
): {
  isValid: boolean;
  error?: string;
} => {
  if (!rpcUrls || rpcUrls.length === 0) {
    return { isValid: false, error: "RPC URLs are required" };
  }

  for (const url of rpcUrls) {
    if (!isValidUrl(url)) {
      return { isValid: false, error: `Invalid RPC URL: ${url}` };
    }
  }

  return { isValid: true };
};

/**
 * Validate block explorer URLs array
 * @param blockExplorerUrls - Array of block explorer URLs to validate
 * @returns Validation result
 */
export const validateBlockExplorerUrls = (
  blockExplorerUrls?: readonly string[],
): {
  isValid: boolean;
  error?: string;
} => {
  if (!blockExplorerUrls) {
    return { isValid: true }; // Optional field
  }

  if (!Array.isArray(blockExplorerUrls) || blockExplorerUrls.length === 0) {
    return {
      isValid: false,
      error: "Block explorer URLs must be a non-empty array",
    };
  }

  for (const url of blockExplorerUrls) {
    if (!isValidUrl(url)) {
      return { isValid: false, error: `Invalid block explorer URL: ${url}` };
    }
  }

  return { isValid: true };
};

/**
 * Validate native currency symbol
 * @param symbol - The currency symbol to validate
 * @returns Validation result
 */
export const validateNativeCurrencySymbol = (
  symbol: string,
): {
  isValid: boolean;
  error?: string;
} => {
  if (symbol.length < 2 || symbol.length > 6) {
    return {
      isValid: false,
      error: "Native currency symbol must be between 2-6 characters",
    };
  }

  return { isValid: true };
};

/**
 * Validate complete chain information
 * @param chain - The chain to validate
 * @param addedChains - Existing chains to check for duplicates
 * @returns Validation result with detailed error information
 */
export const validateChain = (
  chain: Chain,
  addedChains: readonly Chain[],
): {
  isValid: boolean;
  error?: string;
  errorType?:
  | "DUPLICATE_CHAIN"
  | "INVALID_CHAIN_ID"
  | "INVALID_RPC_URLS"
  | "INVALID_BLOCK_EXPLORER_URLS"
  | "INVALID_NATIVE_CURRENCY"
  | "CURRENCY_SYMBOL_MISMATCH";
  errorData?: any;
} => {
  const { rpcUrls, blockExplorerUrls, chainId, nativeCurrency } = chain;

  // Check if chain already exists
  if (addedChains.some((c) => c.chainId === chainId)) {
    return {
      isValid: false,
      error: "Chain already added",
      errorType: "DUPLICATE_CHAIN",
      errorData: { chainId },
    };
  }

  // Validate chain ID format and value
  const chainIdResult = validateChainIdFormat(chainId);
  if (!chainIdResult.isValid) {
    return {
      isValid: false,
      error: chainIdResult.error,
      errorType: "INVALID_CHAIN_ID",
      errorData: { chainId },
    };
  }

  // Validate RPC URLs
  const rpcResult = validateRpcUrls(rpcUrls);
  if (!rpcResult.isValid) {
    return {
      isValid: false,
      error: rpcResult.error,
      errorType: "INVALID_RPC_URLS",
      errorData: { rpcUrls },
    };
  }

  // Validate block explorer URLs
  const blockExplorerResult = validateBlockExplorerUrls(blockExplorerUrls);
  if (!blockExplorerResult.isValid) {
    return {
      isValid: false,
      error: blockExplorerResult.error,
      errorType: "INVALID_BLOCK_EXPLORER_URLS",
      errorData: { blockExplorerUrls },
    };
  }

  // Validate native currency
  if (nativeCurrency) {
    const symbolResult = validateNativeCurrencySymbol(nativeCurrency.symbol);
    if (!symbolResult.isValid) {
      return {
        isValid: false,
        error: symbolResult.error,
        errorType: "INVALID_NATIVE_CURRENCY",
        errorData: { symbol: nativeCurrency.symbol },
      };
    }

    // Check for native currency symbol mismatch with existing chain
    const existingChain = addedChains.find((c) => c.chainId === chainId);
    if (
      existingChain &&
      existingChain.nativeCurrency &&
      existingChain.nativeCurrency.symbol !== nativeCurrency.symbol
    ) {
      return {
        isValid: false,
        error: "Native currency symbol mismatch with existing chain",
        errorType: "CURRENCY_SYMBOL_MISMATCH",
        errorData: {
          chainId,
          existing: existingChain.nativeCurrency.symbol,
          provided: nativeCurrency.symbol,
        },
      };
    }
  }

  return { isValid: true };
};

export const toSerializable = ({
  chainId,
  tx,
}: {
  chainId: string;
  tx: RpcTransactionRequest;
}): TransactionSerializable => {
  const convertValue = <T>(
    value: string | number | undefined,
    converter: (value: string | number) => T,
    defaultValue?: T,
  ): T | undefined => (value !== undefined ? converter(value) : defaultValue);

  const { from, ...transaction } = tx;
  const txType = transaction.type || "0x2"; // Default to EIP-1559

  const baseFields = {
    chainId: parseInt(chainId, 16),
    to: transaction.to || null,
    data: transaction.data,
    gas: convertValue(transaction.gas, BigInt),
    nonce: convertValue(transaction.nonce, (value) =>
      parseInt(value.toString(), 16),
    ),
    value: convertValue(transaction.value, BigInt) || BigInt(0),
  };

  const typeMapping: { [key: string]: "legacy" | "eip2930" | "eip1559" } = {
    "0x0": "legacy",
    "0x1": "eip2930",
    "0x2": "eip1559",
  };

  const mappedType = typeMapping[txType] || "eip1559";

  let transactionSerializable: TransactionSerializable;

  switch (mappedType) {
    case "legacy":
      transactionSerializable = {
        ...baseFields,
        type: "legacy",
        gasPrice: convertValue(transaction.gasPrice, BigInt),
      };
      break;
    case "eip2930":
      transactionSerializable = {
        ...baseFields,
        type: "eip2930",
        gasPrice: convertValue(transaction.gasPrice, BigInt),
        accessList: transaction.accessList || [],
      };
      break;
    case "eip1559":
    default:
      transactionSerializable = {
        ...baseFields,
        type: "eip1559",
        maxPriorityFeePerGas: convertValue(
          transaction.maxPriorityFeePerGas,
          BigInt,
        ),
        maxFeePerGas: convertValue(transaction.maxFeePerGas, BigInt),
      };
      break;
  }

  return transactionSerializable;
};

/**
 * Parse typed data from a string, and convert string to bigint
 * @param text - The string to parse
 * @param reviver - A function to transform the parsed value
 * @returns The parsed typed data
 */
export const parseTypedData = <T>(
  text: string,
  reviver?: (this: any, key: string, value: any) => any,
): T => {
  return JSON.parse(text, (key, val) => {
    // First apply user-provided reviver, if any
    const revived =
      typeof reviver === "function" ? reviver.call(this, key, val) : val;
    // Then detect our explicit bigint tag on the revived value
    if (
      revived !== null &&
      typeof revived === "object" &&
      (revived as any).__type === "bigint" &&
      typeof (revived as any).value === "string"
    ) {
      return BigInt((revived as any).value);
    }
    // Otherwise return the revived or original value
    return revived;
  });
};
