
### Avalanche EVM gasless transaction

[ERC-2771](https://eips.ethereum.org/EIPS/eip-2771) (gasless transaction) on Avalanche EVM chains.

TODOs
- CLI to create AWS KMS keys + grants for cross-account access
- Javascript example for EIP-712 message signing



<br><hr>

#### Step 1. Install dependencies

Install [Foundry](https://github.com/foundry-rs/foundry#installation), and make sure the following works:

```bash
forge --version
cast --version
```



<br><hr>

#### Step 2. Download trusted forwarder contract

```bash
git clone git@github.com:ava-labs/avalanche-evm-gasless-transaction.git

cd ${HOME}/avalanche-evm-gasless-transaction
git submodule update --init --recursive
```



<br><hr>

#### Step 3. Deploy trusted forwarder contract

To deploy the trusted forwarder contract, you need existing EVM RPC URLs.

If you do not have one yet, run the following local node for testing purposes:

```bash
cd ${HOME}/avalanchego
./scripts/build.sh

cd ${HOME}/avalanchego
./build/avalanchego \
--network-id=local \
--staking-enabled=false \
--db-type=memdb \
--log-level=info
```

Now that we have the RPC endpoint, let's deploy the trusted forwarder contract:

```bash
cd ${HOME}/avalanche-evm-gasless-transaction
forge create \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
src/Forwarder.sol:Forwarder
```

Sample outputs are:

```
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922
Transaction hash: ...
```

In the example above, the trusted forwarder contract address is `0x52C84043CD9c865236f11d9Fc9F56aa003c1f922`, which will be required for the following steps (domain separator, type name registration, etc.).



<br><hr>

#### Step 4. Register domain separator and request type

Register the domain separator and request type with the trusted forwarder:

```bash
# private key "56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027" maps to "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC"
cast send \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 \
"registerDomainSeparator(string name, string version)" \
"my domain name" \
"1"

# private key "56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027" maps to "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC"
cast send \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 \
"registerRequestType(string typeName, string typeSuffix)" \
"my type name" \
"bytes8 typeSuffixDatadatadatada)"
```

Without the `registerDomainSeparator` call, EIP-712 transactions will fail with `FWD: unregistered domain sep`. Without the `registerRequestType` call, EIP-712 transactions will fail with `FWD: unregistered typehash`.
- https://eips.ethereum.org/EIPS/eip-712#definition-of-domainseparator
- https://eips.ethereum.org/EIPS/eip-712#rationale-for-domainseparator



<br><hr>

#### Step 5. Start the gas relayer

Please contact the Avalanche support team.



<br><hr>

#### Step 6. Deploy test counter contract

Deploy a simple test `GaslessCounter` contract that is EIP-2771 compliant:

```bash
cd ${HOME}/avalanche-evm-gasless-transaction
forge create \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
src/GaslessCounter.sol:GaslessCounter \
--constructor-args 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922
```

Sample outputs are:

```
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x5DB9A7629912EBF95876228C24A848de0bfB43A9
Transaction hash: ...
```



<br><hr>

#### Step 7. Test counter contract

Make sure the key without any balance cannot increment/decrement the counter, not able to pay the gas fees:

```bash
# THIS SHOULD FAIL
# account with no balance cannot send any transaction
# due to no gas
#
# private key "1af42b797a6bfbd3cf7554bed261e876db69190f5eb1b806acbd72046ee957c3"
# maps to "0xb513578fAb80487a7Af50e0b2feC381D0BD8fa9D"
cast send \
--private-key=1af42b797a6bfbd3cf7554bed261e876db69190f5eb1b806acbd72046ee957c3 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
"increment()"
# (code: -32000, message: gas required exceeds allowance (0), data: None)
```

Confirm that the transactions were NOT processed:

```bash
cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
"getNumber()" | sed -r '/^\s*$/d' | tail -1

cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
"getLast()"
```

Now send the gasless transaction (e.g., http://127.0.0.1:9876/rpc is the gas relayer server RPC URL):

```bash
cd ${HOME}/avalanche-evm-gasless-transaction
./target/release/avalanche-evm-gasless-transaction \
gasless-counter-increment \
--gas-relayer-server-rpc-url http://127.0.0.1:9876/rpc \
--chain-rpc-url http://127.0.0.1:9650/ext/bc/C/rpc \
--trusted-forwarder-contract-address 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 \
--recipient-contract-address 0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
--domain-name "my domain name" \
--domain-version "1" \
--type-name "my type name" \
--type-suffix-data "bytes8 typeSuffixDatadatadatada)" 
```

Confirm that the transactions were successfully processed:

```bash
cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000001

cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
"getLast()"
# 0x00000000000000000000000054ba2b96d1318900f3d1e893c9f2048458ed9120
```
