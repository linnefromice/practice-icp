// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract Int256Oracle is Initializable {
    mapping(address => int256) public state;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function version() public pure virtual returns (int256) {
        return 1; 
    }

    function updateState(
        int256 value
    ) public virtual {
        state[msg.sender] = value;
    }
}
