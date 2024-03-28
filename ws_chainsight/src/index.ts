import {Amplify} from 'aws-amplify';
import AwsExports from './aws-exports';

Amplify.configure(AwsExports);

const execute = async () => {
  console.log('Hello World');
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
