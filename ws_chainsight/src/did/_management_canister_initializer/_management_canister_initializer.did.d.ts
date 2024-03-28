import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CycleManagement {
  'refueling_amount' : bigint,
  'initial_supply' : bigint,
  'refueling_threshold' : bigint,
}
export interface CycleManagements {
  'db' : CycleManagement,
  'vault_intial_supply' : bigint,
  'refueling_interval' : bigint,
  'proxy' : CycleManagement,
  'indexer' : CycleManagement,
}
export interface InitializeOutput {
  'db' : Principal,
  'vault' : Principal,
  'proxy' : Principal,
}
export interface _SERVICE {
  'get_registry' : ActorMethod<[], Principal>,
  'initialize' : ActorMethod<[Principal, CycleManagements], InitializeOutput>,
  'set_registry' : ActorMethod<[Principal], undefined>,
  'upgrade_proxies' : ActorMethod<[], undefined>,
}
