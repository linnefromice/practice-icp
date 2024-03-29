import {
  Metric,
  PATH_COMPONENTS_JSON,
  PATH_METRIC_JSON,
  PATH_SNAPSHOTS_JSON,
  PATH_SUMMURY_CSV,
  Snapshot,
  bootstrap,
  extendedMetric,
  extendedSnapshot,
  flattenObject,
  loadJson,
  writeCsv,
} from '../common';
import { ListComponentsQuery } from '../gql/graphql';

bootstrap();

export const execute = async () => {
  const components =
    await loadJson<ListComponentsQuery['components']['items']>(
      PATH_COMPONENTS_JSON
    );
  const metrics = await loadJson<Metric[]>(PATH_METRIC_JSON);
  const snapshots = await loadJson<Snapshot[]>(PATH_SNAPSHOTS_JSON);
  console.log(`components: ${components.length}`);
  console.log(`metrics: ${metrics.length}`);

  const merged = components.map(c => {
    const metric = metrics.find(m => m.id === c.id);
    const snapshot = snapshots.find(s => s.id === c.id);
    return {
      ...c,
      metric: metric ? extendedMetric(metric) : null,
      snapshot: snapshot?.snapshots
        ? extendedSnapshot(snapshot.snapshots)
        : extendedSnapshot([]),
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
