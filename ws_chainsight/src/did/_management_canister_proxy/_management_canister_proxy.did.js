export const idlFactory = ({ IDL }) => {
  const ComponentInfo = IDL.Record({
    'db' : IDL.Principal,
    'vault' : IDL.Principal,
    'target' : IDL.Principal,
  });
  const IndexingConfig = IDL.Record({
    'method' : IDL.Text,
    'args' : IDL.Vec(IDL.Nat8),
    'task_interval_secs' : IDL.Nat32,
    'delay_secs' : IDL.Nat32,
    'is_rounded_start_time' : IDL.Bool,
  });
  const Error = IDL.Record({ 'message' : IDL.Text });
  const ExecutionResult = IDL.Record({
    'is_succeeded' : IDL.Bool,
    'error' : IDL.Opt(Error),
    'timestamp' : IDL.Nat64,
  });
  const CallLog = IDL.Record({
    'at' : IDL.Int,
    'interactTo' : IDL.Principal,
    'canister' : IDL.Principal,
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Tuple(IDL.Vec(IDL.Nat8)),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  return IDL.Service({
    'db' : IDL.Func([], [IDL.Principal], ['query']),
    'get_component_info' : IDL.Func([], [ComponentInfo], ['query']),
    'get_indexing_config' : IDL.Func([], [IndexingConfig], ['query']),
    'initializer' : IDL.Func([], [IDL.Principal], ['query']),
    'last_execution_result' : IDL.Func([], [ExecutionResult], ['query']),
    'last_succeeded' : IDL.Func([], [IDL.Nat64], ['query']),
    'list_logs' : IDL.Func(
        [IDL.Principal, IDL.Int, IDL.Int],
        [IDL.Vec(CallLog)],
        [],
      ),
    'next_schedule' : IDL.Func([], [IDL.Nat64], ['query']),
    'proxy_call' : IDL.Func([IDL.Text, IDL.Vec(IDL.Nat8)], [Result], []),
    'registry' : IDL.Func([], [IDL.Principal], ['query']),
    'request_upgrades_to_registry' : IDL.Func([], [], []),
    'restart_indexing' : IDL.Func([], [], []),
    'set_registry' : IDL.Func([IDL.Principal], [], []),
    'start_indexing' : IDL.Func(
        [IDL.Nat32, IDL.Nat32, IDL.Text, IDL.Vec(IDL.Nat8)],
        [],
        [],
      ),
    'start_indexing_with_is_rounded' : IDL.Func(
        [IDL.Nat32, IDL.Nat32, IDL.Bool, IDL.Text, IDL.Vec(IDL.Nat8)],
        [],
        [],
      ),
    'target' : IDL.Func([], [IDL.Principal], ['query']),
    'vault' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Principal, IDL.Principal, IDL.Principal, IDL.Principal];
};
