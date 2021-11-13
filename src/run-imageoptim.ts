import execa from 'execa';
import { pathExists } from './fs';
import { AppRunner } from '.';
import { IMAGEOPTIM, IMAGEOPTIM_BIN_PATH, IMAGEOPTIM_URL } from './constants';
import { info, panic, verbose } from './log';

export const runImageOptim: AppRunner = async (options) => {
  info(`Running ${IMAGEOPTIM.name}...`);
  if (!(await pathExists(IMAGEOPTIM_BIN_PATH))) {
    return panic(`ImageOptim.app is not installed (${IMAGEOPTIM_URL})`, options);
  }
  await execa(IMAGEOPTIM_BIN_PATH, [options.tmpDir]);
  verbose(`${IMAGEOPTIM.name} has finished`);
};
