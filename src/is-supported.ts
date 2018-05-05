import { extname } from 'path';

export const isSupported = (supportedFileTypes: string[]) => (filePath: string) =>
  supportedFileTypes.indexOf(extname(filePath)) !== -1;
