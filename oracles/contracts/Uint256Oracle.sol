// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract Uint256Oracle is Initializable {
    mapping(address => uint256) public state;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function version() public pure virtual returns (uint256) {
        return 1; 
    }

    function updateState(
        uint256 value
    ) public virtual {
        state[msg.sender] = value;
    }
}
