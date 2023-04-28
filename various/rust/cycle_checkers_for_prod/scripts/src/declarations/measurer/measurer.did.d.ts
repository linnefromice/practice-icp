import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountInfo { 'address' : string }
export interface _SERVICE {
  'balance_of_erc20' : ActorMethod<
    [string, [] | [string], [] | [string]],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'balance_of_native' : ActorMethod<[], { 'Ok' : string } | { 'Err' : string }>,
  'get_ecdsa_public_key' : ActorMethod<
    [],
    { 'Ok' : Uint8Array | number[] } |
      { 'Err' : string }
  >,
  'get_ethereum_address' : ActorMethod<
    [],
    { 'Ok' : AccountInfo } |
      { 'Err' : string }
  >,
  'get_gas_price' : ActorMethod<[], { 'Ok' : string } | { 'Err' : string }>,
  'get_transaction_count' : ActorMethod<
    [[] | [string]],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'sign_message' : ActorMethod<[string], Uint8Array | number[]>,
  'sign_transfer_erc20' : ActorMethod<
    [string, string, bigint, [] | [bigint], [] | [bigint], bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'sign_transfer_native' : ActorMethod<
    [string, bigint, [] | [bigint], [] | [bigint], bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'transfer_erc20' : ActorMethod<
    [string, string, bigint, [] | [bigint], [] | [bigint], bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'transfer_native' : ActorMethod<
    [string, bigint, [] | [bigint], [] | [bigint], bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
}
