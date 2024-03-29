import * as ListComponent from './list_components';
import * as ListMetrics from './list_metrics';
import * as OutputSummary from './output_summary_to_csv';
import { bootstrap } from '../common';

bootstrap();

const execute = async () => {
  console.log('Run list_components');
  await ListComponent.execute();
  console.log('Run list_metrics');
  await ListMetrics.execute();
  console.log('Run output_summary_to_csv');
  await OutputSummary.execute();
};

execute()
  .then(() => {
    console.log('Execution completed');
  })
  .catch((error: unknown) => {
    console.error(error);
  });
