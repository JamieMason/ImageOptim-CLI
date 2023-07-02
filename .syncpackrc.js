// @ts-check

/** @type {import("syncpack").RcFile} */
const config = {
  versionGroups: [
    {
      dependencies: ['@types/node'],
      packages: ['**'],
      pinVersion: '18.16.19',
    },
    {
      dependencies: ['chalk'],
      packages: ['**'],
      pinVersion: '4.1.2',
    },
    {
      dependencies: ['globby'],
      packages: ['**'],
      pinVersion: '11.1.0',
    },
    {
      dependencies: ['pretty-bytes'],
      packages: ['**'],
      pinVersion: '5.6.0',
    },
  ],
};

module.exports = config;
