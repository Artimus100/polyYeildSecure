# Relayer Service

This relayer service facilitates cross-chain communication between Ethereum and Substrate.

## Setup

1. Install dependencies:

    ```bash
    npm install
    ```

2. Configure environment variables in the `.env` file.

3. Start the relayer service:

    ```bash
    npm start
    ```

## Functionality

- Listens for `ValueChanged` events from the Ethereum contract.
- Triggers corresponding actions on the Substrate blockchain.
