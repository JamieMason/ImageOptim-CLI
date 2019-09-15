import { copy, remove } from 'fs-extra';
import { IFile, IOptions } from '.';
import { verbose } from './log';

const sourceToTmp = ({ source, tmp }: IFile) => copy(source, tmp);
const tmpToSource = ({ source, tmp }: IFile) => copy(tmp, source);

export const clean = (options: { tmpDir: string }) => remove(options.tmpDir);

export const setup = async (options: IOptions) => {
  verbose(`Creating temp directory ${options.tmpDir}`);
  await clean(options);
  verbose(`Copying ${options.filePaths.length} files to temp directory`);
  await Promise.all(options.filePaths.map(sourceToTmp));
};

export const tearDown = async (options: IOptions) => {
  verbose(`Copying ${options.filePaths.length} files back to original location`);
  await Promise.all(options.filePaths.map(tmpToSource));
  verbose(`Deleting temp directory ${options.tmpDir}`);
  await clean(options);
};
