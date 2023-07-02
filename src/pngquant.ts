import { isWalkable } from 'expect-more';
import { Options } from '.';
import { PNGQUANT_BIN_PATH } from './constants';
import { exec } from './exec';

export const pngquant = async (pngFilePaths: string[], options: Options): Promise<void> => {
  try {
    await exec(PNGQUANT_BIN_PATH, [
      '--ext=.png',
      '--force',
      '--skip-if-larger',
      `--speed=${options.speed}`,
      `--quality=${options.quality}`,
      options.numberOfColors,
      '--',
      ...pngFilePaths,
    ]);
  } catch (err) {
    if (isWalkable(err) && err.exitCode !== 99 && err.exitCode !== 98) {
      throw err;
    }
  }
};
