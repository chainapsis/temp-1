import type { KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";
import type { Hex } from "viem";

export function makeGetPublicKey(eWallet: KeplrEWallet): () => Promise<Hex> {
  return async function getPublicKey(): Promise<Hex> {
    const ret = await eWallet.sendMsgToIframe({
      msg_type: "get_public_key",
      payload: null,
    });

    if (ret.payload === null) {
      throw new Error("Invalid response from ewallet");
    }

    const publicKey = ret.payload as string;

    return `0x${publicKey}`;
  };
}
