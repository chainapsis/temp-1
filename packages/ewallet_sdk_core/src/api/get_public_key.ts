import { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";

export async function getPublicKey(this: KeplrEWallet) {
  try {
    const res = await this.sendMsgToIframe({
      msg_type: "get_public_key",
      payload: null,
    });

    return res.payload ?? null;
  } catch (error) {
    console.error("[core] getPublicKey failed with error:", error);
    return null;
  }
}
