import { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";

export async function getEmail(this: KeplrEWallet) {
  try {
    const res = await this.sendMsgToIframe({
      msg_type: "get_email",
      payload: null,
    });

    return res.payload ?? null;
  } catch (error) {
    console.error("[core] getEmail failed with error:", error);
    return null;
  }
}
