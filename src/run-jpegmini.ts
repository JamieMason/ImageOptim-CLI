import { AppRunner } from '.';
import {
  getJpegMini,
  quitApp,
  runJpegMini as startJpegMini,
  supportsAssistiveDevices
} from './applescript';
import { info, verbose, warning } from './log';

export const runJpegMini: AppRunner = async (options) => {
  const homepageUrl = 'https://github.com/JamieMason/ImageOptim-CLI';
  const jpegMiniUrl = 'https://itunes.apple.com/us/app/jpegmini/id498944723';
  const assistiveDevicesUrl = `${homepageUrl}/#jpegmini-and-support-for-assistive-devices`;

  verbose('Locating JPEGmini installation');
  const jpegMini = getJpegMini();

  verbose('Checking support for assistive devices');
  const assistiveDeviceSupport = supportsAssistiveDevices();

  const [app, canAutomate] = await Promise.all([jpegMini, assistiveDeviceSupport]);

  if (!app) {
    return warning(`JPEGmini is not installed (${jpegMiniUrl})`);
  }

  if (!canAutomate) {
    return warning(`Support for assistive devices needed, see ${assistiveDevicesUrl}`);
  }

  info(`Running ${app.name}...`);
  await startJpegMini(app);

  if (options.enabled.quit) {
    verbose(`Quitting ${app.name}`);
    await quitApp(app);
  }

  verbose(`${app.name} has finished`);
};
