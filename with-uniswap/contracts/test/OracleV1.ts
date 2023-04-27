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
    expect((await oracle.getStateLength()).toString()).equal("0");
  });
  it(".updateState", async () => {
    const { oracle } = await setup();
    const currentTime = await time.latest();
    await expect(oracle.updateState({
      rate: 100,
      fromTime: currentTime,
      toTime: currentTime,
    }))
      .to.be.emit(oracle, "AddRate")
      .withArgs("0", "100", currentTime.toString(), currentTime.toString());

    expect((await oracle.getStateLength()).toString()).eq("1");
    const round = await oracle.rates(0);
    expect(round.rate.toString()).eq("100");
    expect(round.fromTime.toString()).eq(currentTime.toString());
    expect(round.toTime.toString()).eq(currentTime.toString());
  });
  it(".debug_cleanExchangeRates", async () => {
    const { oracle } = await setup();
    const currentTime = await time.latest();
    await oracle.updateState({
      rate: 100,
      fromTime: currentTime,
      toTime: currentTime,
    });
    await oracle.updateState({
      rate: 120,
      fromTime: currentTime + 1,
      toTime: currentTime + 1,
    });
    await oracle.updateState({
      rate: 140,
      fromTime: currentTime + 2,
      toTime: currentTime + 2,
    });
    expect((await oracle.getStateLength()).toString()).eq("3");
    expect((await oracle.rates(0)).rate.toString()).eq("100");
    expect((await oracle.rates(1)).rate.toString()).eq("120");
    expect((await oracle.rates(2)).rate.toString()).eq("140");

    await oracle.debug_cleanExchangeRates();
    expect((await oracle.getStateLength()).toString()).eq("0");
  });
});
