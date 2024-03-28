export const idlFactory = ({ IDL }) => {
  const Canister = IDL.Record({
    'principal' : IDL.Principal,
    'vault' : IDL.Principal,
  });
  const Time = IDL.Int;
  const CallLog = IDL.Record({
    'at' : Time,
    'interactTo' : IDL.Principal,
    'canister' : IDL.Principal,
  });
  const RegistryCanister = IDL.Service({
    'autoScaleServiceCanister' : IDL.Func([IDL.Text], [IDL.Text], []),
    'exists' : IDL.Func([IDL.Principal], [IDL.Bool], []),
    'getCanistersByPK' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Text)], ['query']),
    'getRegisteredCanister' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Canister)],
        [],
      ),
    'init' : IDL.Func([], [IDL.Vec(IDL.Opt(IDL.Text))], []),
    'listLogsOf' : IDL.Func(
        [IDL.Principal, Time, Time],
        [IDL.Vec(CallLog)],
        [],
      ),
    'putLog' : IDL.Func([IDL.Principal, IDL.Principal], [], []),
    'registerCanister' : IDL.Func([IDL.Principal, IDL.Principal], [], []),
    'scanCanisters' : IDL.Func([], [IDL.Vec(Canister)], []),
  });
  return RegistryCanister;
};
export const init = ({ IDL }) => { return []; };
