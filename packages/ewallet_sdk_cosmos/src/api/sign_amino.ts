import {
  serializeSignDoc,
  type AminoSignResponse,
  type StdSignDoc,
} from "@cosmjs/amino";
import { sha256 } from "@noble/hashes/sha2";
import type { KeplrSignOptions } from "@keplr-wallet/types";

import { CosmosEWallet } from "@keplr-ewallet-sdk-cosmos/cosmos_ewallet";
import { encodeCosmosSignature } from "@keplr-ewallet-sdk-cosmos/utils/sign";

export async function signAmino(
  this: CosmosEWallet,
  chainId: string,
  signer: string,
  signDoc: StdSignDoc,
  signOptions?: KeplrSignOptions,
): Promise<AminoSignResponse> {
  try {
    const signDocHash = sha256(serializeSignDoc(signDoc));
    const publicKey = await this.getPublicKey();

    const res = await this.eWallet.sendMsgToIframe({
      msg_type: "make_signature",
      payload: {
        msg: signDocHash,
      },
    });

    console.log("[signAmino cosmos] [res] @@@@@", res);

    const signature = encodeCosmosSignature(res.payload.sign_output, publicKey);
    return {
      signed: signDoc,
      signature,
    };
  } catch (error) {
    console.error("[signAmino cosmos] [error] @@@@@", error);
    throw error;
  }
}
