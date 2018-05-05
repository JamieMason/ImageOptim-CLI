import { copy, remove } from 'fs-extra';
import { IFile, IOptions } from '.';
import { verbose } from './log';

const sourceToTmp = ({ source, tmp }: IFile) => copy(source, tmp);
const tmpToSource = ({ source, tmp }: IFile) => copy(tmp, source);

export const clean = (options: IOptions) => remove(options.tmpDir);

export const setup = async (options: IOptions) => {
  verbose(`Creating temp directory ${options.tmpDir}`);
  await clean(options);
  verbose(`Copying ${options.files.supported.length} files to temp directory`);
  await Promise.all(options.files.supported.map(sourceToTmp));
};

export const tearDown = async (options: IOptions) => {
  verbose(`Copying ${options.files.supported.length} files back to original location`);
  await Promise.all(options.files.supported.map(tmpToSource));
  verbose(`Deleting temp directory ${options.tmpDir}`);
  await clean(options);
};
