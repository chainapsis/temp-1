import type { KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";
import { serializeTransaction, serializeTypedData } from "viem";

import type {
  EthSignMethod,
  SignFunction,
  SignFunctionParams,
  SignFunctionResult,
} from "@keplr-ewallet-sdk-eth/types";

export function makeSign(eWallet: KeplrEWallet): SignFunction {
  return async function sign<M extends EthSignMethod>(
    parameters: SignFunctionParams<M>,
  ): Promise<SignFunctionResult<M>> {
    switch (parameters.type) {
      // case "sign_transaction":
      //   const ret = await eWallet.sendMsgToIframe({
      //     msg_type: "make_signature",
      //     payload: {
      //       msg: new Uint8Array(),
      //       // type: parameters.type) as any,
      //       // chainType: "ethereum",
      //       // data: {
      //       //   address: parameters.data.address,
      //       //   serializedTransaction: serializeTransaction(
      //       //     parameters.data.transaction,
      //       //   ),
      //       // },
      //     },
      //   });
      //
      //   if (ret.msg_type !== "make_signature_ack" || ret.payload === null) {
      //     throw new Error("Invalid response from ewallet");
      //   }
      //
      //   // if (ret.payload.type !== "signed_transaction") {
      //   //   throw new Error("Invalid response from ewallet");
      //   // }
      //
      //   return {
      //     type: "signed_transaction",
      //     signedTransaction: ret.payload.signedTransaction,
      //   };
      //
      // case "personal_sign": {
      //   const ret = await eWallet.sendMsgToIframe({
      //     msg_type: "make_signature",
      //     payload: {
      //       type: parameters.type,
      //       chainType: "ethereum",
      //       data: {
      //         address: parameters.data.address,
      //         rawMessage: parameters.data.message,
      //       },
      //     },
      //   });
      //
      //   if (ret.msg_type !== "make_signature_ack" || ret.payload === null) {
      //     throw new Error("Invalid response from ewallet");
      //   }
      //
      //   if (
      //     ret.payload.chainType !== "ethereum" ||
      //     ret.payload.type !== "signature"
      //   ) {
      //     throw new Error("Invalid response from ewallet");
      //   }
      //
      //   return {
      //     type: "signature",
      //     signature: ret.payload.signature,
      //   };
      // }
      // case "sign_typedData_v4": {
      //   const ret = await eWallet.sendMsgToIframe({
      //     msg_type: "make_signature",
      //     payload: {
      //       type: parameters.type,
      //       chainType: "ethereum",
      //       data: {
      //         address: parameters.data.address,
      //         encodedTypedData: serializeTypedData(parameters.data.message),
      //       },
      //     },
      //   });
      //
      //   if (ret.msg_type !== "make_signature_ack" || ret.payload === null) {
      //     throw new Error("Invalid response from ewallet");
      //   }
      //
      //   if (
      //     ret.payload.chainType !== "ethereum" ||
      //     ret.payload.type !== "signature"
      //   ) {
      //     throw new Error("Invalid response from ewallet");
      //   }
      //
      //   return {
      //     type: "signature",
      //     signature: ret.payload.signature,
      //   };
      // }
      default: {
        throw new Error(`Unknown sign method: ${(parameters as any).type}`);
      }
    }
  };
}
