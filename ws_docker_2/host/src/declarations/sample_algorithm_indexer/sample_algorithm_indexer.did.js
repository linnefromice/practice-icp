export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({ 'address' : IDL.Text });
  return IDL.Service({
    'between_account' : IDL.Func(
        [IDL.Tuple(IDL.Text, IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, Account))],
        ['query'],
      ),
    'get_account' : IDL.Func([IDL.Text], [IDL.Opt(Account)], ['query']),
    'last_account' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Text, Account))],
        ['query'],
      ),
    'proxy_between_account' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_get_account' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'proxy_last_account' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
