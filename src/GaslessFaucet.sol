// SPDX-License-Identifier: CC-BY-SA-4.0

pragma solidity ^0.8.13;

import "@opengsn/contracts/src/ERC2771Recipient.sol";

// ref. <https://github.com/ethereumbook/ethereumbook/blob/develop/code/Solidity/Faucet.sol>
// ref. <https://cypherpunks-core.github.io/ethereumbook/07smart-contracts-solidity.html#original_sol_faucet>
// ref. <https://medium.com/coinmonks/solidity-transfer-vs-send-vs-call-function-64c92cfc878a>
contract GaslessFaucet is ERC2771Recipient {
    // Accept any incoming amount
    receive() external payable {}

    // Give out ether to anyone who asks (deposit).
    function withdraw(uint256 withdraw_amount) public payable {
        // Limit withdrawal amount
        require(withdraw_amount <= 100000000000000000);

        // Send the amount to the address that requested it
        // payable(msg.sender).transfer(withdraw_amount);
        payable(_msgSender()).transfer(withdraw_amount);
    }
}
