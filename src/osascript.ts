import { stdout, sync } from 'execa';

export const osascript = (filePath: string, ...args: string[]): Promise<string> =>
  stdout('osascript', [filePath, ...args]);

export const osascriptSync = (filePath: string, ...args: string[]): string =>
  sync('osascript', [filePath, ...args]).stdout;
