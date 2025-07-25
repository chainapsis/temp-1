import { KeplrEWallet } from "@keplr-ewallet-sdk-core/keplr_ewallet";
import { RedirectUriSearchParamsKey } from "@keplr-ewallet-sdk-core/oauth";

const GoogleClientId =
  "239646646986-8on7ql1vmbcshbjk12bdtopmto99iipm.apps.googleusercontent.com";

// TODO:should be given programatically
const IframeOrigin = "http://localhost:3201/";

export async function tryGoogleSignIn(
  customerId: string,
  sendMsgToIframe: KeplrEWallet["sendMsgToIframe"],
) {
  const clientId = GoogleClientId;
  const redirectUri = `${new URL(IframeOrigin).origin}/google/callback`;

  console.log("window host: %s", window.location.host);
  console.log("redirectUri: %s", redirectUri);

  const setCustomerIdAckPromise = sendMsgToIframe({
    msg_type: "set_customer_id",
    payload: customerId,
  });

  const nonce = Array.from(crypto.getRandomValues(new Uint8Array(8)))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");

  // NOTE: safari browser block the new window when async operation is used
  // between user interaction and window opening.
  // so we need to send the message to iframe first and wait for the ack after window opening.
  const ackPromise = sendMsgToIframe({
    msg_type: "set_oauth_nonce",
    payload: nonce,
  });

  const authUrl = new URL("https://accounts.google.com/o/oauth2/v2/auth");
  authUrl.searchParams.set("client_id", clientId);
  authUrl.searchParams.set("redirect_uri", redirectUri);

  // Google implicit auth flow
  // See https://developers.google.com/identity/protocols/oauth2/javascript-implicit-flow
  authUrl.searchParams.set("response_type", "token id_token");

  authUrl.searchParams.set("scope", "openid email profile");
  authUrl.searchParams.set("prompt", "login");
  authUrl.searchParams.set("nonce", nonce);
  authUrl.searchParams.set(
    RedirectUriSearchParamsKey.STATE,
    window.location.origin,
  );

  const popup = window.open(
    authUrl.toString(),
    "google_oauth",
    "width=1200,height=800",
  );

  if (!popup) {
    throw new Error("Failed to open new window for google oauth sign in");
  }

  // TODO: set customer id on init and remove this
  const setCustomerIdAck = await setCustomerIdAckPromise;
  if (!setCustomerIdAck.payload) {
    popup.close();
    throw new Error("Failed to set customer id for google oauth sign in");
  }

  const ack = await ackPromise;
  if (ack.msg_type !== "set_oauth_nonce_ack" || ack.payload !== "ok") {
    popup.close();
    throw new Error("Failed to set nonce for google oauth sign in");
  }

  return new Promise<void>((resolve, reject) => {
    const interval = setInterval(() => {
      if (popup.closed) {
        clearInterval(interval);

        reject(new Error("Window closed by user"));
      }
    }, 1000);

    const handler = (e: MessageEvent) => {
      if (e.data && e.data.msg_type === "oauth_sign_in_ack") {
        window.removeEventListener("message", handler);
        clearInterval(interval);

        resolve();
      }
    };

    window.addEventListener("message", handler);

    setTimeout(
      () => {
        window.removeEventListener("message", handler);
        clearInterval(interval);
        popup.close();

        reject(
          new Error(
            "User is not responding to the sign in request for more than 5 minutes",
          ),
        );
      },
      5 * 60 * 1000,
    );
  });
}
