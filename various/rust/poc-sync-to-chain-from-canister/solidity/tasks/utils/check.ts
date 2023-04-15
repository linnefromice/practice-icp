import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { OracleV1__factory } from "../../typechain-types";

const ADDR_V1 = "";
task("check:oraclev1", "check:oraclev1").setAction(
  async ({}, hre: HardhatRuntimeEnvironment) => {
    const { ethers, network } = hre;
    console.log(`[check:oraclev1] START - ${network.name}`);

    const contract = OracleV1__factory.connect(ADDR_V1, ethers.provider);
    console.log(`address: ${contract.address}`);

    const latestRoundId = await contract.latestRoundId();
    console.log(`> latestRoundId: ${latestRoundId.toString()}`);
  }
);
