import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterMetricsSnapshot {
  'cycles' : bigint,
  'timestamp' : bigint,
}
export type Env = { 'Production' : null } |
  { 'Test' : null } |
  { 'LocalDevelopment' : null };
export interface HttpsSnapshotIndexerSourceAttrs {
  'queries' : Array<[string, string]>,
}
export type InitError = { 'InvalidDestination' : string } |
  { 'InvalidPrincipal' : Principal } |
  { 'InvalidContent' : string } |
  { 'InvalidRequest' : string };
export type Result = { 'Ok' : null } |
  { 'Err' : InitError };
export interface Snapshot { 'value' : SnapshotValue, 'timestamp' : bigint }
export interface SnapshotValue { 'dummy' : bigint }
export type SourceType = { 'evm' : null } |
  { 'https' : null } |
  { 'chainsight' : null };
export interface Sources {
  'source' : string,
  'interval_sec' : [] | [number],
  'attributes' : HttpsSnapshotIndexerSourceAttrs,
  'source_type' : SourceType,
}
export interface _SERVICE {
  'get_last_snapshot' : ActorMethod<[], Snapshot>,
  'get_last_snapshot_value' : ActorMethod<[], SnapshotValue>,
  'get_proxy' : ActorMethod<[], Principal>,
  'get_snapshot' : ActorMethod<[bigint], Snapshot>,
  'get_snapshot_value' : ActorMethod<[bigint], SnapshotValue>,
  'get_snapshots' : ActorMethod<[], Array<Snapshot>>,
  'get_sources' : ActorMethod<[], Array<Sources>>,
  'get_top_snapshot_values' : ActorMethod<[bigint], Array<SnapshotValue>>,
  'get_top_snapshots' : ActorMethod<[bigint], Array<Snapshot>>,
  'init_in' : ActorMethod<[Env], Result>,
  'last_executed' : ActorMethod<[], bigint>,
  'metric' : ActorMethod<[], CanisterMetricsSnapshot>,
  'metrics' : ActorMethod<[bigint], Array<CanisterMetricsSnapshot>>,
  'proxy_canister_metrics_snapshots_len' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_canister_metrics_snapshot' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_canister_metrics_snapshots' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_last_canister_metrics_snapshot' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_last_snapshot' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_last_snapshot_value' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_snapshot' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_snapshot_value' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_snapshots' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_top_canister_metrics_snapshots' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_top_snapshot_values' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_top_snapshots' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_snapshots_len' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'set_task' : ActorMethod<[number, number], undefined>,
  'snapshots_len' : ActorMethod<[], bigint>,
}