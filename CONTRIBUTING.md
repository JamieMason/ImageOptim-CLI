# Contributing

## :cloud: Getting Started

1.  Ensure NodeJS version `8.11.1` is installed.
1.  `git clone https://github.com/JamieMason/ImageOptim-CLI.git`.
1.  `npm install`.
1.  `npm run lint`.
1.  `npm run build`.

## :wrench: Technologies

OSX is automated using the [AppleScript][applescript] files in
[./osascript/\*.applescript][osascript] and those scripts are called from NodeJS by
[./src/applescript.ts][applescript.ts].

ImageOptim-CLI is written in [TypeScript][typescript] and converted into a standalone executable
using [nexe] so that NodeJS is not needed by its users. This is all handled by the `npm run build`
command.

## :construction: Developing Locally

Each time you make a change to the TypeScript or AppleScript, run `npm run build` to update the
executable at **./dist/imageoptim**. You can run your local executable from there:

```
./dist/imageoptim --help
```

## :microscope: Testing Before Release

1.  Run `npm pack` to create a tarball at **./imageoptim-cli-2.0.0.tgz**, where `2.0.0` is whatever
    the current `version` is defined as in **./package.json**.
1.  Run `npm install -g ./imageoptim-cli-2.0.0.tgz` to globally install the release candidate.
1.  `npm ls -g --depth 0` will list your release candidate alongside your other global npm
    dependencies.
1.  `imageoptim --help` can be run as normal, the same way it will once published finally.
1.  Remember to run `npm uninstall -g ./imageoptim-cli-2.0.0.tgz` to remove your local release
    candidate afterwards.

<!-- links -->

[applescript.ts]: https://github.com/JamieMason/ImageOptim-CLI/tree/master/src/applescript.ts
[applescript]:
  https://developer.apple.com/library/content/documentation/AppleScript/Conceptual/AppleScriptLangGuide/introduction/ASLR_intro.html
[nexe]: https://github.com/nexe/nexe
[osascript]: https://github.com/JamieMason/ImageOptim-CLI/tree/master/osascript
[typescript]: https://www.typescriptlang.org/
