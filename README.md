
### Avalanche EVM gasless transaction

[ERC-2771](https://eips.ethereum.org/EIPS/eip-2771) (gasless transaction) on Avalanche EVM chains.

#### Step 1. Install dependencies

Install [Foundry](https://github.com/foundry-rs/foundry#installation), and make sure the following works:

```bash
forge --version
cast --version
```

#### Step 2. Download trusted forwarder contract

```bash
git clone git@github.com:ava-labs/avalanche-evm-gasless-transaction.git

cd ${HOME}/avalanche-evm-gasless-transaction
git submodule update --init --recursive
```

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
Transaction hash: 0xeeb19e22eddd90afe88e9dcbd672338c67b06f8549d96f7847efb46e103f534e
```

In the example above, the trusted forwarder contract address is `0x52C84043CD9c865236f11d9Fc9F56aa003c1f922`, which will be required for the following steps (domain separator, type name registration, etc.).

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
