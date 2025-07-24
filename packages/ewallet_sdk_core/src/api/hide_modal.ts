import { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";

export async function hideModal(this: KeplrEWallet) {
  this.iframe.style.display = "none";
}
