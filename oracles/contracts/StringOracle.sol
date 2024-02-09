// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract StringOracle is Initializable {
    mapping(address => string) public state;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function version() public pure virtual returns (uint256) {
        return 1; 
    }

    function updateState(
        string memory value
    ) public virtual {
        state[msg.sender] = value;
    }

    function hello(string memory value) public pure virtual returns (string memory) {
        bytes memory concatenatedBytes = abi.encodePacked("Hello, ", value);
        return string(concatenatedBytes);
    }
}
