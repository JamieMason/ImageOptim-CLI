import { File, Options } from '.';
import { copy, remove } from './fs';
import { verbose } from './log';

function sourceToTmp({ source, tmp }: File): Promise<void> {
  return copy(source, tmp);
}

function tmpToSource({ source, tmp }: File): Promise<void> {
  return copy(tmp, source);
}

export const clean = (options: { tmpDir: string }) => {
  return remove(options.tmpDir);
};

export const setup = async (options: Options) => {
  await clean(options);
  verbose(`Copying ${options.filePaths.length} files to temp directory`);
  await Promise.all(options.filePaths.map(sourceToTmp));
};

export const tearDown = async (options: Options) => {
  verbose(`Copying ${options.filePaths.length} files back to original location`);
  await Promise.all(options.filePaths.map(tmpToSource));
  await clean(options);
};
