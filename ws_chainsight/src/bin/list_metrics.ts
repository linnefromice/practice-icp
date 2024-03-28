import { Principal } from '@dfinity/principal';
import {
  PATH_COMPONENTS,
  PATH_METRIC,
  bootstrap,
  getAgent,
  loadJson,
  vaultActor,
  writeJson,
} from '../common';
import type { ListComponentsQuery } from '../gql/graphql';

// NOTE: TypeError: Do not know how to serialize a BigInt
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(BigInt.prototype as any).toJSON = function () {
  return this.toString();
};

bootstrap();

const execute = async () => {
  const components =
    await loadJson<ListComponentsQuery['components']['items']>(PATH_COMPONENTS);

  // for: Debug
  // const sliced = components.slice(
  //   0,
  //   components.length < 20 ? components.length : 20
  // );

  const agent = getAgent();
  const result = [];
  for (const c of components) {
    const principal = Principal.fromText(c.vault);
    const actor = vaultActor(principal, agent);

    const metric = await actor.metric().catch(() => {
      return null;
    });
    const data = {
      id: c.id,
      label: c.label,
      metric,
    };
    result.push(data);
  }
  writeJson(PATH_METRIC, result);
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
