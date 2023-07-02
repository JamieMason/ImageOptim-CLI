import { exec } from './exec';

export const osascript = (filePath: string, ...args: string[]): Promise<string> =>
  exec('osascript', [filePath, ...args]).then(({ stdout }) => stdout);
