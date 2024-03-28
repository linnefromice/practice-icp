import * as fs from 'fs';
import * as path from 'path';

import { GraphQLClient } from 'graphql-request';
import { Amplify } from 'aws-amplify';
import * as dotenv from 'dotenv';

import AwsExports from './aws-exports';
import { getSdk } from './gql/graphql';
import * as Vault from './did/_management_canister_vault';
import { Principal } from '@dfinity/principal';
import { HttpAgent } from '@dfinity/agent';

const DFINITY_ENDPOINT = 'https://ic0.app/';

export const bootstrap = () => {
  Amplify.configure(AwsExports);
  dotenv.config();
};

// GraphQL
export const gqlSdk = (endpoint: string, apiKey: string) => {
  const client = new GraphQLClient(endpoint, {
    headers: {
      'X-Api-Key': apiKey,
    },
  });
  return getSdk(client);
};

// IC
export const getAgent = () => {
  const agent = new HttpAgent({ host: DFINITY_ENDPOINT });
  // await agent.fetchRootKey(); // for local
  return agent;
};

export const vaultActor = (canister: Principal, agent: HttpAgent) => {
  return Vault.createActor(canister, { agent });
};

// File Operation
const OUTPUTS_PATH = path.join(process.cwd(), 'outputs');
export const PATH_COMPONENTS = path.join(OUTPUTS_PATH, 'components.json');
export const PATH_METRIC = path.join(OUTPUTS_PATH, 'metric.json');

export const writeJson = async <T>(path: string, data: T) => {
  const json = JSON.stringify(data, null, 2);
  fs.writeFileSync(path, json, {
    encoding: 'utf8',
    flag: 'w',
  });
};

export const loadJson = async <T>(path: string): Promise<T> => {
  const text = fs.readFileSync(path, { encoding: 'utf8' });
  return JSON.parse(text) as T;
};
