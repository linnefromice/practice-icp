import { GraphQLClient, RequestOptions } from 'graphql-request';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends {
    [key: string]: unknown;
}> = {
    [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
    [SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
    [SubKey in K]: Maybe<T[SubKey]>;
};
export type MakeEmpty<T extends {
    [key: string]: unknown;
}, K extends keyof T> = {
    [_ in K]?: never;
};
export type Incremental<T> = T | {
    [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never;
};
type GraphQLClientRequestHeaders = RequestOptions['requestHeaders'];
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
    ID: {
        input: string;
        output: string;
    };
    String: {
        input: string;
        output: string;
    };
    Boolean: {
        input: boolean;
        output: boolean;
    };
    Int: {
        input: number;
        output: number;
    };
    Float: {
        input: number;
        output: number;
    };
};
export type AcceptDeploymentReceipt = {
    __typename?: 'AcceptDeploymentReceipt';
    accepted: Scalars['Boolean']['output'];
    causedBy?: Maybe<Scalars['String']['output']>;
    chainId?: Maybe<Scalars['Int']['output']>;
    id?: Maybe<Scalars['ID']['output']>;
    rpcfUrl: Scalars['String']['output'];
};
export type AddToMyComponentsInput = {
    canisterId: Scalars['ID']['input'];
    id: Scalars['ID']['input'];
};
export type AssetDataStore = {
    __typename?: 'AssetDataStore';
    component: Component;
    description?: Maybe<Scalars['String']['output']>;
    feeds: AssetFeeds;
    id: Scalars['String']['output'];
    symbol: Scalars['String']['output'];
};
export type AssetFeed = {
    __typename?: 'AssetFeed';
    dataStore: AssetDataStore;
    feeder: AssetFeeder;
    id: Scalars['String']['output'];
};
export type AssetFeeder = {
    __typename?: 'AssetFeeder';
    chainId: Scalars['Int']['output'];
    component: Component;
    description?: Maybe<Scalars['String']['output']>;
    duration: Scalars['Int']['output'];
    id: Scalars['String']['output'];
};
export type AssetFeeds = {
    __typename?: 'AssetFeeds';
    feeds: Array<AssetFeed>;
};
export type Component = {
    __typename?: 'Component';
    componentType: ComponentType;
    controllers: Array<Scalars['String']['output']>;
    description: Scalars['String']['output'];
    id: Scalars['ID']['output'];
    intervalSec?: Maybe<Scalars['Int']['output']>;
    label: Scalars['String']['output'];
    proxy: Scalars['String']['output'];
    relayerMetadata?: Maybe<RelayerMetadata>;
    sources: Array<ComponentSource>;
    tags: Array<Scalars['String']['output']>;
    vault: Scalars['String']['output'];
};
export type ComponentResponse = {
    __typename?: 'ComponentResponse';
    items: Array<Component>;
    nextToken?: Maybe<Scalars['String']['output']>;
};
export type ComponentSource = {
    __typename?: 'ComponentSource';
    attributes: ComponentSourceAttribute;
    source: SourceLike;
    sourceType: ComponentSourceType;
};
export type ComponentSourceAttribute = {
    __typename?: 'ComponentSourceAttribute';
    chainId?: Maybe<Scalars['Int']['output']>;
    contractType?: Maybe<Scalars['String']['output']>;
    eventName?: Maybe<Scalars['String']['output']>;
    functionName?: Maybe<Scalars['String']['output']>;
    queries?: Maybe<Array<Maybe<KeyValue>>>;
    sources?: Maybe<Array<Maybe<Component>>>;
};
export declare enum ComponentSourceType {
    Chainsight = "Chainsight",
    Evm = "Evm",
    Https = "Https",
    Icp = "Icp"
}
export declare enum ComponentType {
    AlgorithmIndexer = "AlgorithmIndexer",
    AlgorithmLens = "AlgorithmLens",
    EventIndexer = "EventIndexer",
    Relayer = "Relayer",
    SnapshotIndexerEvm = "SnapshotIndexerEvm",
    SnapshotIndexerHttps = "SnapshotIndexerHttps",
    SnapshotIndexerIcp = "SnapshotIndexerIcp"
}
export type DeleteFeedInput = {
    id: Scalars['String']['input'];
};
export type DeployRelayerInput = {
    rpcUrl: Scalars['String']['input'];
    symbol: Scalars['String']['input'];
};
export type DeployRelayerOutput = {
    __typename?: 'DeployRelayerOutput';
    accepted: Scalars['Boolean']['output'];
    causedBy?: Maybe<Scalars['String']['output']>;
    id: Scalars['ID']['output'];
};
export type KeyValue = {
    __typename?: 'KeyValue';
    key: Scalars['String']['output'];
    value: Scalars['String']['output'];
};
export type Layout = {
    __typename?: 'Layout';
    data: Scalars['String']['output'];
};
export type ListAssetDataStoresInput = {
    limit?: InputMaybe<Scalars['Int']['input']>;
    nextToken?: InputMaybe<Scalars['String']['input']>;
    symbol?: InputMaybe<Scalars['String']['input']>;
};
export type ListAssetFeedersInput = {
    chainId?: InputMaybe<Scalars['Int']['input']>;
    limit?: InputMaybe<Scalars['Int']['input']>;
    nextToken?: InputMaybe<Scalars['String']['input']>;
};
export type ListAssetFeedersResponse = {
    __typename?: 'ListAssetFeedersResponse';
    items: Array<AssetFeeder>;
    nextToken?: Maybe<Scalars['String']['output']>;
};
export type ListAssetStoresResponse = {
    __typename?: 'ListAssetStoresResponse';
    items: Array<AssetDataStore>;
    nextToken?: Maybe<Scalars['String']['output']>;
};
export type Mutation = {
    __typename?: 'Mutation';
    addToMyComponents: Scalars['ID']['output'];
    createFeed: AssetFeed;
    createMyComponents: Scalars['ID']['output'];
    deleteFeed: Scalars['String']['output'];
    deleteMyComponents: Scalars['ID']['output'];
    deployRelayer: AcceptDeploymentReceipt;
    removeFromMyComponents: Scalars['ID']['output'];
    updateLayout?: Maybe<Scalars['String']['output']>;
    updateMyComponents: Scalars['ID']['output'];
    updateRelayer: Relayer;
};
export type MutationAddToMyComponentsArgs = {
    input: AddToMyComponentsInput;
};
export type MutationCreateFeedArgs = {
    input: RegisterFeedInput;
};
export type MutationCreateMyComponentsArgs = {
    label: Scalars['String']['input'];
};
export type MutationDeleteFeedArgs = {
    input: DeleteFeedInput;
};
export type MutationDeleteMyComponentsArgs = {
    id: Scalars['ID']['input'];
};
export type MutationDeployRelayerArgs = {
    input: DeployRelayerInput;
};
export type MutationRemoveFromMyComponentsArgs = {
    input: RemoveFromMyComponentsInput;
};
export type MutationUpdateLayoutArgs = {
    data: Scalars['String']['input'];
};
export type MutationUpdateMyComponentsArgs = {
    input: UpdateMyComponentsInput;
};
export type MutationUpdateRelayerArgs = {
    input: UpdateRelayerInput;
};
export type MyComponent = {
    __typename?: 'MyComponent';
    component: Component;
};
export type MyComponents = {
    __typename?: 'MyComponents';
    components: Array<MyComponent>;
    id: Scalars['ID']['output'];
    label: Scalars['String']['output'];
};
export type Query = {
    __typename?: 'Query';
    assetDataStore?: Maybe<AssetDataStore>;
    assetDataStores: ListAssetStoresResponse;
    assetFeeder?: Maybe<AssetFeeder>;
    assetFeeders: ListAssetFeedersResponse;
    component?: Maybe<Component>;
    components: ComponentResponse;
    componentsByIds: Array<Component>;
    layout?: Maybe<Layout>;
    myComponents: Array<MyComponents>;
    myComponentsById?: Maybe<MyComponents>;
    myFeeds: Array<AssetFeed>;
    relayer: Relayer;
    relayers: RelayersResponse;
    searchComponents: Array<Component>;
};
export type QueryAssetDataStoreArgs = {
    id: Scalars['String']['input'];
};
export type QueryAssetDataStoresArgs = {
    input?: InputMaybe<ListAssetDataStoresInput>;
};
export type QueryAssetFeederArgs = {
    id: Scalars['String']['input'];
};
export type QueryAssetFeedersArgs = {
    input?: InputMaybe<ListAssetFeedersInput>;
};
export type QueryComponentArgs = {
    id: Scalars['ID']['input'];
};
export type QueryComponentsArgs = {
    nextToken?: InputMaybe<Scalars['String']['input']>;
};
export type QueryComponentsByIdsArgs = {
    ids: Array<Scalars['ID']['input']>;
};
export type QueryMyComponentsByIdArgs = {
    id: Scalars['ID']['input'];
};
export type QueryRelayerArgs = {
    id: Scalars['ID']['input'];
};
export type QueryRelayersArgs = {
    nextToken?: InputMaybe<Scalars['String']['input']>;
};
export type QuerySearchComponentsArgs = {
    query?: InputMaybe<Scalars['String']['input']>;
};
export type RegisterFeedInput = {
    dataStore: Scalars['String']['input'];
    feeder: Scalars['String']['input'];
};
export type Relayer = {
    __typename?: 'Relayer';
    asset: Scalars['String']['output'];
    chainId: Scalars['Int']['output'];
    component: Component;
    ethereumAddress: Scalars['String']['output'];
    id: Scalars['ID']['output'];
    rpcUrl: Scalars['String']['output'];
};
export type RelayerMetadata = {
    __typename?: 'RelayerMetadata';
    chainId: Scalars['Int']['output'];
    destination: Scalars['String']['output'];
    oracleType: Scalars['String']['output'];
    signer: Scalars['String']['output'];
};
export type RelayersResponse = {
    __typename?: 'RelayersResponse';
    items: Array<Relayer>;
    nextToken?: Maybe<Scalars['String']['output']>;
};
export type RemoveFromMyComponentsInput = {
    canisterId: Scalars['ID']['input'];
    id: Scalars['ID']['input'];
};
export type SearchComponentsInput = {
    from?: InputMaybe<Scalars['Int']['input']>;
    query?: InputMaybe<Scalars['String']['input']>;
    size?: InputMaybe<Scalars['Int']['input']>;
};
export type SourceLike = Component | StringSource;
export type StringSource = {
    __typename?: 'StringSource';
    value: Scalars['String']['output'];
};
export type Subscription = {
    __typename?: 'Subscription';
    onRelayerDeployed?: Maybe<Relayer>;
};
export type SubscriptionOnRelayerDeployedArgs = {
    id: Scalars['ID']['input'];
};
export type UpdateMyComponentsInput = {
    id: Scalars['ID']['input'];
    label: Scalars['String']['input'];
};
export type UpdateRelayerInput = {
    ethereumAddress: Scalars['String']['input'];
    id: Scalars['ID']['input'];
    principal: Scalars['String']['input'];
};
export type GetComponentQueryVariables = Exact<{
    id: Scalars['ID']['input'];
}>;
export type GetComponentQuery = {
    __typename?: 'Query';
    component?: {
        __typename?: 'Component';
        componentType: ComponentType;
        controllers: Array<string>;
        description: string;
        id: string;
        intervalSec?: number | null;
        label: string;
        proxy: string;
        tags: Array<string>;
        vault: string;
        relayerMetadata?: {
            __typename?: 'RelayerMetadata';
            chainId: number;
            destination: string;
            oracleType: string;
            signer: string;
        } | null;
        sources: Array<{
            __typename?: 'ComponentSource';
            sourceType: ComponentSourceType;
            attributes: {
                __typename?: 'ComponentSourceAttribute';
                chainId?: number | null;
                contractType?: string | null;
                eventName?: string | null;
                functionName?: string | null;
                queries?: Array<{
                    __typename?: 'KeyValue';
                    key: string;
                    value: string;
                } | null> | null;
                sources?: Array<{
                    __typename?: 'Component';
                    id: string;
                    componentType: ComponentType;
                } | null> | null;
            };
            source: {
                __typename?: 'Component';
                id: string;
                componentType: ComponentType;
            } | {
                __typename?: 'StringSource';
                value: string;
            };
        }>;
    } | null;
};
export type ListComponentsQueryVariables = Exact<{
    nextToken?: InputMaybe<Scalars['String']['input']>;
}>;
export type ListComponentsQuery = {
    __typename?: 'Query';
    components: {
        __typename?: 'ComponentResponse';
        nextToken?: string | null;
        items: Array<{
            __typename?: 'Component';
            componentType: ComponentType;
            controllers: Array<string>;
            description: string;
            id: string;
            intervalSec?: number | null;
            label: string;
            proxy: string;
            tags: Array<string>;
            vault: string;
        }>;
    };
};
export declare const GetComponentDocument: import("graphql").DocumentNode;
export declare const ListComponentsDocument: import("graphql").DocumentNode;
export type SdkFunctionWrapper = <T>(action: (requestHeaders?: Record<string, string>) => Promise<T>, operationName: string, operationType?: string, variables?: any) => Promise<T>;
export declare function getSdk(client: GraphQLClient, withWrapper?: SdkFunctionWrapper): {
    GetComponent(variables: GetComponentQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<GetComponentQuery>;
    ListComponents(variables?: ListComponentsQueryVariables, requestHeaders?: GraphQLClientRequestHeaders): Promise<ListComponentsQuery>;
};
export type Sdk = ReturnType<typeof getSdk>;
export {};
