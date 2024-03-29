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
import { ComponentMetricsSnapshot } from './did/_management_canister_vault/_management_canister_vault.did';
import { time } from 'console';

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

// Model
export type Metric = {
  id: string;
  label: string;
  metric: ComponentMetricsSnapshot | null;
};
export type Extended = Metric & {
  metric_str: {
    cycles: string;
    timestamp: string;
  } | null;
};
const TIMESTAMP_DECIMALS = BigInt(1e6);
export const extendedMetric = (metric: Metric) => {
  if (!metric.metric) {
    return metric;
  }
  const { cycles, timestamp } = metric.metric;

  const epoch = Number(BigInt(timestamp) / TIMESTAMP_DECIMALS);
  const timestampStr = new Date(epoch).toISOString();

  return {
    ...metric,
    metric_str: {
      cycles: BigInt(cycles).toLocaleString('en-US').replace(/,/g, '_'),
      timestamp: timestampStr,
    },
  };
};

export type Snapshot = {
  id: string;
  intervalSec: number;
  snapshots: Array<{
    value: any; // temp
    timestamp: bigint;
  }>;
};
// todo: use length of snapshots
const extendSnapshot = (snapshot: Snapshot['snapshots'][0]) => {
  const timestampStr = new Date(
    Number(BigInt(snapshot.timestamp))
  ).toISOString();

  return {
    value: JSON.stringify(snapshot.value).replace(/,/g, '_'),
    timestamp: snapshot.timestamp,
    timestamp_str: timestampStr,
  };
};
export const extendedSnapshot = (snapshots: Snapshot['snapshots']) => {
  const nullVal = {
    value: '',
    timestamp: BigInt(0),
    timestamp_str: '',
  };

  return {
    snapshot_1: snapshots[0] ? extendSnapshot(snapshots[0]) : nullVal,
    snapshot2: snapshots[1] ? extendSnapshot(snapshots[1]) : nullVal,
  };
};

// File Operation
const OUTPUTS_PATH = path.join(process.cwd(), 'outputs');
export const PATH_COMPONENTS_JSON = path.join(OUTPUTS_PATH, 'components.json');
export const PATH_COMPONENTS_CSV = path.join(OUTPUTS_PATH, 'components.csv');
export const PATH_METRIC_JSON = path.join(OUTPUTS_PATH, 'metric.json');
export const PATH_METRIC_CSV = path.join(OUTPUTS_PATH, 'metric.csv');
export const PATH_METRIC_ERR_JSON = path.join(OUTPUTS_PATH, 'metric.err.json');
export const PATH_METRIC_ERR_CSV = path.join(OUTPUTS_PATH, 'metric.err.csv');
export const PATH_SNAPSHOTS_JSON = path.join(OUTPUTS_PATH, 'snapshots.json');
export const PATH_SNAPSHOTS_CSV = path.join(OUTPUTS_PATH, 'snapshots.csv');
export const PATH_SNAPSHOTS_ERR_JSON = path.join(
  OUTPUTS_PATH,
  'snapshots.err.json'
);
export const PATH_SNAPSHOTS_ERR_CSV = path.join(
  OUTPUTS_PATH,
  'snapshots.err.csv'
);
export const PATH_SUMMURY_CSV = path.join(OUTPUTS_PATH, 'summury.csv');

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

export const writeCsv = async (path: string, data: any) => {
  const csv = convertToCSV(data);
  fs.writeFileSync(path, csv, {
    encoding: 'utf8',
    flag: 'w',
  });
};

export const flattenObject = (obj: any) => {
  const toReturn = {} as Record<string, any>;

  for (const i in obj) {
    // eslint-disable-next-line no-prototype-builtins
    if (!obj.hasOwnProperty(i)) continue;

    if (
      typeof obj[i] === 'object' &&
      !Array.isArray(obj[i]) &&
      obj[i] !== null
    ) {
      const flatObject = flattenObject(obj[i]);
      for (const x in flatObject) {
        // eslint-disable-next-line no-prototype-builtins
        if (!flatObject.hasOwnProperty(x)) continue;
        toReturn[i + '_' + x] = flatObject[x];
      }
    } else {
      let val = obj[i];
      if (Array.isArray(val)) val = val.join('|');
      if (typeof val === 'string' || val instanceof String) {
        val = val.replace(',', '');
      }
      toReturn[i] = val;
    }
  }
  return toReturn;
};

const convertToCSV = (flattenObjectArray: any) => {
  const array =
    typeof flattenObjectArray !== 'object'
      ? JSON.parse(flattenObjectArray)
      : flattenObjectArray;
  let str = `${Object.keys(array[0]).join()},\r\n`;

  for (let i = 0; i < array.length; i++) {
    let line = '';
    for (const index in array[i]) {
      if (line !== '') line += ',';
      line += array[i][index];
    }

    str += line + '\r\n';
  }

  return str;
};
