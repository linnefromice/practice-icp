import {Amplify} from 'aws-amplify';
import * as dotenv from 'dotenv';
import {HttpAgent} from '@dfinity/agent';
import {Principal} from '@dfinity/principal';

import AwsExports from './aws-exports';
import {GraphQLClient} from 'graphql-request';
import {getSdk} from './gql/graphql';

import * as Vault from './did/_management_canister_vault';

Amplify.configure(AwsExports);

dotenv.config();

const DFINITY_ENDPOINT = 'https://ic0.app/';

const getVault = (canister: Principal, agent: HttpAgent) => {
  return Vault.createActor(canister, {agent});
};

const execute = async () => {
  console.log('Hello World');

  const endpoint = process.env.GRAPHQL_ENDPOINT;
  const apiKey = process.env.AMPLIFY_API_KEY;
  if (!endpoint || !apiKey) {
    throw new Error('Missing environment variables');
  }

  const client = new GraphQLClient(endpoint, {
    headers: {
      'X-Api-Key': apiKey,
    },
  });
  const sdk = getSdk(client);
  const {components} = await sdk.ListComponents();
  console.log(components.items.length);

  const agent = new HttpAgent({host: DFINITY_ENDPOINT});
  // await agent.fetchRootKey(); // for local
  const res = await agent.status();
  console.log(res.ic_api_version, res.replica_health_status);

  const vaultPrincipal = Principal.fromText('3emnb-tiaaa-aaaal-qdrfq-cai');
  const vault = getVault(vaultPrincipal, agent);
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
