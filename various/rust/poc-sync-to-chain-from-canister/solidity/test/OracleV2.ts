import { ethers } from "hardhat";
import { OracleV2__factory } from "../typechain-types";
import { expect } from "chai";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("OracleV2", () => {
  const setup = async () => {
    const [deployer] = await ethers.getSigners();

    const oracle = await new OracleV2__factory(deployer).deploy();
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
  it(".updateStates", async () => {
    const { oracle } = await setup();
    const currentTime = await time.latest();

    await oracle.updateStates([
      {
        roundId: 1,
        answer: 100,
        startedAt: currentTime,
        updatedAt: currentTime,
      },
      {
        roundId: 2,
        answer: 120,
        startedAt: currentTime + 1,
        updatedAt: currentTime + 1,
      },
      {
        roundId: 3,
        answer: 140,
        startedAt: currentTime + 2,
        updatedAt: currentTime + 2,
      },
    ]);
    expect((await oracle.latestRoundId()).toString()).eq("3");
    expect((await oracle.rounds(1)).answer.toString()).eq("100");
    expect((await oracle.rounds(2)).answer.toString()).eq("120");
    expect((await oracle.rounds(3)).answer.toString()).eq("140");
    expect((await oracle.rounds(4)).answer.toString()).eq("0");
    expect((await oracle.rounds(5)).answer.toString()).eq("0");
    expect((await oracle.rounds(6)).answer.toString()).eq("0");

    await oracle.updateStates([
      {
        roundId: 4,
        answer: 160,
        startedAt: currentTime + 3,
        updatedAt: currentTime + 3,
      },
      {
        roundId: 5,
        answer: 180,
        startedAt: currentTime + 4,
        updatedAt: currentTime + 4,
      },
    ]);
    expect((await oracle.latestRoundId()).toString()).eq("5");
    expect((await oracle.rounds(1)).answer.toString()).eq("100");
    expect((await oracle.rounds(2)).answer.toString()).eq("120");
    expect((await oracle.rounds(3)).answer.toString()).eq("140");
    expect((await oracle.rounds(4)).answer.toString()).eq("160");
    expect((await oracle.rounds(5)).answer.toString()).eq("180");
    expect((await oracle.rounds(6)).answer.toString()).eq("0");
  });
  it(".debug_cleanState", async () => {
    const { oracle } = await setup();
    const currentTime = await time.latest();
    await oracle.updateState(1, 100, currentTime, currentTime);
    await oracle.updateState(2, 120, currentTime + 1, currentTime + 1);
    await oracle.updateState(3, 140, currentTime + 2, currentTime + 2);
    expect((await oracle.latestRoundId()).toString()).eq("3");
    expect((await oracle.rounds(1)).answer.toString()).eq("100");
    expect((await oracle.rounds(2)).answer.toString()).eq("120");
    expect((await oracle.rounds(3)).answer.toString()).eq("140");

    await oracle.debug_cleanState();
    expect((await oracle.latestRoundId()).toString()).eq("0");
    expect((await oracle.rounds(1)).answer.toString()).eq("0");
    expect((await oracle.rounds(2)).answer.toString()).eq("0");
    expect((await oracle.rounds(3)).answer.toString()).eq("0");
  });
});
