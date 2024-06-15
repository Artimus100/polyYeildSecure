require('dotenv').config();
const Web3 = require('web3');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

// Ethereum setup
const web3 = new Web3(process.env.ETH_NODE_URL);
const ethContractAddress = process.env.ETH_CONTRACT_ADDRESS;
const ethAbi = JSON.parse(process.env.ETH_CONTRACT_ABI);

// Substrate setup
const substrateProvider = new WsProvider(process.env.SUBSTRATE_NODE_URL);
const keyring = new Keyring({ type: 'sr25519' });
const substrateAccount = keyring.addFromUri(process.env.SUBSTRATE_ACCOUNT_URI);

async function listenToEthereum() {
    const ethContract = new web3.eth.Contract(ethAbi, ethContractAddress);

    ethContract.events.ValueChanged()
        .on('data', async (event) => {
            const newValue = event.returnValues.newValue;
            console.log(`New value from Ethereum: ${newValue}`);

            const api = await ApiPromise.create({ provider: substrateProvider });

            // Interact with Substrate contract
            await api.tx.myContractModule
                .set(newValue)
                .signAndSend(substrateAccount, ({ status }) => {
                    if (status.isInBlock) {
                        console.log(`Transaction included at blockHash ${status.asInBlock}`);
                    }
                });
        })
        .on('error', console.error);
}

listenToEthereum();
