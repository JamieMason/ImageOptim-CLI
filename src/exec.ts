import { exec as execProcess } from 'child_process';
import { promisify } from 'util';

const execProm = promisify(execProcess);

export const exec = (program: string, args: string[]) => execProm(`${program} ${args.join(' ')}`);
