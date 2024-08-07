import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { OracleV1__factory, OracleV2__factory } from "../../typechain-types";

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

      // Deployment
      const contract = await new OracleV1__factory(_deployer).deploy();
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployed();
      console.log(`deployed! address: ${contract.address}`);

      // Verification
      await hre.run("verify:verify", {
        address: contract.address,
        constructorArguments: [],
      });

      // Check after deploying
      console.log(`Check phase`);
      const latestRoundId = await contract.latestRoundId();
      console.log(`> latestRoundId: ${latestRoundId.toString()}`);

      console.log(`[deploy:oraclev1] END`);
    }
  );

task("deploy:oraclev2", "deploy:oraclev2")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network } = hre;
      console.log(`[deploy:oraclev2] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      // Deployment
      const contract = await new OracleV2__factory(_deployer).deploy();
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployed();
      console.log(`deployed! address: ${contract.address}`);

      // Verification
      await hre.run("verify:verify", {
        address: contract.address,
        constructorArguments: [],
      });

      // Check after deploying
      console.log(`Check phase`);
      const latestRoundId = await contract.latestRoundId();
      console.log(`> latestRoundId: ${latestRoundId.toString()}`);

      console.log(`[deploy:oraclev2] END`);
    }
  );
