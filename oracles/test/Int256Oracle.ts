import { ethers, upgrades } from "hardhat";
import { expect } from "chai";
import { Int256Oracle__factory, Int256Oracle } from "../typechain-types";

describe("Uint256Oracle", () => {
  const setup = async () => {
    const [deployer, user] = await ethers.getSigners();

    const oracle = (await upgrades.deployProxy(
      new Int256Oracle__factory(deployer)
    )) as Int256Oracle;
    await oracle.deployTransaction.wait();

    return { deployer, user, oracle };
  };
  it(".version", async () => {
    const { oracle } = await setup();
    expect((await oracle.version()).toString()).eq("1");
  });
  it(".updateState", async () => {
    const { deployer, user, oracle } = await setup();
    await oracle.connect(deployer).updateState(100);
    await oracle.connect(user).updateState(-200);
    expect((await oracle.state(deployer.address)).toString()).eq("100");
    expect((await oracle.state(user.address)).toString()).eq("-200");

    await oracle.connect(user).updateState(1000);
    await oracle.connect(deployer).updateState(-20);
    expect((await oracle.state(deployer.address)).toString()).eq("-20");
    expect((await oracle.state(user.address)).toString()).eq("1000");
  });
});
