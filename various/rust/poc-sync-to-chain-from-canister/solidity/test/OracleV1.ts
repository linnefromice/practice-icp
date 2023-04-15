import { ethers } from "hardhat";
import { OracleV1__factory } from "../typechain-types";
import { expect } from "chai";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("OracleV1", () => {
  const setup = async () => {
    const [deployer] = await ethers.getSigners();

    const oracle = await new OracleV1__factory(deployer).deploy();
    await oracle.deployed();

    return { deployer, oracle };
  };

  it("initialize", async () => {
    const { oracle } = await setup();
    expect((await oracle.latestRoundId()).toString()).equal("0");
  });
  it(".updateState", async () => {
    const { oracle } = await setup();
    const currentTime = await time.latest();
    await expect(oracle.updateState(1, 100, currentTime, currentTime))
      .to.be.emit(oracle, "UpdateState")
      .withArgs("1", "100", currentTime.toString(), currentTime.toString());

    expect((await oracle.latestRoundId()).toString()).eq("1");
    const round = await oracle.rounds(1);
    expect(round.answer.toString()).eq("100");
    expect(round.startedAt.toString()).eq(currentTime.toString());
    expect(round.updatedAt.toString()).eq(currentTime.toString());
  });
});
