export const idlFactory = ({ IDL }) => {
  const RefuelTarget = IDL.Record({
    'id' : IDL.Principal,
    'threshold' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  const CycleBalance = IDL.Record({ 'id' : IDL.Principal, 'amount' : IDL.Nat });
  const ComponentMetricsSnapshot = IDL.Record({
    'cycles' : IDL.Nat,
    'timestamp' : IDL.Nat64,
  });
  return IDL.Service({
    'balance_of' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'get_cumulative_refueled' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'get_cumulative_refueled_all' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))],
        ['query'],
      ),
    'get_cycle_balances' : IDL.Func([], [IDL.Vec(CycleBalance)], []),
    'get_refuel_targets' : IDL.Func([], [IDL.Vec(RefuelTarget)], ['query']),
    'index' : IDL.Func([], [IDL.Nat], ['query']),
    'metric' : IDL.Func([], [ComponentMetricsSnapshot], ['query']),
    'metrics' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(ComponentMetricsSnapshot)],
        ['query'],
      ),
    'put_refuel_target' : IDL.Func([RefuelTarget], [], []),
    'receive_revenue' : IDL.Func([], [], []),
    'refuel' : IDL.Func([], [], []),
    'set_canister' : IDL.Func([IDL.Principal], [], []),
    'share_of' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'supply' : IDL.Func([IDL.Opt(IDL.Principal)], [], []),
    'target_canister' : IDL.Func([], [IDL.Principal], ['query']),
    'total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'withdraw' : IDL.Func([IDL.Nat], [], []),
    'withdrawable_of' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
  });
};
export const init = ({ IDL }) => {
  const RefuelTarget = IDL.Record({
    'id' : IDL.Principal,
    'threshold' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  return [
    IDL.Principal,
    IDL.Principal,
    IDL.Nat,
    IDL.Nat64,
    IDL.Vec(RefuelTarget),
    IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
  ];
};
