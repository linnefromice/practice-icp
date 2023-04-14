import { task } from "hardhat/config";

task("utils:accounts", "Prints the list of accounts", async (_, hre) => {
  const { ethers, network } = hre;
  console.log(`[utils:accounts] network: ${network.name}`);
  const accounts = await ethers.getSigners();

  for await (const account of accounts) {
    const balance = await ethers.provider.getBalance(account.address);
    console.log(`${account.address}: ${ethers.utils.formatEther(balance)}`);
  }
});
