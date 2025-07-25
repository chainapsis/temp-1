import type { EWalletMsg, KeplrEWallet } from "@keplr-ewallet/ewallet-sdk-core";
import { serializeSignature, serializeTransaction } from "viem";

import type {
  EthSignMethod,
  SignFunction,
  SignFunctionParams,
  SignFunctionResult,
} from "@keplr-ewallet-sdk-eth/types";
import {
  hashEthereumMessage,
  hashEthereumTransaction,
  hashEthereumTypedData,
} from "@keplr-ewallet-sdk-eth/hash";
import { encodeEthereumSignature } from "@keplr-ewallet-sdk-eth/encode";

export function makeSign(eWallet: KeplrEWallet): SignFunction {
  return async function sign<M extends EthSignMethod>(
    parameters: SignFunctionParams<M>,
  ): Promise<SignFunctionResult<M>> {
    switch (parameters.type) {
      case "sign_transaction": {
        const origin = eWallet.origin;

        // TODO: simulate the tx and get the estimated fee

        const msg: EWalletMsg = {
          msg_type: "show_modal",
          payload: {
            modal_type: "make_signature",
            is_demo: true,
            data: {
              chain_type: "eth",
              sign_type: "tx",
              payload: {
                // TODO: get chain info from ewallet or
                // there needs to be a public client on the attached side
                // for chain info management and tx simulation
                chain_info: {
                  chain_id: "1",
                  chain_name: "ethereum",
                  chain_symbol_image_url:
                    "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/eip155:1/chain.png",
                },
                signer: parameters.data.address,
                data: parameters.data.transaction,
                origin,
              },
            },
          },
        };

        const openModalAck = await eWallet.showModal(msg);
        if (openModalAck.msg_type === "show_modal_ack") {
          await eWallet.hideModal();

          if (openModalAck.payload === "approve") {
            const msgHash = hashEthereumTransaction(
              parameters.data.transaction,
            );

            // TODO: receive the simulated tx from the attached side (this is not required for the MVP)
            // we cannot estimate the fee here and just pass it to the attached side
            const res = await eWallet.sendMsgToIframe({
              msg_type: "make_signature",
              payload: {
                msg: msgHash,
              },
            });
            const signature = encodeEthereumSignature(res.payload.sign_output);

            const signedTransaction = serializeTransaction(
              parameters.data.transaction,
              signature,
            );

            return {
              type: "signed_transaction",
              signedTransaction,
            };
          }

          if (openModalAck.payload === "reject") {
            throw new Error("User rejected the signature request");
          }
        }

        throw new Error("Unreachable");
      }
      case "personal_sign": {
        const origin = eWallet.origin;

        const msg: EWalletMsg = {
          msg_type: "show_modal",
          payload: {
            modal_type: "make_signature",
            is_demo: true,
            data: {
              chain_type: "eth",
              sign_type: "arbitrary",
              payload: {
                // TODO: get chain info from ewallet or
                // there needs to be a public client on the attached side
                // for chain info management and tx simulation
                chain_info: {
                  chain_id: "1",
                  chain_name: "ethereum",
                  chain_symbol_image_url:
                    "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/eip155:1/chain.png",
                },
                signer: parameters.data.address,
                data: parameters.data.message,
                origin,
              },
            },
          },
        };

        const openModalAck = await eWallet.showModal(msg);
        if (openModalAck.msg_type === "show_modal_ack") {
          await eWallet.hideModal();

          if (openModalAck.payload === "approve") {
            const msgHash = hashEthereumMessage(parameters.data.message);

            const res = await eWallet.sendMsgToIframe({
              msg_type: "make_signature",
              payload: {
                msg: msgHash,
              },
            });
            const signature = encodeEthereumSignature(res.payload.sign_output);

            return {
              type: "signature",
              signature: serializeSignature(signature),
            };
          }

          if (openModalAck.payload === "reject") {
            throw new Error("User rejected the signature request");
          }
        }

        throw new Error("Unreachable");
      }
      case "sign_typedData_v4": {
        const origin = eWallet.origin;

        const msg: EWalletMsg = {
          msg_type: "show_modal",
          payload: {
            modal_type: "make_signature",
            is_demo: true,
            data: {
              chain_type: "eth",
              sign_type: "eip712",
              payload: {
                chain_info: {
                  chain_id: "1",
                  chain_name: "ethereum",
                  chain_symbol_image_url:
                    "https://raw.githubusercontent.com/chainapsis/keplr-chain-registry/main/images/eip155:1/chain.png",
                },
                signer: parameters.data.address,
                data: parameters.data.message,
                origin,
              },
            },
          },
        };

        const openModalAck = await eWallet.showModal(msg);
        if (openModalAck.msg_type === "show_modal_ack") {
          await eWallet.hideModal();

          if (openModalAck.payload === "approve") {
            const msgHash = hashEthereumTypedData(parameters.data.message);

            const res = await eWallet.sendMsgToIframe({
              msg_type: "make_signature",
              payload: {
                msg: msgHash,
              },
            });
            const signature = encodeEthereumSignature(res.payload.sign_output);

            return {
              type: "signature",
              signature: serializeSignature(signature),
            };
          }

          if (openModalAck.payload === "reject") {
            throw new Error("User rejected the signature request");
          }
        }

        throw new Error("Unreachable");
      }
      default: {
        throw new Error(`Unknown sign method: ${(parameters as any).type}`);
      }
    }
  };
}
