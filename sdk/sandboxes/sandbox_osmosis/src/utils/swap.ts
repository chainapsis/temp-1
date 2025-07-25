import type { CosmosEWallet } from "@keplr-ewallet/ewallet-sdk-cosmos";
import { SigningStargateClient } from "@cosmjs/stargate";
import { coins } from "@cosmjs/amino";
import type { OfflineDirectSigner } from "@cosmjs/proto-signing";
import type { Key } from "@keplr-wallet/types";

export interface SwapParams {
  fromToken: string;
  toToken: string;
  amount: string;
  slippage: number;
}

export interface Pool {
  id: string;
  token1: string;
  token2: string;
  token1Reserve: string;
  token2Reserve: string;
}

// Osmosis pool information (should be fetched from chain in production)
export const OSMOSIS_POOLS: Pool[] = [
  {
    id: "1",
    token1: "uosmo",
    token2:
      "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2", // ATOM
    token1Reserve: "1000000000000", // 1M OSMO
    token2Reserve: "100000000000", // 100K ATOM
  },
  {
    id: "678",
    token1: "uosmo",
    token2:
      "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858", // USDC
    token1Reserve: "2000000000000", // 2M OSMO
    token2Reserve: "1000000000000", // 1M USDC
  },
];

// Token information
export const TOKENS = {
  OSMO: {
    denom: "uosmo",
    symbol: "OSMO",
    decimals: 6,
    icon: "🧬",
  },
  ATOM: {
    denom:
      "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
    symbol: "ATOM",
    decimals: 6,
    icon: "⚛️",
  },
  USDC: {
    denom:
      "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858",
    symbol: "USDC",
    decimals: 6,
    icon: "💵",
  },
};

// Simple constant product formula for swap calculation
export function calculateSwapAmount(
  inputAmount: string,
  inputReserve: string,
  outputReserve: string,
  slippage: number,
): { outputAmount: string; priceImpact: number } {
  const input = BigInt(inputAmount);
  const reserveIn = BigInt(inputReserve);
  const reserveOut = BigInt(outputReserve);

  // Constant product formula: (reserveIn + input) * (reserveOut - output) = reserveIn * reserveOut
  // output = (input * reserveOut) / (reserveIn + input)
  const output = (input * reserveOut) / (reserveIn + input);

  // Apply slippage
  const slippageMultiplier = BigInt(Math.floor((100 - slippage) * 100));
  const outputWithSlippage = (output * slippageMultiplier) / BigInt(10000);

  // Calculate price impact
  const priceImpact =
    (Number(output - outputWithSlippage) / Number(output)) * 100;

  return {
    outputAmount: outputWithSlippage.toString(),
    priceImpact,
  };
}

// Create simple bank send message for testing (since Osmosis messages aren't registered)
export function createSwapMessage(
  sender: string,
  poolId: string,
  tokenIn: { denom: string; amount: string },
  tokenOutMinAmount: string,
  tokenOutDenom: string,
) {
  // For now, return a bank send message which is guaranteed to be registered
  // This is just for testing the signing pipeline
  return {
    typeUrl: "/cosmos.bank.v1beta1.MsgSend",
    value: {
      fromAddress: sender,
      toAddress: sender, // Send to self for testing
      amount: [tokenIn],
    },
  };
}

// Execute swap using CosmosEWallet
export async function executeSwapWithEWallet(
  swapParams: SwapParams,
  cosmosWallet: CosmosEWallet,
  osmosisAccount: Key,
): Promise<string> {
  const { fromToken, toToken, amount, slippage } = swapParams;

  // Find pool
  const pool = OSMOSIS_POOLS.find(
    (p) =>
      (p.token1 === fromToken && p.token2 === toToken) ||
      (p.token1 === toToken && p.token2 === fromToken),
  );

  if (!pool) {
    throw new Error("Pool not found for the token pair");
  }

  // Calculate swap
  const isToken1ToToken2 = pool.token1 === fromToken;
  const inputReserve = isToken1ToToken2
    ? pool.token1Reserve
    : pool.token2Reserve;
  const outputReserve = isToken1ToToken2
    ? pool.token2Reserve
    : pool.token1Reserve;
  const tokenOutDenom = isToken1ToToken2 ? pool.token2 : pool.token1;

  const { outputAmount } = calculateSwapAmount(
    amount,
    inputReserve,
    outputReserve,
    slippage,
  );

  console.log("Swap Parameters:", {
    fromToken,
    toToken,
    amount,
    outputAmount,
    slippage,
    poolId: pool.id,
    tokenOutDenom,
  });

  try {
    // Send transaction using CosmosEWallet
    const message = createSwapMessage(
      osmosisAccount.bech32Address,
      pool.id,
      { denom: fromToken, amount },
      outputAmount,
      tokenOutDenom,
    );

    console.log("Swap Message:", message);

    // Actual transaction sending logic
    console.log("Attempting to send transaction using eWallet");

    try {
      // 1. Get signer from eWallet
      const signer = cosmosWallet.getOfflineSigner("osmosis-1");

      // 2. Osmosis RPC endpoint
      const rpcEndpoint = "https://rpc.osmosis.zone";

      // 3. Create SigningStargateClient (try without custom registry first)
      const client = await SigningStargateClient.connectWithSigner(
        rpcEndpoint,
        signer,
      );

      // 4. Configure transaction message
      const fee = {
        amount: coins(5000, "uosmo"),
        gas: "200000",
      };

      // 5. Send transaction
      const result = await client.signAndBroadcast(
        osmosisAccount.bech32Address,
        [message],
        fee,
        "Swap tokens via eWallet",
      );

      console.log("Transaction result:", result);

      if (result.code === 0) {
        console.log("Transaction successful:", result.transactionHash);
        return result.transactionHash;
      } else {
        throw new Error(`Transaction failed: ${result.rawLog}`);
      }
    } catch (txError) {
      console.error("Transaction broadcast failed:", txError);
      throw new Error(
        `Transaction failed: ${txError instanceof Error ? txError.message : "Unknown error"}`,
      );
    }
  } catch (error) {
    console.error("Swap transaction failed:", error);
    throw error;
  }
}

