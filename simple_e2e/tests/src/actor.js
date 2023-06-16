import { Actor, HttpAgent } from "@dfinity/agent";
import {Ed25519KeyIdentity} from '@dfinity/identity';
import fetch from "isomorphic-fetch";
import canisterIds from "../../.dfx/local/canister_ids.json";
import { idlFactory as idlFactoryMotoko } from "../../src/declarations/hello_motoko/hello_motoko.did.js";
import { idlFactory as idlFactoryRust } from "../../src/declarations/hello_rust/hello_rust.did.js";

const createActorMotoko = async (canisterId, options) => {
    const agent = new HttpAgent({ ...options.agentOptions });
    await agent.fetchRootKey();

    return Actor.createActor(idlFactoryMotoko, {
        agent,
        canisterId,
        ...options?.actorOptions
    });
}
const createActorRust = async (canisterId, options) => {
    const agent = new HttpAgent({ ...options.agentOptions });
    await agent.fetchRootKey();

    return Actor.createActor(idlFactoryRust, {
        agent,
        canisterId,
        ...options?.actorOptions
    });
}

const URL = "http://127.0.0.1:4943";
const identity = Ed25519KeyIdentity.generate(new Uint8Array(Array.from({length: 32}).fill(0)));
const helloMotokoCanister = canisterIds.hello_motoko.local;
const helloRustCanister = canisterIds.hello_rust.local;

const helloMotoko = await createActorMotoko(helloMotokoCanister, {
    agentOptions: {
        host: URL,
        fetch,
        identity: identity,
    },
});
const helloRust = await createActorRust(helloRustCanister, {
    agentOptions: {
        host: URL,
        fetch,
        identity: identity,
    },
});

module.exports = {
    createActorMotoko,
    createActorRust,
    helloMotokoCanister,
    helloRustCanister,
    helloMotoko,
    helloRust,
}
