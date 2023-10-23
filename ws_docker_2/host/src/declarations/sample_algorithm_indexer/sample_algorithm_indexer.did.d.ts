import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account { 'address' : string }
export interface _SERVICE {
  'between_account' : ActorMethod<[[string, string]], Array<[string, Account]>>,
  'get_account' : ActorMethod<[string], [] | [Account]>,
  'last_account' : ActorMethod<[bigint], Array<[string, Account]>>,
  'proxy_between_account' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_get_account' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'proxy_last_account' : ActorMethod<
    [Uint8Array | number[]],
    Uint8Array | number[]
  >,
}
