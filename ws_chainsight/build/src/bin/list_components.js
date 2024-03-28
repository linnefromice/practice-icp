"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("../common");
(0, common_1.bootstrap)();
const execute = async () => {
    const endpoint = process.env.GRAPHQL_ENDPOINT;
    const apiKey = process.env.AMPLIFY_API_KEY;
    if (!endpoint || !apiKey) {
        throw new Error('Missing environment variables');
    }
    const sdk = (0, common_1.gqlSdk)(endpoint, apiKey);
    const { components } = await sdk.ListComponents();
    (0, common_1.writeJson)(common_1.PATH_COMPONENTS, components.items);
};
execute()
    .then(() => {
    console.log('Execution completed');
})
    .catch((error) => {
    console.error(error);
});
//# sourceMappingURL=list_components.js.map