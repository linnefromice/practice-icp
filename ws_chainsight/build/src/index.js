"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const principal_1 = require("@dfinity/principal");
const common_1 = require("./common");
(0, common_1.bootstrap)();
const execute = async () => {
    console.log('Hello World');
    const endpoint = process.env.GRAPHQL_ENDPOINT;
    const apiKey = process.env.AMPLIFY_API_KEY;
    if (!endpoint || !apiKey) {
        throw new Error('Missing environment variables');
    }
    const sdk = (0, common_1.gqlSdk)(endpoint, apiKey);
    const { components } = await sdk.ListComponents();
    console.log(components.items.length);
    const agent = (0, common_1.getAgent)();
    const res = await agent.status();
    console.log(res.ic_api_version, res.replica_health_status);
    const vaultPrincipal = principal_1.Principal.fromText('3emnb-tiaaa-aaaal-qdrfq-cai');
    const vault = (0, common_1.vaultActor)(vaultPrincipal, agent);
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