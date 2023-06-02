import { ethers, upgrades } from "hardhat";
import { StringOracle, StringOracle__factory } from "../typechain-types";
import { expect } from "chai";

describe("StringOracle", () => {
  const setup = async () => {
    const [deployer, user] = await ethers.getSigners();

    const oracle = (await upgrades.deployProxy(
      new StringOracle__factory(deployer)
    )) as StringOracle;
    await oracle.deployTransaction.wait();

    return { deployer, user, oracle };
  };
  it(".version", async () => {
    const { oracle } = await setup();
    expect((await oracle.version()).toString()).eq("1");
  });
  it(".updateState", async () => {
    const { deployer, user, oracle } = await setup();
    await oracle.connect(deployer).updateState("AAAA");
    await oracle.connect(user).updateState("BBBB");
    expect(await oracle.state(deployer.address)).eq("AAAA");
    expect(await oracle.state(user.address)).eq("BBBB");

    await oracle.connect(user).updateState("xyZ123");
    await oracle.connect(deployer).updateState("Abc321");
    expect(await oracle.state(deployer.address)).eq("Abc321");
    expect(await oracle.state(user.address)).eq("xyZ123");
  });
});
