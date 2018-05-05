import { stdout } from 'execa';
import { AppRunner } from '.';
import { IMAGEOPTIM, IMAGEOPTIM_BIN_PATH } from './constants';
import { info, verbose } from './log';

export const runImageOptim: AppRunner = async (options) => {
  info(`Running ${IMAGEOPTIM.name}...`);
  await stdout(IMAGEOPTIM_BIN_PATH, [options.tmpDir]);
  verbose(`${IMAGEOPTIM.name} has finished`);
};
