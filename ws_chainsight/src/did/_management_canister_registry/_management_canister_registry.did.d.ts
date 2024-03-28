import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CallLog {
  'at' : Time,
  'interactTo' : Principal,
  'canister' : Principal,
}
export interface Canister { 'principal' : Principal, 'vault' : Principal }
export interface RegistryCanister {
  'autoScaleServiceCanister' : ActorMethod<[string], string>,
  'exists' : ActorMethod<[Principal], boolean>,
  'getCanistersByPK' : ActorMethod<[string], Array<string>>,
  'getRegisteredCanister' : ActorMethod<[Principal], [] | [Canister]>,
  'init' : ActorMethod<[], Array<[] | [string]>>,
  'listLogsOf' : ActorMethod<[Principal, Time, Time], Array<CallLog>>,
  'putLog' : ActorMethod<[Principal, Principal], undefined>,
  'registerCanister' : ActorMethod<[Principal, Principal], undefined>,
  'scanCanisters' : ActorMethod<[], Array<Canister>>,
}
export type Time = bigint;
export interface _SERVICE extends RegistryCanister {}
