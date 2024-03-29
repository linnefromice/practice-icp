import { Principal } from '@dfinity/principal';
import {
  Metric,
  PATH_COMPONENTS_JSON,
  PATH_METRIC_CSV,
  PATH_METRIC_ERR_CSV,
  PATH_METRIC_ERR_JSON,
  PATH_METRIC_JSON,
  bootstrap,
  flattenObject,
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

export const execute = async () => {
  const components =
    await loadJson<ListComponentsQuery['components']['items']>(
      PATH_COMPONENTS_JSON
    );

  // for: Debug
  // const sliced = components.slice(
  //   0,
  //   components.length < 20 ? components.length : 20
  // );

  const agent = getAgent();
  const result = [];
  const errs: { id: string; error: string }[] = [];
  for (const c of components) {
    const principal = Principal.fromText(c.vault);
    const actor = vaultActor(principal, agent);

    const metric = await actor.metric().catch((e: any) => {
      errs.push({ id: c.id, error: e.totring() });
      return null;
    });
    const data: Metric = {
      id: c.id,
      label: c.label,
      metric: metric ?? null,
    };
    result.push(data);
  }
  writeJson(PATH_METRIC_ERR_JSON, errs);
  writeJson(PATH_METRIC_ERR_CSV, errs.map(flattenObject));

  // writeJson(PATH_METRIC_JSON, result);
  // writeJson(PATH_METRIC_CSV, result.map(flattenObject));
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
