// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract Int128Oracle is Initializable {
    mapping(address => int128) public state;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function version() public pure virtual returns (int128) {
        return 1; 
    }

    function updateState(
        int128 value
    ) public virtual {
        state[msg.sender] = value;
    }
}
