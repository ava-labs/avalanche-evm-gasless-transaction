```bash
rm -rf ${HOME}/avalanche-evm-gasless-transaction
mkdir -p ${HOME}/avalanche-evm-gasless-transaction
cd ${HOME}/avalanche-evm-gasless-transaction
forge init

cd ${HOME}/avalanche-evm-gasless-transaction
# forge install openzeppelin/openzeppelin-contracts
forge install https://github.com/OpenZeppelin/openzeppelin-contracts

# forge install opengsn/gsn@v3.0.0-beta.3
forge install https://github.com/opengsn/gsn

# vi ./lib/gsn/packages/contracts/src/forwarder/Forwarder.sol
cp ./lib/gsn/packages/contracts/src/forwarder/Forwarder.sol src/Forwarder.sol
cp ./lib/gsn/packages/contracts/src/forwarder/IForwarder.sol src/IForwarder.sol

cd ${HOME}/avalanche-evm-gasless-transaction
cat << EOF > remappings.txt
@opengsn/=lib/gsn/packages/
@openzeppelin/=lib/openzeppelin-contracts/
forge-std/=lib/forge-std/src/
hardhat/=lib/forge-std/src/
EOF
```
