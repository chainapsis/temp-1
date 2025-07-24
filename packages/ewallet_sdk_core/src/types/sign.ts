import type { Hex } from "viem";

export interface FullSignature {
  big_r: string;
  s: string;
}

export interface SignOutput {
  sig: FullSignature;
  is_high: boolean;
}

export type ChainType = "ethereum" | "cosmos";

export type MakeSignatureRequestType =
  | "personal_sign"
  | "sign_transaction"
  | "sign_typedData_v4"
  | "sign_amino"
  | "sign_direct";

export type MakeSignatureResultType =
  | "signature"
  | "signed_transaction"
  | "signed_direct"
  | "signed_amino";

export type MakeSignatureResult<
  T extends MakeSignatureResultType,
  C extends ChainType,
  V,
> = {
  type: T;
  chainType: C;
} & V;

export type EthereumSignatureResult = MakeSignatureResult<
  "signature",
  "ethereum",
  { signature: Hex }
>;

export type EWalletMakeSignaturePayload = {
  msg: Uint8Array;
};

export type EWalletMakeSignatureAckPayload = {
  sign_output: SignOutput;
};
