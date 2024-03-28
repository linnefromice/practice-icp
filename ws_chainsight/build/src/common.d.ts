import { Principal } from '@dfinity/principal';
import { HttpAgent } from '@dfinity/agent';
export declare const bootstrap: () => void;
export declare const gqlSdk: (endpoint: string, apiKey: string) => {
    GetComponent(variables: import("./gql/graphql").Exact<{
        id: string;
    }>, requestHeaders?: import("graphql-request/build/esm/types").GraphQLClientRequestHeaders | undefined): Promise<import("./gql/graphql").GetComponentQuery>;
    ListComponents(variables?: import("./gql/graphql").Exact<{
        nextToken?: import("./gql/graphql").InputMaybe<string> | undefined;
    }> | undefined, requestHeaders?: import("graphql-request/build/esm/types").GraphQLClientRequestHeaders | undefined): Promise<import("./gql/graphql").ListComponentsQuery>;
};
export declare const getAgent: () => HttpAgent;
export declare const vaultActor: (canister: Principal, agent: HttpAgent) => import("@dfinity/agent").ActorSubclass<import("./did/_management_canister_vault/_management_canister_vault.did")._SERVICE>;
export declare const PATH_COMPONENTS: string;
export declare const PATH_METRIC: string;
export declare const writeJson: <T>(path: string, data: T) => Promise<void>;
export declare const loadJson: <T>(path: string) => Promise<T>;
