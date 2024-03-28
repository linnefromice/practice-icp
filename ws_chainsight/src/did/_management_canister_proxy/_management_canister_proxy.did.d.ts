import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CallLog {
  'at' : bigint,
  'interactTo' : Principal,
  'canister' : Principal,
}
export interface ComponentInfo {
  'db' : Principal,
  'vault' : Principal,
  'target' : Principal,
}
export interface Error { 'message' : string }
export interface ExecutionResult {
  'is_succeeded' : boolean,
  'error' : [] | [Error],
  'timestamp' : bigint,
}
export interface IndexingConfig {
  'method' : string,
  'args' : Uint8Array | number[],
  'task_interval_secs' : number,
  'delay_secs' : number,
  'is_rounded_start_time' : boolean,
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : [Uint8Array | number[]] } |
  { 'Err' : [RejectionCode, string] };
export interface _SERVICE {
  'db' : ActorMethod<[], Principal>,
  'get_component_info' : ActorMethod<[], ComponentInfo>,
  'get_indexing_config' : ActorMethod<[], IndexingConfig>,
  'initializer' : ActorMethod<[], Principal>,
  'last_execution_result' : ActorMethod<[], ExecutionResult>,
  'last_succeeded' : ActorMethod<[], bigint>,
  'list_logs' : ActorMethod<[Principal, bigint, bigint], Array<CallLog>>,
  'next_schedule' : ActorMethod<[], bigint>,
  'proxy_call' : ActorMethod<[string, Uint8Array | number[]], Result>,
  'registry' : ActorMethod<[], Principal>,
  'request_upgrades_to_registry' : ActorMethod<[], undefined>,
  'restart_indexing' : ActorMethod<[], undefined>,
  'set_registry' : ActorMethod<[Principal], undefined>,
  'start_indexing' : ActorMethod<
    [number, number, string, Uint8Array | number[]],
    undefined
  >,
  'start_indexing_with_is_rounded' : ActorMethod<
    [number, number, boolean, string, Uint8Array | number[]],
    undefined
  >,
  'target' : ActorMethod<[], Principal>,
  'vault' : ActorMethod<[], Principal>,
}
