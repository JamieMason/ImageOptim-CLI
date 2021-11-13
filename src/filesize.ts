import prettyBytes from 'pretty-bytes';

export const formatFilesize = (value: number) => prettyBytes(value).replace(' ', '');
