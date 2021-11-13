import { IStats } from './get-stats';
import { result, warning } from './log';

export const writeReport = async (stats: IStats) => {
  const { total } = stats;
  const results = stats.files.concat(total);
  if (total.pretty.saving === '0B') {
    warning('No size savings');
  } else {
    results.forEach(({ path, pretty, raw }) => {
      result(path, pretty.before, pretty.after, pretty.saving, raw.percentSaving);
    });
  }
};
