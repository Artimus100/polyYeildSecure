require('dotenv').config();
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function main() {
    // Source chain connection
    const sourceProvider = new WsProvider(process.env.SOURCE_WS_PROVIDER_URL);
    const sourceApi = await ApiPromise.create({ provider: sourceProvider });

    // Target chain connection
    const targetProvider = new WsProvider(process.env.TARGET_WS_PROVIDER_URL);
    const targetApi = await ApiPromise.create({ provider: targetProvider });

    const keyring = new Keyring({ type: 'sr25519' });
    const relayerAccount = keyring.addFromUri(process.env.RELAYER_SEED);

    sourceApi.query.system.events(async (events) => {
        events.forEach(async (record) => {
            const { event, phase } = record;
            if (event.section === 'crossChain' && event.method === 'AssetLocked') {
                const [account, amount] = event.data;
                console.log(`AssetLocked: ${account} locked ${amount.toString()}`);

                // Implement the logic to initiate the transfer on the target blockchain
                try {
                    const unsub = await targetApi.tx.crossChain
                        .unlockAsset(account, amount)
                        .signAndSend(relayerAccount, (result) => {
                            console.log(`Current status is ${result.status}`);
                            if (result.status.isInBlock) {
                                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                            } else if (result.status.isFinalized) {
                                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                                unsub();
                            }
                        });
                } catch (error) {
                    console.error('Error sending transaction to target chain:', error);
                }
            }
        });
    });
}

main().catch(console.error);
