import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ComponentMetricsSnapshot {
  'cycles' : bigint,
  'timestamp' : bigint,
}
export interface CycleBalance { 'id' : Principal, 'amount' : bigint }
export interface RefuelTarget {
  'id' : Principal,
  'threshold' : bigint,
  'amount' : bigint,
}
export interface _SERVICE {
  'balance_of' : ActorMethod<[Principal], bigint>,
  'get_cumulative_refueled' : ActorMethod<[Principal], bigint>,
  'get_cumulative_refueled_all' : ActorMethod<[], Array<[Principal, bigint]>>,
  'get_cycle_balances' : ActorMethod<[], Array<CycleBalance>>,
  'get_refuel_targets' : ActorMethod<[], Array<RefuelTarget>>,
  'index' : ActorMethod<[], bigint>,
  'metric' : ActorMethod<[], ComponentMetricsSnapshot>,
  'metrics' : ActorMethod<[bigint], Array<ComponentMetricsSnapshot>>,
  'put_refuel_target' : ActorMethod<[RefuelTarget], undefined>,
  'receive_revenue' : ActorMethod<[], undefined>,
  'refuel' : ActorMethod<[], undefined>,
  'set_canister' : ActorMethod<[Principal], undefined>,
  'share_of' : ActorMethod<[Principal], bigint>,
  'supply' : ActorMethod<[[] | [Principal]], undefined>,
  'target_canister' : ActorMethod<[], Principal>,
  'total_supply' : ActorMethod<[], bigint>,
  'withdraw' : ActorMethod<[bigint], undefined>,
  'withdrawable_of' : ActorMethod<[Principal], bigint>,
}
