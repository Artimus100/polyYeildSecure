const { ApiPromise } = require('@polkadot/api');

// Function to log metadata
async function logMetadata(api) {
    try {
        const metadata = await api.rpc.state.getMetadata();
        console.log('Raw Metadata:', metadata);
    } catch (error) {
        console.error('Error fetching metadata:', error);
    }
}

// Function to decode metadata
async function decodeMetadata(api) {
    try {
        const metadata = await api.rpc.state.getMetadata();
        const decodedMetadata = api.createType('Metadata', metadata);
        console.log('Decoded Metadata:', decodedMetadata.toJSON());
    } catch (error) {
        console.error('Error decoding metadata:', error);
    }
}

module.exports = {
    logMetadata,
    decodeMetadata,
};
