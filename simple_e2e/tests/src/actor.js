import { Actor, HttpAgent } from "@dfinity/agent";
import {Ed25519KeyIdentity} from '@dfinity/identity';
import fetch from "isomorphic-fetch";
import canisterIds from "../../.dfx/local/canister_ids.json";
import { idlFactory as idlFactoryMotoko } from "../../src/declarations/hello_motoko/hello_motoko.did.js";
import { idlFactory as idlFactoryRust } from "../../src/declarations/hello_rust/hello_rust.did.js";
import { idlFactory as idlFactoryCounterMotoko } from "../../src/declarations/counter_motoko/counter_motoko.did.js";
import { idlFactory as idlFactoryCounterRust } from "../../src/declarations/counter_rust/counter_rust.did.js";

const createActor = async (canisterId, options, idlFactory) => {
    const agent = new HttpAgent({ ...options.agentOptions });
    await agent.fetchRootKey();

    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options?.actorOptions
    });
}

const URL = "http://127.0.0.1:4943";
const identity = Ed25519KeyIdentity.generate(new Uint8Array(Array.from({length: 32}).fill(0)));

const createActorHelloMotoko = async (canisterId, options) => createActor(canisterId, options, idlFactoryMotoko);
const createActorHelloRust = async (canisterId, options) => createActor(canisterId, options, idlFactoryRust);
const createActorCounterMotoko = async (canisterId, options) => createActor(canisterId, options, idlFactoryCounterMotoko);
const createActorCounterRust = async (canisterId, options) => createActor(canisterId, options, idlFactoryCounterRust);

const helloMotokoCanister = canisterIds.hello_motoko.local;
const helloRustCanister = canisterIds.hello_rust.local;
const counterMotokoCanister = canisterIds.counter_motoko.local;
const counterRustCanister = canisterIds.counter_rust.local;

const commonAgentOption = {
    host: URL,
    fetch,
    identity: identity,
};
const helloMotoko = await createActorHelloMotoko(
    helloMotokoCanister,
    { agentOptions: commonAgentOption }
);
const helloRust = await createActorHelloRust(
    helloRustCanister,
    { agentOptions: commonAgentOption }
);
const counterMotoko = await createActorCounterMotoko(
    counterMotokoCanister,
    { agentOptions: commonAgentOption }
);
const counterRust = await createActorCounterRust(
    counterRustCanister,
    { agentOptions: commonAgentOption }
);

module.exports = {
    helloMotoko,
    helloRust,
    counterMotoko,
    counterRust,
}
