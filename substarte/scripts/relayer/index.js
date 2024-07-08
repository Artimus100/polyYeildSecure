require('dotenv').config();
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    try {
        console.log('Connecting to source chain...');
        const sourceProvider = new WsProvider(process.env.SOURCE_WS_PROVIDER_URL);
        const sourceApi = await ApiPromise.create({ provider: sourceProvider });

        // Fetch metadata
        const metadata = await sourceApi.rpc.state.getMetadata();

        // Decode metadata
        const decodedMetadata = metadata.asLatest;
        console.log('Decoded Metadata:', decodedMetadata);

        // Example: Query system events
        sourceApi.query.system.events(async (events) => {
            events.forEach((record) => {
                const { event } = record;
                if (event.section === 'system' && event.method === 'ExtrinsicSuccess') {
                    console.log('Extrinsic success event detected:', event);
                    const eventData = event.data.toJSON();
                    // Perform actions based on the event data
                    if (eventData.length > 0) {
                        const accountId = eventData[0];
                        console.log(`Account ${accountId} successfully executed an extrinsic.`);
                        // Example: Trigger another transaction based on the event
                        triggerTransaction(sourceApi, accountId);
                    }
                }
            });
        });

        sourceApi.on('disconnected', () => {
            console.error('Disconnected from source chain');
        });

    } catch (error) {
        console.error('Error initializing the API or fetching metadata:', error);
    }
}

async function triggerTransaction(api, accountId) {
    try {
        // Example: Construct and send a transaction to another chain
        const txHash = await api.tx.exampleModule.exampleFunction(accountId).signAndSend();
        console.log(`Transaction sent with hash: ${txHash}`);
    } catch (error) {
        console.error('Error triggering transaction:', error);
    }
}

main().catch(console.error);
