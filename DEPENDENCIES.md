# imageoptim-cli

Automates ImageOptim, ImageAlpha, and JPEGmini for Mac to make batch optimisation of images part of your automated build process.

## Installation

This is a [Node.js](https://nodejs.org/) module available through the 
[npm registry](https://www.npmjs.com/). It can be installed using the 
[`npm`](https://docs.npmjs.com/getting-started/installing-npm-packages-locally)
or 
[`yarn`](https://yarnpkg.com/en/)
command line tools.

```sh
npm install imageoptim-cli --save
```

## Dependencies

- [chalk](https://ghub.io/chalk): Terminal string styling done right
- [commander](https://ghub.io/commander): the complete solution for node.js command-line programs
- [execa](https://ghub.io/execa): A better `child_process`
- [fs-extra](https://ghub.io/fs-extra): fs-extra contains methods that aren&#39;t included in the vanilla Node.js fs package. Such as mkdir -p, cp -r, and rm -rf.
- [globby](https://ghub.io/globby): Extends `glob` with support for multiple patterns and exposes a Promise API
- [pretty-bytes](https://ghub.io/pretty-bytes): Convert bytes to a human readable string: 1337 → 1.34 kB

## Dev Dependencies

- [@types/execa](https://ghub.io/@types/execa): TypeScript definitions for execa
- [@types/fs-extra](https://ghub.io/@types/fs-extra): TypeScript definitions for fs-extra
- [@types/globby](https://ghub.io/@types/globby): TypeScript definitions for globby
- [@types/node](https://ghub.io/@types/node): TypeScript definitions for Node.js
- [nexe](https://ghub.io/nexe): Create a single executable out of your Node.js application
- [prettier](https://ghub.io/prettier): Prettier is an opinionated code formatter
- [rimraf](https://ghub.io/rimraf): A deep deletion module for node (like `rm -rf`)
- [tslint](https://ghub.io/tslint): An extensible static analysis linter for the TypeScript language
- [typescript](https://ghub.io/typescript): TypeScript is a language for application scale JavaScript development

## License

MIT