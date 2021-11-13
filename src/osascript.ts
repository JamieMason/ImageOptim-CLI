import execa from 'execa';

export const osascript = (filePath: string, ...args: string[]): Promise<string> =>
  execa('osascript', [filePath, ...args]).then(({ stdout }) => stdout);

export const osascriptSync = (filePath: string, ...args: string[]): string =>
  execa.sync('osascript', [filePath, ...args]).stdout;
