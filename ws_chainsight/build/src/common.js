"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.loadJson = exports.writeJson = exports.PATH_METRIC = exports.PATH_COMPONENTS = exports.vaultActor = exports.getAgent = exports.gqlSdk = exports.bootstrap = void 0;
const fs = require("fs");
const path = require("path");
const graphql_request_1 = require("graphql-request");
const aws_amplify_1 = require("aws-amplify");
const dotenv = require("dotenv");
const aws_exports_1 = require("./aws-exports");
const graphql_1 = require("./gql/graphql");
const Vault = require("./did/_management_canister_vault");
const agent_1 = require("@dfinity/agent");
const DFINITY_ENDPOINT = 'https://ic0.app/';
const bootstrap = () => {
    aws_amplify_1.Amplify.configure(aws_exports_1.default);
    dotenv.config();
};
exports.bootstrap = bootstrap;
// GraphQL
const gqlSdk = (endpoint, apiKey) => {
    const client = new graphql_request_1.GraphQLClient(endpoint, {
        headers: {
            'X-Api-Key': apiKey,
        },
    });
    return (0, graphql_1.getSdk)(client);
};
exports.gqlSdk = gqlSdk;
// IC
const getAgent = () => {
    const agent = new agent_1.HttpAgent({ host: DFINITY_ENDPOINT });
    // await agent.fetchRootKey(); // for local
    return agent;
};
exports.getAgent = getAgent;
const vaultActor = (canister, agent) => {
    return Vault.createActor(canister, { agent });
};
exports.vaultActor = vaultActor;
// File Operation
const OUTPUTS_PATH = path.join(process.cwd(), 'outputs');
exports.PATH_COMPONENTS = path.join(OUTPUTS_PATH, 'components.json');
exports.PATH_METRIC = path.join(OUTPUTS_PATH, 'metric.json');
const writeJson = async (path, data) => {
    const json = JSON.stringify(data, null, 2);
    fs.writeFileSync(path, json, {
        encoding: 'utf8',
        flag: 'w',
    });
};
exports.writeJson = writeJson;
const loadJson = async (path) => {
    const text = fs.readFileSync(path, { encoding: 'utf8' });
    return JSON.parse(text);
};
exports.loadJson = loadJson;
//# sourceMappingURL=common.js.map