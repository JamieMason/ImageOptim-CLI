import { join } from 'path';
import { getStats } from './get-stats';
import { bug, complete, enableColor, warning } from './log';
import { runImageAlpha } from './run-imagealpha';
import { runImageOptim } from './run-imageoptim';
import { runJpegMini } from './run-jpegmini';
import { clean, setup, tearDown } from './tmpdir';
import { writeReport } from './write-report';

export type AppRunner = (options: IOptions) => Promise<any>;

export interface IFile {
  source: string;
  tmp: string;
}

export interface ICliOptions {
  batchSize: number;
  enabled: {
    color: boolean;
    imageAlpha: boolean;
    imageOptim: boolean;
    jpegMini: boolean;
    quit: boolean;
    stats: boolean;
  };
  filePaths: string[];
  numberOfColors: string;
  quality: string;
  speed: string;
  tmpDir: string;
}

export interface IOptions {
  batchSize: number;
  enabled: {
    color: boolean;
    imageAlpha: boolean;
    imageOptim: boolean;
    jpegMini: boolean;
    quit: boolean;
    stats: boolean;
  };
  filePaths: IFile[];
  numberOfColors: string;
  quality: string;
  speed: string;
  tmpDir: string;
}

const runnersByName = {
  imageAlpha: runImageAlpha,
  imageOptim: runImageOptim,
  jpegMini: runJpegMini,
  stats: getStats,
};

const cloneArray = (array: string[]) => [...array];

const runIfEnabled = (key: keyof typeof runnersByName, options: IOptions) =>
  options.enabled[key] ? runnersByName[key](options) : Promise.resolve();

const processBatch = async (options: IOptions) => {
  await setup(options);
  await Promise.all([runIfEnabled('imageAlpha', options), runIfEnabled('jpegMini', options)]);
  await runIfEnabled('imageOptim', options);
  const stats = await runIfEnabled('stats', options);
  await tearDown(options);
  if (stats) {
    await writeReport(stats);
  }
};

export const cli = async (options: ICliOptions) => {
  try {
    const filesMutable = cloneArray(options.filePaths);
    enableColor(options.enabled.color);
    if (filesMutable.length === 0) {
      return warning('No images matched the patterns provided');
    }
    while (filesMutable.length > 0) {
      const filePaths = filesMutable.splice(0, options.batchSize);
      await processBatch({
        batchSize: options.batchSize,
        enabled: options.enabled,
        filePaths: filePaths.map((filePath) => ({
          source: filePath,
          tmp: join(options.tmpDir, filePath),
        })),
        numberOfColors: options.numberOfColors,
        quality: options.quality,
        speed: options.speed,
        tmpDir: options.tmpDir,
      });
    }
    complete('Finished');
  } catch (err) {
    await clean(options);
    bug(err);
  }
};
