import { AppRunner, IOptions } from '.';
import { IMAGEALPHA } from './constants';
import { isSupported } from './is-supported';
import { info, verbose } from './log';
import { pngquant } from './pngquant';

export const runImageAlpha: AppRunner = async (options: IOptions) => {
  info(`Running ${IMAGEALPHA.name}...`);
  const pngFilePaths = options.files.supported
    .map((file) => file.tmp)
    .filter(isSupported(IMAGEALPHA.supports));
  await pngquant(pngFilePaths, options);
  verbose(`${IMAGEALPHA.name} has finished`);
};
