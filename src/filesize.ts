import prettyBytes = require('pretty-bytes');

export const formatFilesize = (value: number) => prettyBytes(value).replace(' ', '');
