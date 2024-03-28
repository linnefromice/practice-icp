import { Principal } from '@dfinity/principal';
import { bootstrap, getAgent, gqlSdk, vaultActor } from './common';

bootstrap();

const execute = async () => {
  console.log('Hello World');

  const endpoint = process.env.GRAPHQL_ENDPOINT;
  const apiKey = process.env.AMPLIFY_API_KEY;
  if (!endpoint || !apiKey) {
    throw new Error('Missing environment variables');
  }

  const sdk = gqlSdk(endpoint, apiKey);
  const { components } = await sdk.ListComponents();
  console.log(components.items.length);

  const agent = getAgent();
  const res = await agent.status();
  console.log(res.ic_api_version, res.replica_health_status);

  const vaultPrincipal = Principal.fromText('3emnb-tiaaa-aaaal-qdrfq-cai');
  const vault = vaultActor(vaultPrincipal, agent);
  const metric = await vault.metric();
  console.log(metric);
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