// Execute swap using eWallet signer (more efficient method)
export async function executeSwapWithEWalletSigner(
  swapParams: SwapParams,
  ewalletSigner: OfflineDirectSigner,
  osmosisAccount: Key,
): Promise<string> {
  const { fromToken, toToken, amount, slippage } = swapParams;

  // Find pool
  const pool = OSMOSIS_POOLS.find(
    (p) =>
      (p.token1 === fromToken && p.token2 === toToken) ||
      (p.token1 === toToken && p.token2 === fromToken),
  );

  if (!pool) {
    throw new Error("Pool not found for the token pair");
  }

  // Calculate swap
  const isToken1ToToken2 = pool.token1 === fromToken;
  const inputReserve = isToken1ToToken2
    ? pool.token1Reserve
    : pool.token2Reserve;
  const outputReserve = isToken1ToToken2
    ? pool.token2Reserve
    : pool.token1Reserve;
  const tokenOutDenom = isToken1ToToken2 ? pool.token2 : pool.token1;

  const { outputAmount } = calculateSwapAmount(
    amount,
    inputReserve,
    outputReserve,
    slippage,
  );

  console.log("eWallet Signer Swap Parameters:", {
    fromToken,
    toToken,
    amount,
    outputAmount,
    slippage,
    poolId: pool.id,
    tokenOutDenom,
  });

  try {
    // Send transaction using eWallet signer
    const message = createSwapMessage(
      osmosisAccount.bech32Address,
      pool.id,
      { denom: fromToken, amount },
      outputAmount,
      tokenOutDenom,
    );

    console.log("Swap Message:", message);

    // Actual transaction sending logic
    console.log("Attempting to send transaction using eWallet signer");

    try {
      // 1. Osmosis RPC endpoint
      const rpcEndpoint = "https://rpc.osmosis.zone";

      // 2. Create SigningStargateClient with eWallet signer (try without custom registry first)
      const client = await SigningStargateClient.connectWithSigner(
        rpcEndpoint,
        ewalletSigner,
      );

      // 3. Configure transaction message
      const fee = {
        amount: coins(5000, "uosmo"),
        gas: "200000",
      };

      // 4. Send transaction
      const result = await client.signAndBroadcast(
        osmosisAccount.bech32Address,
        [message],
        fee,
        "Swap tokens via eWallet signer",
      );

      console.log("Transaction result:", result);

      if (result.code === 0) {
        console.log("Transaction successful:", result.transactionHash);
        return result.transactionHash;
      } else {
        throw new Error(
          `Transaction failed: ${result.rawLog || "Unknown error"}`,
        );
      }
    } catch (txError) {
      console.error("Transaction broadcast failed:", txError);
      throw new Error(
        `Transaction failed: ${txError instanceof Error ? txError.message : "Unknown error"}`,
      );
    }
  } catch (error) {
    console.error("Swap transaction failed:", error);
    throw error;
  }
}

// Execute swap using Keplr wallet
export async function executeSwap(
  swapParams: SwapParams,
  walletAddress: string,
): Promise<string> {
  const { fromToken, toToken, amount, slippage } = swapParams;

  // Find pool
  const pool = OSMOSIS_POOLS.find(
    (p) =>
      (p.token1 === fromToken && p.token2 === toToken) ||
      (p.token1 === toToken && p.token2 === fromToken),
  );

  if (!pool) {
    throw new Error("Pool not found for the token pair");
  }

  // Calculate swap
  const isToken1ToToken2 = pool.token1 === fromToken;
  const inputReserve = isToken1ToToken2
    ? pool.token1Reserve
    : pool.token2Reserve;
  const outputReserve = isToken1ToToken2
    ? pool.token2Reserve
    : pool.token1Reserve;

  const { outputAmount } = calculateSwapAmount(
    amount,
    inputReserve,
    outputReserve,
    slippage,
  );

  console.log("Keplr Swap Parameters:", {
    fromToken,
    toToken,
    amount,
    outputAmount,
    slippage,
    walletAddress,
    poolId: pool.id,
  });

  // Send transaction using Keplr
  // In actual implementation, use Keplr's offline signer to send transaction
  throw new Error(
    "Keplr wallet integration not yet implemented. Please use eWallet for now.",
  );
}
