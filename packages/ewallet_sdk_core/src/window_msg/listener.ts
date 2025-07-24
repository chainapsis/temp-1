import type { EWalletMsg } from "@keplr-ewallet-sdk-core/types";

// Only used for "init" message which is triggered by the child.
// After initialization, message communication is only triggered
// by parent window and child replies on the dedicated channel
export function registerMsgListener() {
  return new Promise((resolve) => {
    async function msgHandler(event: MessageEvent) {
      const message = event.data as EWalletMsg;

      switch (message.msg_type) {
        case "init": {
          resolve(1);
        }
      }
    }

    window.addEventListener("message", msgHandler);
    console.log("sdk core event listener registered");
  });
}
