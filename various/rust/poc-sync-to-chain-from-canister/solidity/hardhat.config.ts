import * as dotenv from "dotenv"

import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { HardhatNetworkUserConfig, HttpNetworkAccountsUserConfig, NetworksUserConfig } from "hardhat/types";

dotenv.config()

const MNEMONIC = process.env.MNEMONIC || ''
const COINMARKETCAP_KEY = process.env.COINMARKETCAP_KEY || ''
const POLYGON_RPC = process.env.POLYGON_RPC || "https://polygon-rpc.com/"
const POLYGON_MUMBAI_RPC = process.env.POLYGON_MUMBAI_RPC || 'https://rpc-mumbai.maticvigil.com/'

const HARDHAT_CHAINID = 31337;
const DEFAULT_BLOCK_GAS_LIMIT = 30000000;
const GWEI = 1000 * 1000 * 1000;
const localNetwork: HardhatNetworkUserConfig = {
  blockGasLimit: DEFAULT_BLOCK_GAS_LIMIT,
  gas: DEFAULT_BLOCK_GAS_LIMIT,
  gasPrice: 3 * GWEI,
  throwOnTransactionFailures: true,
  throwOnCallFailures: true,
  allowUnlimitedContractSize: true,
}

const ACCOUNTS: HttpNetworkAccountsUserConfig = {
  mnemonic: MNEMONIC,
  path: "m/44'/60'/0'/0",
  initialIndex: 0,
  count: 20,
}
const NETWORK_CONFIGS: NetworksUserConfig = {
  polygon: {
    chainId: 137,
    url: POLYGON_RPC,
    gasPrice: 1 * GWEI,
    accounts: ACCOUNTS,
  },
  mumbai: {
    chainId: 80001,
    url: POLYGON_MUMBAI_RPC,
    gasPrice: 1 * GWEI,
    accounts: ACCOUNTS,
  }
}

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.18",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    hardhat: {
      ...localNetwork,
      chainId: HARDHAT_CHAINID
    },
    ...NETWORK_CONFIGS
  },
  gasReporter: {
    enabled: true,
    currency: 'JPY',
    gasPrice: 20,
    token: 'MATIC',
    coinmarketcap: COINMARKETCAP_KEY,
    showTimeSpent: true,
    showMethodSig: true,
  },
};

export default config;
