import { sha256 } from "@noble/hashes/sha2";
import type { StdSignature, StdSignDoc } from "@cosmjs/amino";
import { serializeSignDoc } from "@cosmjs/amino";

import { CosmosEWallet } from "@keplr-ewallet-sdk-cosmos/cosmos_ewallet";
import { encodeCosmosSignature } from "@keplr-ewallet-sdk-cosmos/utils/sign";
import { makeADR36AminoSignDoc } from "@keplr-ewallet-sdk-cosmos/utils/arbitrary";

export async function signArbitrary(
  this: CosmosEWallet,
  chainId: string,
  signer: string,
  data: string | Uint8Array,
): Promise<StdSignature> {
  try {
    // Create ADR-36 sign doc for arbitrary message signing
    const signDoc = makeADR36AminoSignDoc(signer, data);
    const signDocHash = sha256(serializeSignDoc(signDoc));

    const publicKey = await this.getPublicKey();

    const res = await this.eWallet.sendMsgToIframe({
      msg_type: "make_signature",
      payload: {
        msg: signDocHash,
      },
    });

    console.log("[signArbitrary cosmos] [res] @@@@@", res);

    const signature = encodeCosmosSignature(res.payload.sign_output, publicKey);
    return signature;
  } catch (error) {
    console.error("[signArbitrary cosmos] [error] @@@@@", error);
    throw error;
  }
}
