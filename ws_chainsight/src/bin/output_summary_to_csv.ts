import {
  Metric,
  PATH_COMPONENTS,
  PATH_METRIC,
  PATH_SUMMURY_CSV,
  bootstrap,
  extendedMetric,
  flattenObject,
  loadJson,
  writeCsv,
} from '../common';
import { ListComponentsQuery } from '../gql/graphql';

bootstrap();

const execute = async () => {
  const components =
    await loadJson<ListComponentsQuery['components']['items']>(PATH_COMPONENTS);
  const metrics = await loadJson<Metric[]>(PATH_METRIC);

  const merged = components.map(c => {
    const metric = metrics.find(m => m.id === c.id);
    return {
      ...c,
      metric: metric ? extendedMetric(metric) : null,
    };
  });

  const raw = merged.map(c => flattenObject(c));
  writeCsv(PATH_SUMMURY_CSV, raw);
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
