"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const aws_amplify_1 = require("aws-amplify");
const dotenv = require("dotenv");
const agent_1 = require("@dfinity/agent");
const principal_1 = require("@dfinity/principal");
const aws_exports_1 = require("./aws-exports");
const graphql_request_1 = require("graphql-request");
const graphql_1 = require("./gql/graphql");
const Vault = require("./did/_management_canister_vault");
aws_amplify_1.Amplify.configure(aws_exports_1.default);
dotenv.config();
const DFINITY_ENDPOINT = 'https://ic0.app/';
const getVault = (canister, agent) => {
    return Vault.createActor(canister, { agent });
};
const execute = async () => {
    console.log('Hello World');
    const endpoint = process.env.GRAPHQL_ENDPOINT;
    const apiKey = process.env.AMPLIFY_API_KEY;
    if (!endpoint || !apiKey) {
        throw new Error('Missing environment variables');
    }
    const client = new graphql_request_1.GraphQLClient(endpoint, {
        headers: {
            'X-Api-Key': apiKey,
        },
    });
    const sdk = (0, graphql_1.getSdk)(client);
    const { components } = await sdk.ListComponents();
    console.log(components.items.length);
    const agent = new agent_1.HttpAgent({ host: DFINITY_ENDPOINT });
    // await agent.fetchRootKey(); // for local
    const res = await agent.status();
    console.log(res.ic_api_version, res.replica_health_status);
    const vaultPrincipal = principal_1.Principal.fromText('3emnb-tiaaa-aaaal-qdrfq-cai');
    const vault = getVault(vaultPrincipal, agent);
    const metric = await vault.metric();
    console.log(metric);
};
execute()
    .then(() => {
    console.log('Execution completed');
})
    .catch((error) => {
    console.error(error);
});
//# sourceMappingURL=index.js.map