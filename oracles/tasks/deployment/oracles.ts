import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import {
  OracleV1,
  OracleV1__factory,
  StringOracle,
  StringOracle__factory,
  Uint256Oracle,
  Uint256Oracle__factory,
} from "../../typechain-types";

task("deploy:oraclev1", "deploy:oraclev1")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network, upgrades } = hre;
      console.log(`[deploy:oraclev1] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      // Deployment
      const contract = (await upgrades.deployProxy(
        new OracleV1__factory(_deployer)
      )) as OracleV1;
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployTransaction.wait();
      console.log(`deployed! address: ${contract.address}`);

      // Check after deploying
      console.log(`Check phase`);
      console.log(`> version: ${(await contract.version()).toString()}`);

      // Verification
      if (network.name !== "hardhat") {
        await hre.run("verify:verify", {
          address: contract.address,
          constructorArguments: [],
        });
      }

      console.log(`[deploy:oraclev1] END`);
    }
  );

task("deploy:string-oracle", "deploy:string-oracle")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network, upgrades } = hre;
      console.log(`[deploy:string-oracle] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      // Deployment
      const contract = (await upgrades.deployProxy(
        new StringOracle__factory(_deployer)
      )) as StringOracle;
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployTransaction.wait();
      console.log(`deployed! address: ${contract.address}`);

      // Check after deploying
      console.log(`Check phase`);
      console.log(`> version: ${(await contract.version()).toString()}`);

      // Verification
      if (network.name !== "hardhat") {
        await hre.run("verify:verify", {
          address: contract.address,
          constructorArguments: [],
        });
      }

      console.log(`[deploy:string-oracle] END`);
    }
  );

task("deploy:uint256-oracle", "deploy:uint256-oracle")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network, upgrades } = hre;
      console.log(`[deploy:uint256-oracle] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      // Deployment
      const contract = (await upgrades.deployProxy(
        new Uint256Oracle__factory(_deployer)
      )) as Uint256Oracle;
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployTransaction.wait();
      console.log(`deployed! address: ${contract.address}`);

      // Check after deploying
      console.log(`Check phase`);
      console.log(`> version: ${(await contract.version()).toString()}`);

      // Verification
      if (network.name !== "hardhat") {
        await hre.run("verify:verify", {
          address: contract.address,
          constructorArguments: [],
        });
      }

      console.log(`[deploy:uint256-oracle] END`);
    }
  );
