import type { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";
import type { EWalletMsg } from "@keplr-ewallet-sdk-core/types";

export function sendMsgToIframe(this: KeplrEWallet, msg: EWalletMsg) {
  return new Promise<EWalletMsg>((resolve, reject) => {
    if (this.iframe.contentWindow === null) {
      reject("iframe contentWindow is null");
      return;
    }

    const contentWindow = this.iframe.contentWindow;

    const channel = new MessageChannel();

    channel.port1.onmessage = (obj: any) => {
      const data = obj.data as EWalletMsg;

      console.log("data", data);

      if (data.hasOwnProperty("payload")) {
        resolve(data);
      } else {
        resolve({
          msg_type: "unknown",
          payload: JSON.stringify(data),
        });
      }
    };

    contentWindow.postMessage(msg, this.attachedEndpoint, [channel.port2]);
  });
}
