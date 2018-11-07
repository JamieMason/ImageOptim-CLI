import { dirname, resolve } from 'path';
import {
  IApp,
  JPEGMINI,
  JPEGMINI_LITE,
  JPEGMINI_LITE_RETAIL,
  JPEGMINI_PRO,
  JPEGMINI_PRO_RETAIL,
  JPEGMINI_RETAIL,
  TMPDIR
} from './constants';
import { osascript } from './osascript';

const getDirname = () => dirname(module.filename || process.execPath);
const getScriptPath = (name: string) => resolve(getDirname(), `../osascript/${name}.applescript`);
const toBoolean = (value: string) => value === 'true';

export const isInstalled = (app: IApp): Promise<boolean> =>
  osascript(getScriptPath('is-installed'), app.bundleId).then(toBoolean);

export const quitApp = (app: IApp): Promise<void> =>
  osascript(getScriptPath('quit-app'), app.name).then(() => undefined);

export const runJpegMini = (app: IApp): Promise<void> =>
  osascript(getScriptPath('run-jpegmini'), TMPDIR, app.name).then(() => undefined);

export const supportsAssistiveDevices = (): Promise<boolean> =>
  osascript(getScriptPath('supports-assistive-devices')).then(toBoolean);

export const getJpegMini = async (): Promise<IApp | null> =>
  (await isInstalled(JPEGMINI_PRO))
    ? JPEGMINI_PRO
    : (await isInstalled(JPEGMINI_PRO_RETAIL))
    ? JPEGMINI_PRO_RETAIL
    : (await isInstalled(JPEGMINI))
    ? JPEGMINI
    : (await isInstalled(JPEGMINI_RETAIL))
    ? JPEGMINI_RETAIL
    : (await isInstalled(JPEGMINI_LITE))
    ? JPEGMINI_LITE
    : (await isInstalled(JPEGMINI_LITE_RETAIL))
    ? JPEGMINI_LITE_RETAIL
    : null;
