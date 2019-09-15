import { AppRunner } from '.';
import {
  getJpegMini,
  quitApp,
  runJpegMini as startJpegMini,
  supportsAssistiveDevices
} from './applescript';
import { ASSISTIVE_DEVICES_URL, JPEG_MINI_URL } from './constants';
import { info, panic, verbose } from './log';

export const runJpegMini: AppRunner = async (options) => {
  verbose('Locating JPEGmini installation');
  const jpegMini = getJpegMini();

  verbose('Checking support for assistive devices');
  const assistiveDeviceSupport = supportsAssistiveDevices();

  const [app, canAutomate] = await Promise.all([jpegMini, assistiveDeviceSupport]);

  if (!app) {
    return panic(`JPEGmini is not installed (${JPEG_MINI_URL})`);
  }

  if (!canAutomate) {
    return panic(`Support for assistive devices needed, see ${ASSISTIVE_DEVICES_URL}`);
  }

  info(`Running ${app.name}...`);
  await startJpegMini(app);

  if (options.enabled.quit) {
    verbose(`Quitting ${app.name}`);
    await quitApp(app);
  }

  verbose(`${app.name} has finished`);
};
