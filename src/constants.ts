import { tmpdir } from 'os';
import { join } from 'path';

export interface IApp {
  readonly bundleId: string;
  readonly name: string;
  readonly supports: string[];
}

const manifest = require('../package.json');
const supports = {
  imageAlpha: ['.PNG', '.png'],
  imageOptim: [
    '.BMP',
    '.bmp',
    '.GIF',
    '.gif',
    '.JPEG',
    '.jpeg',
    '.JPG',
    '.jpg',
    '.PCX',
    '.pcx',
    '.PNG',
    '.png',
    '.PNM',
    '.pnm',
    '.TGA',
    '.tga',
    '.TIFF',
    '.tiff'
  ],
  jpegmini: ['.JPEG', '.jpeg', '.JPG', '.jpg']
};

export const SUPPORTED_FILE_TYPES = [
  ...supports.imageAlpha,
  ...supports.imageOptim,
  ...supports.jpegmini
].filter((value, i, list) => list.indexOf(value) === i);

export const TMPDIR = join(tmpdir(), 'imageoptim-cli');
export const VERSION = manifest.version;
export const PNGQUANT_NUMBER_OF_COLORS = '256';
export const PNGQUANT_QUALITY = '65-80';
export const PNGQUANT_SPEED = '1';
export const PNGQUANT_BIN_PATH = '/Applications/ImageAlpha.app/Contents/MacOS/pngquant';
export const IMAGEOPTIM_BIN_PATH = '/Applications/ImageOptim.app/Contents/MacOS/ImageOptim';

export const HOMEPAGE_URL = 'https://github.com/JamieMason/ImageOptim-CLI';
export const ASSISTIVE_DEVICES_URL = `${HOMEPAGE_URL}/#%EF%B8%8F-jpegmini-and-support-for-assistive-devices`;
export const IMAGEALPHA_URL = 'https://pngmini.com/';
export const IMAGEOPTIM_URL = 'https://imageoptim.com/mac';
export const JPEG_MINI_URL = 'https://itunes.apple.com/us/app/jpegmini/id498944723';

export const IMAGEALPHA: IApp = {
  bundleId: 'net.pornel.ImageAlpha',
  name: 'ImageAlpha',
  supports: supports.imageAlpha
};

export const IMAGEOPTIM: IApp = {
  bundleId: 'net.pornel.ImageOptim',
  name: 'ImageOptim',
  supports: supports.imageOptim
};

export const JPEGMINI: IApp = {
  bundleId: 'com.icvt.JPEGmini',
  name: 'JPEGmini',
  supports: supports.jpegmini
};

export const JPEGMINI_RETAIL: IApp = {
  bundleId: 'com.icvt.JPEGmini-retail',
  name: 'JPEGmini',
  supports: supports.jpegmini
};

export const JPEGMINI_LITE: IApp = {
  bundleId: 'com.icvt.JPEGminiLite',
  name: 'JPEGmini Lite',
  supports: supports.jpegmini
};

export const JPEGMINI_LITE_RETAIL: IApp = {
  bundleId: 'com.icvt.JPEGminiLite-retail',
  name: 'JPEGmini Lite',
  supports: supports.jpegmini
};

export const JPEGMINI_PRO: IApp = {
  bundleId: 'com.icvt.JPEGmini-Pro',
  name: 'JPEGmini Pro',
  supports: supports.jpegmini
};

export const JPEGMINI_PRO_RETAIL: IApp = {
  bundleId: 'com.icvt.JPEGmini-Pro-retail',
  name: 'JPEGmini Pro',
  supports: supports.jpegmini
};

export const ERROR_JPEGMINI_NOT_INSTALLED = 'ERROR_JPEGMINI_NOT_INSTALLED';
export const ERROR_CANNOT_AUTOMATE_OSX = 'ERROR_CANNOT_AUTOMATE_OSX';
