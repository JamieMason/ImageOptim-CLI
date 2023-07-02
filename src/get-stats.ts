import { stat } from './fs';
import { Options } from '.';
import { formatFilesize } from './filesize';

export interface FileStats {
  path: string;
  pretty: {
    after: string;
    before: string;
    saving: string;
  };
  raw: {
    after: number;
    before: number;
    percentSaving: number;
    saving: number;
  };
}

export interface Stats {
  files: FileStats[];
  total: FileStats;
}

const getFileSize = async (filePath: string) => {
  const { size } = await stat(filePath);
  return size;
};

const getPercentOf = (whole: number, part: number) => (part ? (part / whole) * 100 : 0);

const createStat = (label: string, sizeAfter: number, sizeBefore: number) => {
  const sizeSaving = sizeBefore - sizeAfter;
  return {
    path: label,
    pretty: {
      after: formatFilesize(sizeAfter),
      before: formatFilesize(sizeBefore),
      saving: formatFilesize(sizeSaving),
    },
    raw: {
      after: sizeAfter,
      before: sizeBefore,
      percentSaving: getPercentOf(sizeBefore, sizeSaving),
      saving: sizeSaving,
    },
  };
};

export const getStats = async (options: Options): Promise<Stats> => {
  const fileStats: FileStats[] = await Promise.all(
    options.filePaths.map(async ({ source, tmp }) => {
      const sizeBefore = await getFileSize(source);
      const sizeAfter = await getFileSize(tmp);
      return createStat(source, sizeAfter, sizeBefore);
    }),
  );

  const totalStats = fileStats.reduce((total, file) => {
    const sizeAfter = total.raw.after + file.raw.after;
    const sizeBefore = total.raw.before + file.raw.before;
    return createStat('TOTAL', sizeAfter, sizeBefore);
  }, createStat('TOTAL', 0, 0));

  return {
    files: fileStats,
    total: totalStats,
  };
};
