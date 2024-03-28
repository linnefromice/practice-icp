import {Amplify} from 'aws-amplify';
import * as dotenv from 'dotenv';

import AwsExports from './aws-exports';
import {GraphQLClient} from 'graphql-request';
import {getSdk} from './gql/graphql';

Amplify.configure(AwsExports);

dotenv.config();

const execute = async () => {
  console.log('Hello World');

  const endpoint = process.env.GRAPHQL_ENDPOINT;
  const apiKey = process.env.AMPLIFY_API_KEY;
  if (!endpoint || !apiKey) {
    throw new Error('Missing environment variables');
  }

  const client = new GraphQLClient(endpoint, {
    headers: {
      'X-Api-Key': apiKey,
    },
  });
  const sdk = getSdk(client);
  const {components} = await sdk.ListComponents();
  console.log(components.items.length);
  console.log(components);
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
