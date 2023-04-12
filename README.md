# Avalanche EVM gasless transaction

[ERC-2771](https://eips.ethereum.org/EIPS/eip-2771) (gasless transaction) on Avalanche EVM chains.

## Step 1. Install dependencies

Install [Foundry](https://github.com/foundry-rs/foundry#installation), and make sure the following works:

```bash
forge --version
cast --version
```

## Step 2. Download trusted forwarder contract

```bash
git clone git@github.com:ava-labs/avalanche-evm-gasless-transaction.git

cd ${HOME}/avalanche-evm-gasless-transaction
git submodule update --init --recursive

cd ${HOME}/avalanche-evm-gasless-transaction
forge update

cd ${HOME}/avalanche-evm-gasless-transaction
# vi ./lib/gsn/packages/contracts/src/forwarder/Forwarder.sol
cp ./lib/gsn/packages/contracts/src/forwarder/Forwarder.sol src/Forwarder.sol
cp ./lib/gsn/packages/contracts/src/forwarder/IForwarder.sol src/IForwarder.sol
```

## Step 3. Deploy trusted forwarder contract

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

```yaml
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922
Transaction hash: ...
```

In the example above, the trusted forwarder contract address is `0x52C84043CD9c865236f11d9Fc9F56aa003c1f922`, which will be required for the following steps (domain separator, type name registration, etc.).

## Step 4. Register domain separator and request type

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

Without the `registerDomainSeparator` call, EIP-712 transactions will fail with `FWD: unregistered domain sep`. Without the `registerRequestType` call, EIP-712 transactions will fail with `FWD: unregistered typehash`:
- https://eips.ethereum.org/EIPS/eip-712#definition-of-domainseparator
- https://eips.ethereum.org/EIPS/eip-712#rationale-for-domainseparator

## Step 5. Create and fund the gas paying keys

Download `avalanche-kms` from [avalanche-ops release page](https://github.com/ava-labs/avalanche-ops/releases/tag/latest):

```bash
# to check the balance of "ewoq" key
# to create keys with cross account grants, use --grantee-principal (optional)
cd ${HOME}/avalanche-ops
./target/release/avalanche-kms create \
--region=ap-northeast-2 \
--key-name-prefix aws-gas-relayer-gas-payer \
--keys 10 \
--evm-chain-rpc-url http://127.0.0.1:9650/ext/bc/C/rpc \
--evm-funding-hotkey 56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--evm-funding-amount-in-avax "999999999" \
--grantee-principal ...

# fetch the balance of the test "ewoq" key account (should be non-zero)
cd ${HOME}/avalanche-ops
./target/release/avalanche-kms evm-balance \
--chain-rpc-url http://127.0.0.1:9650/ext/bc/C/rpc \
--address 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
```

Or manually create or use existing AWS KMS keys, and pre-fund them.

## Step 6. Start the gas relayer

Please contact the Avalanche support team (with the list of your funded AWS KMS keys).

## Step 7. Deploy test counter contract

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

```bash
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x5DB9A7629912EBF95876228C24A848de0bfB43A9
Transaction hash: ...
```

## Step 8. Test counter contract in Rust

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
# 0x00000000000000000000000009cdb41fcec6410a00c7751257c33e9ea0d0c835
```

The above `avalanche-evm-gasless-transaction gasless-counter-increment` commands creates and signs the message as follows in Rust, and sends it to the gas relayer server:

```json
{
    "forwardRequest": {
        "domain": {
            "name": "my name",
            "version": "1",
            "chainId": "0xa868",
            "verifyingContract": "0x52c84043cd9c865236f11d9fc9f56aa003c1f922"
        },
        "types": {
            "EIP712Domain": [
                {
                    "name": "name",
                    "type": "string"
                },
                {
                    "name": "version",
                    "type": "string"
                },
                {
                    "name": "chainId",
                    "type": "uint256"
                },
                {
                    "name": "verifyingContract",
                    "type": "address"
                }
            ],
            "Message": [
                {
                    "name": "from",
                    "type": "address"
                },
                {
                    "name": "to",
                    "type": "address"
                },
                {
                    "name": "value",
                    "type": "uint256"
                },
                {
                    "name": "gas",
                    "type": "uint256"
                },
                {
                    "name": "nonce",
                    "type": "uint256"
                },
                {
                    "name": "data",
                    "type": "bytes"
                },
                {
                    "name": "validUntilTime",
                    "type": "uint256"
                }
            ]
        },
        "primaryType": "Message",
        "message": {
            "data": "d09de08a",
            "from": "0xc886c5a4939c8835bf7bf643f3dbcadc6eb242d1",
            "gas": "0x1d0f6",
            "nonce": "0x0",
            "to": "0x5db9a7629912ebf95876228c24a848de0bfb43a9",
            "validUntilTime": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "value": "0x0"
        }
    },
    "metadata": {
        "signature": "914b460ab5dda9bbfd0675913b19c3c0e55a0886698dfc07f0f6dd4c28e449363826e26c4fa67bde3ee8a832e0a2a1f47a471ce2ac00b0856e81b2acc61af0dc1b"
    }
}
```

See [MetaMask/eth-sig-util/sign-typed-data.ts](https://github.com/MetaMask/eth-sig-util/blob/main/src/sign-typed-data.ts) for a TypeScript EIP-712 message signing example.

## Step 9. Deploy test faucet contract

TODO: not working...

Deploy a simple test `GaslessFaucet` contract that is EIP-2771 compliant:

```bash
cd ${HOME}/avalanche-evm-gasless-transaction
forge create \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
src/GaslessFaucet.sol:GaslessFaucet \
--constructor-args 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922
```

Sample outputs are:

```bash
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0xA4cD3b0Eb6E5Ab5d8CE4065BcCD70040ADAB1F00
Transaction hash: ...
```

```bash
cd ${HOME}/avalanche-evm-gasless-transaction
./target/release/avalanche-evm-gasless-transaction \
gasless-faucet-withdraw \
--gas-relayer-server-rpc-url http://127.0.0.1:9876/rpc \
--chain-rpc-url http://127.0.0.1:9650/ext/bc/C/rpc \
--trusted-forwarder-contract-address 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 \
--recipient-contract-address 0xA4cD3b0Eb6E5Ab5d8CE4065BcCD70040ADAB1F00 \
--domain-name "my domain name" \
--domain-version "1" \
--type-name "my type name" \
--type-suffix-data "bytes8 typeSuffixDatadatadatada)" \
--withdraw-amount-in-hex "0x123456789"
```
