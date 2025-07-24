# ewallet_sdk_eth

- This package is a variant of `@keplr-ewallet/ewallet-sdk-core` for Ethereum.
- It provides a way to interact with Ethereum wallets using `@keplr-ewallet/ewallet-sdk-core`.

## Example

### Create Wallet

```ts
import { initKeplrEwallet } from "@keplr-ewallet/ewallet-sdk-core";
import { initEthEWallet } from "@keplr-ewallet/ewallet-sdk-eth";

const eWallet = await initKeplrEwallet({
  element_id: "keplr-ewallet",
});

const ethEWallet = await initEthEWallet({
  eWallet,
});
```

### Get Provider

```ts
const provider = await ethEWallet.getEthereumProvider();
const chainId = await provider.request({
  method: "eth_chainId",
});
```

### Viem Integration

1. Create a viem account

```ts
const account = await toViemAccount(eWallet);
```

2. Create a viem client

```ts
import { createPublicClient, createWalletClient, custom } from "viem";

const provider = await ethEWallet.getEthereumProvider();

const publicClient = createPublicClient({
  chain: sepolia,
  transport: custom(provider),
});

const walletClient = createWalletClient({
  account: (
    await provider.request({
      method: "eth_accounts",
    })
  )[0],
  chain: sepolia,
  transport: custom(provider),
});
```

### Ethers Integration

```ts
import { BrowserProvider } from "ethers";

const provider = new BrowserProvider(await ethEWallet.getEthereumProvider());

const blockNumber = await provider.getBlockNumber();

const signer = await provider.getSigner();

const signature = await signer.signMessage("Hello, world!");
```

### Create Provider only

```ts
import { initEWalletEIP1193Provider } from "@keplr-ewallet/ewallet-sdk-eth";

const provider = await initEWalletEIP1193Provider({
  id: "keplr-ewallet",
});
```

or create a provider manually

```ts
import { initEWalletEIP1193Provider } from "@keplr-ewallet/ewallet-sdk-eth";

const provider = new EWalletEIP1193Provider({
  id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
  chains: [
    {
      chainId: "0x1",
      chainName: "Ethereum Mainnet",
      nativeCurrency: {
        name: "Ether",
        symbol: "ETH",
        decimals: 18,
      },
      rpcUrls: ["https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID"],
      blockExplorerUrls: ["https://etherscan.io"],
    },
  ],
});
```

## Tests

```bash
$ yarn test
$ yarn test:debug
$ yarn test:watch
$ yarn test:coverage
```
