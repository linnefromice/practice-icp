yarn hardhat --network mumbai deploy:int256-oracle
yarn hardhat --network mumbai deploy:int128-oracle
yarn hardhat --network mumbai deploy:int64-oracle

yarn hardhat --network mumbai verify --contract contracts/Int256Oracle.sol:Int256Oracle 0x3e5a5f9fe551e75b7f590d815b4eba421b92b9e6
yarn hardhat --network mumbai verify --contract contracts/Int128Oracle.sol:Int128Oracle 0xEeF0CCeb47554872D4E6C1471e0037254148B96B
yarn hardhat --network mumbai verify --contract contracts/Int64Oracle.sol:Int64Oracle 0x1B9A10F9FC5770cAa4878102E11a239A5CAe57e5
