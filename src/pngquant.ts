import execa = require('execa');
import { IOptions } from '.';
import { PNGQUANT_BIN_PATH } from './constants';

export const pngquant = async (pngFilePaths: string[], options: IOptions): Promise<void> => {
  try {
    await execa(PNGQUANT_BIN_PATH, [
      '--ext=.png',
      '--force',
      '--skip-if-larger',
      `--speed=${options.speed}`,
      `--quality=${options.quality}`,
      options.numberOfColors,
      '--',
      ...pngFilePaths
    ]);
  } catch (err) {
    if (err.exitCode !== 99 && err.exitCode !== 98) {
      throw err;
    }
  }
};
