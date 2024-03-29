import { Actor, ActorSubclass, type ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

import {
  PATH_COMPONENTS_JSON,
  PATH_SNAPSHOTS_CSV,
  PATH_SNAPSHOTS_JSON,
  bootstrap,
  flattenObject,
  getAgent,
  loadJson,
  writeJson,
} from '../common';
import { ListComponentsQuery } from '../gql/graphql';

// NOTE: TypeError: Do not know how to serialize a BigInt
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(BigInt.prototype as any).toJSON = function () {
  return this.toString();
};

bootstrap();

interface Snapshot {
  value: any; // string; (if scalar)
  timestamp: bigint;
}
interface _SERVICE {
  get_last_snapshot: ActorMethod<[], Snapshot>;
  get_top_snapshots: ActorMethod<[bigint], Array<Snapshot>>;
}
const idlFactory: IDL.InterfaceFactory = ({ IDL }) => {
  const Snapshot = IDL.Record({
    value: IDL.Unknown, // IDL.Text (if scalar)
    timestamp: IDL.Nat64,
  });
  return IDL.Service({
    get_last_snapshot: IDL.Func([], [Snapshot], ['query']),
    get_top_snapshots: IDL.Func([IDL.Nat64], [IDL.Vec(Snapshot)], ['query']),
  });
};

export const execute = async () => {
  const agent = getAgent();

  const components =
    await loadJson<ListComponentsQuery['components']['items']>(
      PATH_COMPONENTS_JSON
    );

  const snapshots = components.filter(c =>
    c.componentType.startsWith('SnapshotIndexer')
  ); // NOTE: Unknown is not filtered because it cannot be determined without a request.
  console.log(`targets: ${snapshots.length}`);

  const result = [];
  const errs: { id: string; error: string }[] = [];
  for (const snapshot of snapshots) {
    const actor = Actor.createActor(idlFactory, {
      agent,
      canisterId: snapshot.id,
    }) as ActorSubclass<_SERVICE>;
    const data = await actor.get_top_snapshots(BigInt(2)).catch((e: any) => {
      errs.push({ id: snapshot.id, error: e.toString() });
      return [];
    });
    const raw = {
      id: snapshot.id,
      intervalSec: snapshot.intervalSec,
      snapshots: data,
    };
    result.push(raw);
  }

  console.log(`result: ${result.length}`);
  console.log(`errors: ${errs.length}`);
  writeJson(PATH_SNAPSHOTS_JSON, result);
  writeJson(PATH_SNAPSHOTS_CSV, result.map(flattenObject));
  writeJson(PATH_SNAPSHOTS_JSON, errs);
  writeJson(PATH_SNAPSHOTS_CSV, errs.map(flattenObject));
  console.log(JSON.stringify(errs, null, 2));
};

// For Example
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const example = async () => {
  const agent = getAgent();

  const canisters = [
    'voxnn-byaaa-aaaao-a3jtq-cai', // BTC/USD price from Chainlink
    'xvisy-nqaaa-aaaal-qdpoq-cai', //  ETH/USD price data from Chainlink Aggregator Contract
  ];
  for (const canister of canisters) {
    const actor = Actor.createActor(idlFactory, {
      agent,
      canisterId: canister,
    }) as ActorSubclass<_SERVICE>;
    console.log(await actor.get_last_snapshot());
  }
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((e: unknown) => {
    console.error(e);
  });
