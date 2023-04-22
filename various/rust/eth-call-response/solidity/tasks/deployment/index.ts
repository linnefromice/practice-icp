import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { SampleV1__factory } from "../../typechain-types";

task("deploy:samplev1", "deploy:samplev1")
  .addOptionalParam("deployer", "deployer")
  .setAction(
    async (
      { deployer }: { deployer: string },
      hre: HardhatRuntimeEnvironment
    ) => {
      const { ethers, network } = hre;
      console.log(`[deploy:samplev1] START - ${network.name}`);

      const _deployer = deployer
        ? await ethers.getSigner(deployer)
        : (await ethers.getSigners())[0];

      // Deployment
      const contract = await new SampleV1__factory(_deployer).deploy();
      console.log(`deployed tx: ${contract.deployTransaction.hash}`);
      await contract.deployed();
      console.log(`deployed! address: ${contract.address}`);

      // Verification
      if (network.name != "hardhat") {
        await hre.run("verify:verify", {
          address: contract.address,
          constructorArguments: [],
        });
      }

      console.log(`[deploy:samplev1] END`);
    }
  );
