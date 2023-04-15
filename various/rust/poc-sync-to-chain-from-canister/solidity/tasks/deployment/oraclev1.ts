import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { OracleV1__factory } from "../../typechain-types";

task("deploy:oraclev1", "deploy:oraclev1")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network } = hre;
      console.log(`[deploy:oraclev1] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      const contract = await new OracleV1__factory(_deployer).deploy();
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployed();
      console.log(`deployed! address: ${contract.address}`);

      console.log(`Check phase`);
      const latestRoundId = await contract.latestRoundId();
      console.log(`> latestRoundId: ${latestRoundId.toString()}`);

      console.log(`[deploy:oraclev1] END`);
    }
  );
