import { IStats } from './get-stats';
import { result } from './log';

export const writeReport = async (stats: IStats) => {
  const { total } = stats;
  const results = stats.files.concat(total);
  results.forEach(({ path, pretty, raw }) => {
    result(path, pretty.before, pretty.after, pretty.saving, raw.percentSaving, 100);
  });
};
