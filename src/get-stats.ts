import { stat } from 'fs-extra';
import { IOptions } from '.';
import { formatFilesize } from './filesize';

export interface IFileStats {
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

export interface IStats {
  files: IFileStats[];
  total: IFileStats;
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
      saving: formatFilesize(sizeSaving)
    },
    raw: {
      after: sizeAfter,
      before: sizeBefore,
      percentSaving: getPercentOf(sizeBefore, sizeSaving),
      saving: sizeSaving
    }
  };
};

export const getStats = async (options: IOptions): Promise<IStats> => {
  const fileStats: IFileStats[] = await Promise.all(
    options.files.supported.map(async ({ source, tmp }) => {
      const sizeBefore = await getFileSize(source);
      const sizeAfter = await getFileSize(tmp);
      return createStat(source, sizeAfter, sizeBefore);
    })
  );

  const totalStats = fileStats.reduce((total, file) => {
    const sizeAfter = total.raw.after + file.raw.after;
    const sizeBefore = total.raw.before + file.raw.before;
    return createStat('TOTAL', sizeAfter, sizeBefore);
  }, createStat('TOTAL', 0, 0));

  return {
    files: fileStats,
    total: totalStats
  };
};
