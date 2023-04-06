// SPDX-License-Identifier: MIT

pragma solidity ^0.8.13;

import "@opengsn/contracts/src/ERC2771Recipient.sol";

contract GaslessCounter is ERC2771Recipient {
    uint256 public number;
    address public last;

    event DebugAddress(address indexed _addr);

    constructor(address _forwarder) {
        _setTrustedForwarder(_forwarder);
    }

    function setNumber(uint256 newNumber) public {
        number = newNumber;

        last = _msgSender(); // not "msg.sender"
    }

    function increment() public {
        number++;

        last = _msgSender(); // not "msg.sender"
    }

    function decrement() public {
        require(number > 0, "Counter: decrement overflow");
        number--;

        last = _msgSender(); // not "msg.sender"
    }

    function getNumber() public view returns (uint256) {
        return number;
    }

    function getLast() public view returns (address) {
        return last;
    }
}
