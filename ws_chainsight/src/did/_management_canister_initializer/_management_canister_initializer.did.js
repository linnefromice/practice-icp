export const idlFactory = ({ IDL }) => {
  const CycleManagement = IDL.Record({
    'refueling_amount' : IDL.Nat,
    'initial_supply' : IDL.Nat,
    'refueling_threshold' : IDL.Nat,
  });
  const CycleManagements = IDL.Record({
    'db' : CycleManagement,
    'vault_intial_supply' : IDL.Nat,
    'refueling_interval' : IDL.Nat64,
    'proxy' : CycleManagement,
    'indexer' : CycleManagement,
  });
  const InitializeOutput = IDL.Record({
    'db' : IDL.Principal,
    'vault' : IDL.Principal,
    'proxy' : IDL.Principal,
  });
  return IDL.Service({
    'get_registry' : IDL.Func([], [IDL.Principal], ['query']),
    'initialize' : IDL.Func(
        [IDL.Principal, CycleManagements],
        [InitializeOutput],
        [],
      ),
    'set_registry' : IDL.Func([IDL.Principal], [], []),
    'upgrade_proxies' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
