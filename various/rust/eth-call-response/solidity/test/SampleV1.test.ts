import { ethers } from "hardhat";
import { SampleV1__factory } from "../typechain-types";
import { expect } from "chai";

describe("SampleV1", () => {
  const returnedAddr = "0xdCad3a6d3569DF655070DEd06cb7A1b2Ccd1D3AF";
  const setup = async () => {
    const [deployer] = await ethers.getSigners();

    const contract = await new SampleV1__factory(deployer).deploy();
    await contract.deployed();

    return { deployer, contract };
  };

  it("initialize", async () => {
    const { contract } = await setup();
    expect((await contract.return_address()).toString()).equal(returnedAddr);
    expect(await contract.return_uint8(false)).equal(0);
    expect(await contract.return_uint8(true)).equal(2 ** 8 - 1);
    expect((await contract.return_uint256(false)).toString()).equal("0");
    expect((await contract.return_uint256(true)).toString()).equal(
      ethers.constants.MaxUint256.toString()
    );
    expect(await contract.return_string()).equal("Hello World");
  });
  it(".return_transfer", async () => {
    const { contract } = await setup();
    const transferZero = {
      from: ethers.constants.AddressZero,
      to: ethers.constants.AddressZero,
      value: "0",
    };
    const transferMax = {
      from: returnedAddr,
      to: returnedAddr,
      value: ethers.constants.MaxUint256.toString(),
    };
    contract.return_transfer_zero_value().then((v) => {
      const expected = transferZero;
      expect(v.from).to(expected.from);
      expect(v.to).to(expected.to);
      expect(v.value).to(expected.value);
    });
    contract.return_transfer_max_value().then((v) => {
      const expected = transferMax;
      expect(v.from).to(expected.from);
      expect(v.to).to(expected.to);
      expect(v.value).to(expected.value);
    });
    contract.return_multi_transfer(false, 3).then((values) => {
      const expected = transferZero;
      for (const v of values) {
        expect(v.from).to(expected.from);
        expect(v.to).to(expected.to);
        expect(v.value).to(expected.value);
      }
    });
    contract.return_multi_transfer(true, 3).then((values) => {
      const expected = transferMax;
      for (const v of values) {
        expect(v.from).to(expected.from);
        expect(v.to).to(expected.to);
        expect(v.value).to(expected.value);
      }
    });
  });
  it("transfers", async () => {
    const { contract } = await setup();
    expect((await contract.getTransferCount()).toString()).equal("0");
    await contract.addTransfer(ethers.constants.AddressZero, returnedAddr, 100);
    await contract.addTransfer(ethers.constants.AddressZero, returnedAddr, 200);
    await contract.addTransfer(ethers.constants.AddressZero, returnedAddr, 300);
    expect((await contract.getTransferCount()).toString()).equal("3");
    contract.getTransfers(0, 3).then((values) => {
      for (const [idx, v] of values.entries()) {
        expect(v.from).to(ethers.constants.AddressZero);
        expect(v.to).to(returnedAddr);
        expect(v.value.toString()).to((100 * (idx + 1)).toString());
      }
    });
    await contract.removeAllTransfers();
    expect((await contract.getTransferCount()).toString()).equal("0");
  });
});
