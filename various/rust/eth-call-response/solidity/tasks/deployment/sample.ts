import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { Lock__factory } from "../../typechain-types";

task("deploy:lock", "deploy:lock")
  .addOptionalParam("unlockTime", "unlockTime")
  .addOptionalParam("lockedAmount", "lockedAmount")
  .addOptionalParam("deployer", "deployer")
  .setAction(async (
    { unlockTime, lockedAmount, deployer }: { unlockTime: string, lockedAmount: string, deployer: string },
    hre: HardhatRuntimeEnvironment
  ) => {
    const { ethers, network } = hre
    console.log(`[deploy:lock] START - ${network.name}`)

    const _unlockTime = unlockTime
      ? Number.parseInt(unlockTime)
      : Math.round(Date.now() / 1000) + 300;
    const _lockedAmount = ethers.utils.parseEther(lockedAmount ?? "0.01")
    const _deployer = deployer
      ? await ethers.getSigner(deployer)
      : (await ethers.getSigners())[0]

    const contract = await new Lock__factory(_deployer).deploy(_unlockTime, { value: _lockedAmount })
    console.log(`deployed tx: ${contract.deployTransaction.hash}`)
    await contract.deployed()
    console.log(`deployed! address: ${contract.address}`)

    console.log(`Check phase`)
    const resUnlockTime = await contract.unlockTime()
    console.log(`> unlockTime: ${new Date(resUnlockTime.toNumber() * 1000).toISOString()}`)
    const balance = await ethers.provider.getBalance(contract.address)
    console.log(`> balance: ${ethers.utils.formatEther(balance)}`)
    console.log(`> owner: ${await contract.owner()}`)

    console.log(`[deploy:lock] END`)
  })
