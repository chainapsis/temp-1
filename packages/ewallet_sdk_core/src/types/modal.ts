export type ShowModalPayload = MakeSignatureModalPayload | OtherModalPayload;

export type ModalResponse = "approve" | "reject";

export interface OtherModalPayload {
  modal_type: "other";
  data: {};
}

export type MakeSignatureModalPayload = {
  modal_type: "make_signature";
  is_demo: boolean;
  data: MakeCosmosSigData | MakeEthereumSigData;
};

export type ChainInfoForAttachedModal = {
  readonly chain_id: string;
  readonly chain_name: string;
  readonly chain_symbol_image_url?: string;
};

type MakeCosmosSigData =
  | {
    chain_type: "cosmos";
    sign_type: "tx";
    payload: CosmosTxSignPayload;
  }
  | {
    chain_type: "cosmos";
    sign_type: "arbitrary";
    payload: CosmosArbitrarySignPayload;
  };

export type CosmosTxSignPayload = {
  origin: string;
  chain_info: ChainInfoForAttachedModal;
  signer: string;
  signDocString: string;
  msgs: any[];
};

export type CosmosArbitrarySignPayload = {
  chain_info: ChainInfoForAttachedModal;
  signer: string;
  data: string | Uint8Array;
  origin: string;
};

type MakeEthereumSigData =
  | {
    chain_type: "eth";
    sign_type: "tx";
    payload: EthereumTxSignPayload;
  }
  | {
    chain_type: "eth";
    sign_type: "arbitrary";
    payload: EthereumArbitrarySignPayload;
  }
  | {
    chain_type: "eth";
    sign_type: "eip712";
    payload: EthereumEip712SignPayload;
  };

//NOTE 아직 확정된 타입이 아닌 임의로 만들어 둔거라서 실제 sign 작업할때 수정 필요 @retto
export type EthereumTxSignPayload = {
  origin: string;
  chain_info: ChainInfoForAttachedModal;
  signer: string;
  data: any;
};

export type EthereumArbitrarySignPayload = {
  origin: string;
  chain_info: ChainInfoForAttachedModal;
  signer: string;
  data: any;
};

export type EthereumEip712SignPayload = {
  origin: string;
  chain_info: ChainInfoForAttachedModal;
  signer: string;
  data: any;
};
