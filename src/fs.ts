import execa from 'execa';
import { access, constants, copyFile, stat as fsStat, Stats } from 'fs';
import { dirname } from 'path';
import { verbose } from './log';

async function mkdirP(src: string): Promise<void> {
  verbose(`mkdir -p ${src}`);
  await execa('mkdir', ['-p', src]);
}

export function copy(src: string, target: string): Promise<void> {
  return mkdirP(dirname(target)).then(() => {
    return new Promise((resolve, reject) => {
      verbose(`fs.copyFile(${src}, ${target})`);
      copyFile(src, target, (err) => (err ? reject(err) : resolve()));
    });
  });
}

export function pathExists(src: string): Promise<boolean> {
  return new Promise((resolve) => {
    verbose(`fs.access(${src}, ${constants.F_OK})`);
    access(src, constants.F_OK, (err) => (err ? resolve(false) : resolve(true)));
  });
}

export async function remove(src: string): Promise<void> {
  verbose(`rm -rf ${src}`);
  await execa('rm', ['-rf', src]);
}

export function stat(src: string): Promise<Stats> {
  return new Promise((resolve, reject) => {
    verbose(`fs.stat(${src})`);
    fsStat(src, (err, stats) => (err ? reject(err) : resolve(stats)));
  });
}
