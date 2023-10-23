export const idlFactory = ({ IDL }) => {
  const Snapshot = IDL.Record({ 'value' : IDL.Text, 'timestamp' : IDL.Nat64 });
  const Web3AlgorithmIndexerSourceAttrs = IDL.Record({
    'chain_id' : IDL.Nat64,
    'function_name' : IDL.Text,
  });
  const SourceType = IDL.Variant({
    'evm' : IDL.Null,
    'https' : IDL.Null,
    'chainsight' : IDL.Null,
  });
  const Sources = IDL.Record({
    'source' : IDL.Text,
    'interval_sec' : IDL.Opt(IDL.Nat32),
    'attributes' : Web3AlgorithmIndexerSourceAttrs,
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
  const Web3CtxParam = IDL.Record({
    'env' : Env,
    'url' : IDL.Text,
    'from' : IDL.Opt(IDL.Text),
    'chain_id' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  const TransformArgs = IDL.Record({
    'context' : IDL.Vec(IDL.Nat8),
    'response' : HttpResponse,
  });
  return IDL.Service({
    'get_last_snapshot' : IDL.Func([], [Snapshot], ['query']),
    'get_last_snapshot_value' : IDL.Func([], [IDL.Text], ['query']),
    'get_proxy' : IDL.Func([], [IDL.Principal], []),
    'get_snapshot' : IDL.Func([IDL.Nat64], [Snapshot], ['query']),
    'get_snapshot_value' : IDL.Func([IDL.Nat64], [IDL.Text], ['query']),
    'get_snapshots' : IDL.Func([], [IDL.Vec(Snapshot)], ['query']),
    'get_sources' : IDL.Func([], [IDL.Vec(Sources)], ['query']),
    'get_top_snapshot_values' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
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
    'setup' : IDL.Func([IDL.Text, Web3CtxParam], [Result_1], []),
    'snapshots_len' : IDL.Func([], [IDL.Nat64], ['query']),
    'transform' : IDL.Func([TransformArgs], [HttpResponse], ['query']),
    'transform_get_filter_changes' : IDL.Func(
        [TransformArgs],
        [HttpResponse],
        ['query'],
      ),
    'transform_send_transaction' : IDL.Func(
        [TransformArgs],
        [HttpResponse],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
