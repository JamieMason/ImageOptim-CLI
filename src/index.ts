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

export interface IOptions {
  enabled: {
    color: boolean;
    imageAlpha: boolean;
    imageOptim: boolean;
    jpegMini: boolean;
    quit: boolean;
    stats: boolean;
  };
  files: {
    all: string[];
    supported: IFile[];
  };
  numberOfColors: string;
  quality: string;
  speed: string;
  tmpDir: string;
}

const runnersByName = {
  imageAlpha: runImageAlpha,
  imageOptim: runImageOptim,
  jpegMini: runJpegMini,
  stats: getStats
};

export const cli = async (options: IOptions) => {
  try {
    const runIfEnabled = (key: keyof typeof runnersByName) =>
      options.enabled[key] ? runnersByName[key](options) : Promise.resolve();

    enableColor(options.enabled.color);
    if (options.files.supported.length === 0) {
      return warning('No images matched the patterns provided');
    }
    await setup(options);
    await Promise.all([runIfEnabled('imageAlpha'), runIfEnabled('jpegMini')]);
    await runIfEnabled('imageOptim');
    const stats = await runIfEnabled('stats');
    await tearDown(options);
    if (stats) {
      await writeReport(stats);
    }
    complete('Finished');
  } catch (err) {
    bug(err);
    await clean(options);
  }
};
