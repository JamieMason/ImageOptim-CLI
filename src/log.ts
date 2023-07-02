import chalk from 'chalk';
import { types } from 'util';
import { Options } from '.';
import { clean } from './tmpdir';

let color = new chalk.Instance({ level: 3 });

export const complete = (value: string): void => console.log(color.green('✓ %s'), value);
export const info = (value: string): void => console.log(color.blue('i %s'), value);
export const warning = (value: string): void => console.log(color.yellow('! %s'), value);

export const bug = (err: Error | unknown): void => {
  if (types.isNativeError(err)) {
    console.log(
      color.red('! %s\n\n! Please raise an issue at %s\n\n%s'),
      err.message,
      color.underline('https://github.com/JamieMason/ImageOptim-CLI/issues'),
      String(err.stack).replace(/^/gm, '    '),
    );
  } else {
    console.log(
      color.red('! %s\n\n! Please raise an issue at %s'),
      err,
      color.underline('https://github.com/JamieMason/ImageOptim-CLI/issues'),
    );
  }
  process.exit(1);
};

export const panic = async (value: string, options: Options): Promise<void> => {
  console.log(color.red('! %s'), value);
  await clean(options);
  process.exit(1);
};

export const result = (
  label = 'TOTAL',
  prettySizeBefore: string,
  prettySizeAfter: string,
  prettySizeSaving: string,
  sizeSavingPercent: number,
) => {
  console.log(
    '%s %s was: %s now: %s saving: %s (%s)',
    color.green('✓'),
    chalk.underline(label),
    color.red(prettySizeBefore),
    color.green(prettySizeAfter),
    color.green(prettySizeSaving),
    color.green(`${sizeSavingPercent.toFixed(2)}%`),
  );
};

export const verbose =
  process.env.NODE_ENV === 'development'
    ? (value: string): void => console.info(color.grey('? %s'), value)
    : (): void => undefined;

export const enableColor = (enabled: boolean) => {
  color = new chalk.Instance({ level: enabled ? 3 : 0 });
};
