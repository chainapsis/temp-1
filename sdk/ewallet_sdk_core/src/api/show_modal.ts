import { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";
import type { EWalletMsg } from "@keplr-ewallet-sdk-core/types";

export async function showModal(this: KeplrEWallet, msg: EWalletMsg) {
  this.iframe.style.display = "block";

  const showModalAck = await this.sendMsgToIframe(msg);
  console.log("[showModal] [showModalAck] @@@@@", showModalAck);

  return showModalAck;
}
