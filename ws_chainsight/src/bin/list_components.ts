import {
  PATH_COMPONENTS_CSV,
  PATH_COMPONENTS_JSON,
  bootstrap,
  flattenObject,
  gqlSdk,
  writeJson,
} from '../common';

bootstrap();

export const execute = async () => {
  const endpoint = process.env.GRAPHQL_ENDPOINT;
  const apiKey = process.env.AMPLIFY_API_KEY;
  if (!endpoint || !apiKey) {
    throw new Error('Missing environment variables');
  }
  const sdk = gqlSdk(endpoint, apiKey);
  const { components } = await sdk.ListComponents();

  writeJson(PATH_COMPONENTS_JSON, components.items);
  writeJson(PATH_COMPONENTS_CSV, components.items.map(flattenObject));
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
