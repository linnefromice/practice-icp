"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getSdk = exports.ListComponentsDocument = exports.GetComponentDocument = exports.ComponentType = exports.ComponentSourceType = void 0;
const graphql_tag_1 = require("graphql-tag");
var ComponentSourceType;
(function (ComponentSourceType) {
    ComponentSourceType["Chainsight"] = "Chainsight";
    ComponentSourceType["Evm"] = "Evm";
    ComponentSourceType["Https"] = "Https";
    ComponentSourceType["Icp"] = "Icp";
})(ComponentSourceType || (exports.ComponentSourceType = ComponentSourceType = {}));
var ComponentType;
(function (ComponentType) {
    ComponentType["AlgorithmIndexer"] = "AlgorithmIndexer";
    ComponentType["AlgorithmLens"] = "AlgorithmLens";
    ComponentType["EventIndexer"] = "EventIndexer";
    ComponentType["Relayer"] = "Relayer";
    ComponentType["SnapshotIndexerEvm"] = "SnapshotIndexerEvm";
    ComponentType["SnapshotIndexerHttps"] = "SnapshotIndexerHttps";
    ComponentType["SnapshotIndexerIcp"] = "SnapshotIndexerIcp";
})(ComponentType || (exports.ComponentType = ComponentType = {}));
exports.GetComponentDocument = (0, graphql_tag_1.default) `
    query GetComponent($id: ID!) {
  component(id: $id) {
    componentType
    controllers
    description
    id
    intervalSec
    label
    proxy
    relayerMetadata {
      chainId
      destination
      oracleType
      signer
    }
    sources {
      attributes {
        chainId
        contractType
        eventName
        functionName
        queries {
          key
          value
        }
        sources {
          id
          componentType
        }
      }
      source {
        ... on StringSource {
          value
        }
        ... on Component {
          id
          componentType
        }
      }
      sourceType
    }
    tags
    vault
  }
}
    `;
exports.ListComponentsDocument = (0, graphql_tag_1.default) `
    query ListComponents($nextToken: String) {
  components(nextToken: $nextToken) {
    nextToken
    items {
      componentType
      controllers
      description
      id
      intervalSec
      label
      proxy
      tags
      vault
    }
  }
}
    `;
const defaultWrapper = (action, _operationName, _operationType, _variables) => action();
function getSdk(client, withWrapper = defaultWrapper) {
    return {
        GetComponent(variables, requestHeaders) {
            return withWrapper((wrappedRequestHeaders) => client.request(exports.GetComponentDocument, variables, { ...requestHeaders, ...wrappedRequestHeaders }), 'GetComponent', 'query', variables);
        },
        ListComponents(variables, requestHeaders) {
            return withWrapper((wrappedRequestHeaders) => client.request(exports.ListComponentsDocument, variables, { ...requestHeaders, ...wrappedRequestHeaders }), 'ListComponents', 'query', variables);
        }
    };
}
exports.getSdk = getSdk;
//# sourceMappingURL=graphql.js.map