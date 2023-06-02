// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

contract OracleV1 {
    struct ExchangeRate {
        uint256 rate;
        uint256 fromTime;
        uint256 toTime;
    }

    event AddRate(
        uint256 index,
        uint256 rate,
        uint256 fromTime,
        uint256 toTime
    );

    ExchangeRate[] public rates;

    function updateState(
        ExchangeRate memory _rate
    ) public {
        rates.push(_rate);

        emit AddRate(
            rates.length - 1,
            _rate.rate,
            _rate.fromTime,
            _rate.toTime
        );
    }

    function getStateLength() public view returns (uint256) {
        return rates.length;
    }

    function debug_getExchangeRates(uint256 from, uint256 count) public view returns(ExchangeRate[] memory) {
        ExchangeRate[] memory _rates = new ExchangeRate[](count);
        for (uint i = 0; i < count; i++) {
            _rates[i] = rates[from + i];
        }
        return _rates;
    }

    function debug_getExchangeRatesFromIdxs(uint256[] memory idxs) public view returns(ExchangeRate[] memory) {
        ExchangeRate[] memory _rates = new ExchangeRate[](idxs.length);
        for (uint i = 0; i < idxs.length; i++) {
            _rates[i] = rates[idxs[i]];
        }
        return _rates;
    }
    
    function debug_cleanExchangeRates() public {
        delete rates;
    }
}
