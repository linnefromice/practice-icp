export const idlFactory = ({ IDL }) => {
  const SnapshotValue = IDL.Record({ 'dummy' : IDL.Nat64 });
  const Snapshot = IDL.Record({
    'value' : SnapshotValue,
    'timestamp' : IDL.Nat64,
  });
  const HttpsSnapshotIndexerSourceAttrs = IDL.Record({
    'queries' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const SourceType = IDL.Variant({
    'evm' : IDL.Null,
    'https' : IDL.Null,
    'chainsight' : IDL.Null,
  });
  const Sources = IDL.Record({
    'source' : IDL.Text,
    'interval_sec' : IDL.Opt(IDL.Nat32),
    'attributes' : HttpsSnapshotIndexerSourceAttrs,
    'source_type' : SourceType,
  });
  const Env = IDL.Variant({
    'Production' : IDL.Null,
    'Test' : IDL.Null,
    'LocalDevelopment' : IDL.Null,
  });
  const InitError = IDL.Variant({
    'InvalidDestination' : IDL.Text,
    'InvalidPrincipal' : IDL.Principal,
    'InvalidContent' : IDL.Text,
    'InvalidRequest' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : InitError });
  const CanisterMetricsSnapshot = IDL.Record({
    'cycles' : IDL.Nat,
    'timestamp' : IDL.Nat64,
  });
  return IDL.Service({
    'get_last_snapshot' : IDL.Func([], [Snapshot], ['query']),
    'get_last_snapshot_value' : IDL.Func([], [SnapshotValue], ['query']),
    'get_proxy' : IDL.Func([], [IDL.Principal], []),
    'get_snapshot' : IDL.Func([IDL.Nat64], [Snapshot], ['query']),
    'get_snapshot_value' : IDL.Func([IDL.Nat64], [SnapshotValue], ['query']),
    'get_snapshots' : IDL.Func([], [IDL.Vec(Snapshot)], ['query']),
    'get_sources' : IDL.Func([], [IDL.Vec(Sources)], ['query']),
    'get_top_snapshot_values' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(SnapshotValue)],
        ['query'],
      ),
    'get_top_snapshots' : IDL.Func([IDL.Nat64], [IDL.Vec(Snapshot)], ['query']),
    'init_in' : IDL.Func([Env], [Result], []),
    'last_executed' : IDL.Func([], [IDL.Nat64], ['query']),
    'metric' : IDL.Func([], [CanisterMetricsSnapshot], ['query']),
    'metrics' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(CanisterMetricsSnapshot)],
        ['query'],
      ),
    'proxy_canister_metrics_snapshots_len' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_canister_metrics_snapshot' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_canister_metrics_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_last_canister_metrics_snapshot' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_last_snapshot' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_last_snapshot_value' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_snapshot' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_snapshot_value' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_top_canister_metrics_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_top_snapshot_values' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_top_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_snapshots_len' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'set_task' : IDL.Func([IDL.Nat32, IDL.Nat32], [], []),
    'snapshots_len' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
