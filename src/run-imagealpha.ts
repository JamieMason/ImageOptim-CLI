import { pathExists } from './fs';
import { AppRunner, IOptions } from '.';
import { IMAGEALPHA, IMAGEALPHA_URL, PNGQUANT_BIN_PATH } from './constants';
import { isSupported } from './is-supported';
import { info, panic, verbose } from './log';
import { pngquant } from './pngquant';

export const runImageAlpha: AppRunner = async (options: IOptions) => {
  info(`Running ${IMAGEALPHA.name}...`);
  const pngFilePaths = options.filePaths
    .map((file) => file.tmp)
    .filter(isSupported(IMAGEALPHA.supports));
  if (!(await pathExists(PNGQUANT_BIN_PATH))) {
    return panic(`ImageAlpha.app is not installed (${IMAGEALPHA_URL})`, options);
  }
  await pngquant(pngFilePaths, options);
  verbose(`${IMAGEALPHA.name} has finished`);
};
