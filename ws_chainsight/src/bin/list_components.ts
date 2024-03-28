import { PATH_COMPONENTS, bootstrap, gqlSdk, writeJson } from '../common';

bootstrap();

const execute = async () => {
  const endpoint = process.env.GRAPHQL_ENDPOINT;
  const apiKey = process.env.AMPLIFY_API_KEY;
  if (!endpoint || !apiKey) {
    throw new Error('Missing environment variables');
  }
  const sdk = gqlSdk(endpoint, apiKey);
  const { components } = await sdk.ListComponents();

  writeJson(PATH_COMPONENTS, components.items);
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
