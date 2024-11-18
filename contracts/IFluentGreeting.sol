// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IFluentGreeting {
    function startGame() external  returns (string memory);

    function getResult() external view returns (string memory);
}
