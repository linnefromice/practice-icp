import { ethers, providers } from "ethers"

const URL = "https://eth-mainnet.g.alchemy.com/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy";
const MINITE = 60;
const HOUR = 60 * MINITE;
const DAY = 24 * HOUR;
const MONTH = 30 * DAY;

const BLOCK_GENERATED_SEC = 15
const BLOCK_COUNT_PER_MONTH = MONTH / BLOCK_GENERATED_SEC

// Get the latest block number
const getLatestBlockNumber = async (provider: providers.JsonRpcProvider) => {
  const blockNumber = await provider.getBlockNumber();
  // console.log(`Latest block number: ${blockNumber}`);
  return blockNumber;
}

// Get a block by its number
const getBlockByNumber = async (provider: providers.JsonRpcProvider, blockNumber: number) => {
  const block = await provider.getBlock(blockNumber);
  // console.log(`Block number ${blockNumber}:`, block);
  return block
}

const logBlockTimestamp = (block: providers.Block) => console.log(`${block.number}: ${new Date(block.timestamp * 1000).toISOString()} (${block.timestamp})`)

const main = async () => {
  const now = Math.floor(new Date().getTime() / 1000)
  const provider = new ethers.providers.JsonRpcProvider(URL);

  const latestBlockNumber = await getLatestBlockNumber(provider);
  // const block0 = await getBlockByNumber(provider, 0);
  // console.log(`0: ${new Date(block0.timestamp * 1000).toString()} (${block0.timestamp})`)
  // const block1 = await getBlockByNumber(provider, 1);
  // console.log(`1: ${new Date(block1.timestamp * 1000).toString()} (${block1.timestamp})`)
  const latestBlock = await getBlockByNumber(provider, latestBlockNumber);
  console.log(`${latestBlockNumber}: ${new Date(latestBlock.timestamp * 1000).toString()} (${latestBlock.timestamp})`)
  const baseBlockNumber = 13916168
  console.log(`baseBlockNumber: ${baseBlockNumber}`)
  logBlockTimestamp(await getBlockByNumber(provider, baseBlockNumber - 4))
  logBlockTimestamp(await getBlockByNumber(provider, baseBlockNumber - 3))
  logBlockTimestamp(await getBlockByNumber(provider, baseBlockNumber - 2))
  logBlockTimestamp(await getBlockByNumber(provider, baseBlockNumber - 1))
  logBlockTimestamp(await getBlockByNumber(provider, baseBlockNumber))
}

main()
  .then(() => console.log("DONE"))
  .catch((err: any) => {
    console.log("FAILED")
    console.error(err)
  })