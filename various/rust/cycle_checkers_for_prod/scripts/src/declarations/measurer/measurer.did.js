export const idlFactory = ({ IDL }) => {
  const AccountInfo = IDL.Record({ 'address' : IDL.Text });
  return IDL.Service({
    'balance_of_erc20' : IDL.Func(
        [IDL.Text, IDL.Opt(IDL.Text), IDL.Opt(IDL.Text)],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'balance_of_native' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'get_ecdsa_public_key' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text })],
        [],
      ),
    'get_ethereum_address' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : AccountInfo, 'Err' : IDL.Text })],
        [],
      ),
    'get_gas_price' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'get_transaction_count' : IDL.Func(
        [IDL.Opt(IDL.Text)],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'sign_message' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Nat8)], []),
    'sign_transfer_erc20' : IDL.Func(
        [
          IDL.Text,
          IDL.Text,
          IDL.Nat64,
          IDL.Opt(IDL.Nat),
          IDL.Opt(IDL.Nat),
          IDL.Nat64,
        ],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'sign_transfer_native' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat), IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'transfer_erc20' : IDL.Func(
        [
          IDL.Text,
          IDL.Text,
          IDL.Nat64,
          IDL.Opt(IDL.Nat),
          IDL.Opt(IDL.Nat),
          IDL.Nat64,
        ],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'transfer_native' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat), IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
