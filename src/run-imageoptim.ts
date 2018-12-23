import { stdout } from 'execa';
import { pathExists } from 'fs-extra';
import { AppRunner } from '.';
import { IMAGEOPTIM, IMAGEOPTIM_BIN_PATH, IMAGEOPTIM_URL } from './constants';
import { info, verbose, warning } from './log';

export const runImageOptim: AppRunner = async (options) => {
  info(`Running ${IMAGEOPTIM.name}...`);
  if (!(await pathExists(IMAGEOPTIM_BIN_PATH))) {
    return warning(`ImageOptim.app is not installed (${IMAGEOPTIM_URL})`);
  }
  await stdout(IMAGEOPTIM_BIN_PATH, [options.tmpDir]);
  verbose(`${IMAGEOPTIM.name} has finished`);
};
