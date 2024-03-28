import type {CodegenConfig} from '@graphql-codegen/cli';

const config: CodegenConfig = {
  overwrite: true,
  schema: 'resources/schema.graphql',
  documents: 'src/graphql/**.graphql',
  generates: {
    'src/gql/graphql.ts': {
      // preset: "client",
      plugins: [
        'typescript',
        'typescript-operations',
        'typescript-graphql-request',
      ],
    },
  },
};

export default config;
