import { pathExists } from 'fs-extra';
import { AppRunner, IOptions } from '.';
import { IMAGEALPHA, IMAGEALPHA_URL, PNGQUANT_BIN_PATH } from './constants';
import { isSupported } from './is-supported';
import { info, verbose, warning } from './log';
import { pngquant } from './pngquant';

export const runImageAlpha: AppRunner = async (options: IOptions) => {
  info(`Running ${IMAGEALPHA.name}...`);
  const pngFilePaths = options.files.supported
    .map((file) => file.tmp)
    .filter(isSupported(IMAGEALPHA.supports));
  if (!(await pathExists(PNGQUANT_BIN_PATH))) {
    return warning(`ImageAlpha.app is not installed (${IMAGEALPHA_URL})`);
  }
  await pngquant(pngFilePaths, options);
  verbose(`${IMAGEALPHA.name} has finished`);
};
