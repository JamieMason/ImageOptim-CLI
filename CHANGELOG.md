## [3.1.9](https://github.com/JamieMason/ImageOptim-CLI/compare/3.1.7...3.1.9) (2023-11-06)


### Bug Fixes

* **engines:** support node >= 18 ([5c18d6c](https://github.com/JamieMason/ImageOptim-CLI/commit/5c18d6cab4a0dbcd2d0ecc26e1a5df114710af13))
* **npm:** update dependencies ([201e63f](https://github.com/JamieMason/ImageOptim-CLI/commit/201e63f8d06281bd3499283f3b74f20ac5d9dfa9))



## [3.1.7](https://github.com/JamieMason/ImageOptim-CLI/compare/3.0.7...3.1.7) (2023-07-02)


### Features

* **node:** upgrade to node v18 ([40dfb41](https://github.com/JamieMason/ImageOptim-CLI/commit/40dfb41c533626bbc32c5baec3f492679bc1013d))



## [3.0.7](https://github.com/JamieMason/ImageOptim-CLI/compare/3.0.2...3.0.7) (2021-11-13)


### Bug Fixes

* **cli:** use unique temp dir per invocation ([5fac801](https://github.com/JamieMason/ImageOptim-CLI/commit/5fac801a817cee90c51adb1dbde1753b4909a9af)), closes [#183](https://github.com/JamieMason/ImageOptim-CLI/issues/183)
* **jpegmini:** add com.beamr.jpegminipro.app ([d865434](https://github.com/JamieMason/ImageOptim-CLI/commit/d865434095a630d5a831ce4c70092c767cf8eb29)), closes [#187](https://github.com/JamieMason/ImageOptim-CLI/issues/187)
* **jpegmini:** add support for macOS Monterey ([7060d05](https://github.com/JamieMason/ImageOptim-CLI/commit/7060d05ee5d8d0cdcaf31f11c10582ac2308d515))
* **nexe:** fix __dirname issues in v3.3.7 ([aac53ac](https://github.com/JamieMason/ImageOptim-CLI/commit/aac53acd6edb5e568a7798518558b5476b82caf5))
* **npm:** update dependencies ([59cdebc](https://github.com/JamieMason/ImageOptim-CLI/commit/59cdebce5ba4f1627b8d9b4dee4973377202ae26)), closes [#191](https://github.com/JamieMason/ImageOptim-CLI/issues/191)



## [3.0.2](https://github.com/JamieMason/ImageOptim-CLI/compare/3.0.0...3.0.2) (2019-11-04)


### Bug Fixes

* **cli:** ensure temp files are always removed ([7413b22](https://github.com/JamieMason/ImageOptim-CLI/commit/7413b227d681b851018ab479ecfbc195d3f76da6))
* **pngquant:** handle upstream errors ([9557b8f](https://github.com/JamieMason/ImageOptim-CLI/commit/9557b8fce4b8c0a5368ddab9092790988cf4aba6)), closes [#182](https://github.com/JamieMason/ImageOptim-CLI/issues/182)



# [3.0.0](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.9...3.0.0) (2019-09-15)


### Bug Fixes

* **cli:** exit with error status if an app is not installed ([6c3d799](https://github.com/JamieMason/ImageOptim-CLI/commit/6c3d799c7e94037522d6f19d0e6ef3f0ca08c833)), closes [#180](https://github.com/JamieMason/ImageOptim-CLI/issues/180)
* **cli:** handle upper case file extensions ([e7c84ac](https://github.com/JamieMason/ImageOptim-CLI/commit/e7c84ac0eb0f34cf34f3a69ea8771257f9e51f37)), closes [#181](https://github.com/JamieMason/ImageOptim-CLI/issues/181)
* **npm:** update npm dependencies ([b492ad2](https://github.com/JamieMason/ImageOptim-CLI/commit/b492ad2ad29a16ed46cc0559a6b3f66a34dda5f0))


### Performance Improvements

* **cli:** reduce memory consumption ([3166245](https://github.com/JamieMason/ImageOptim-CLI/commit/316624513ddf18ce7948cc58eb0f4d768affeca3)), closes [#173](https://github.com/JamieMason/ImageOptim-CLI/issues/173)


### BREAKING CHANGES

* **cli:** The following scenarios will display an error and exit with a status
code of 1, where previously they would display a warning:

+ Not using `--no-imageoptim` when ImageOptim.app is not installed.
+ Using `--imagealpha` when ImageAlpha.app is not installed.
+ Using `--jpegmini` when JPEGmini.app is not installed.



## [2.3.9](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.8...2.3.9) (2019-07-16)


### Bug Fixes

* **npm:** potential security vulnerability ([cd48880](https://github.com/JamieMason/ImageOptim-CLI/commit/cd4888044f012d363ebbeb52bfe6ff1861bcf36f))



## [2.3.8](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.7...2.3.8) (2019-07-16)


### Bug Fixes

* **npm:** potential security vulnerability ([1560041](https://github.com/JamieMason/ImageOptim-CLI/commit/1560041c5344c3eb17254220654c1d0e2db38597)), closes [#178](https://github.com/JamieMason/ImageOptim-CLI/issues/178)



## [2.3.7](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.6...2.3.7) (2019-07-01)


### Bug Fixes

* **npm:** revert nexe update ([c3aa684](https://github.com/JamieMason/ImageOptim-CLI/commit/c3aa68425366d672852af495e6e650aa6c7f8934)), closes [#176](https://github.com/JamieMason/ImageOptim-CLI/issues/176) [#163](https://github.com/JamieMason/ImageOptim-CLI/issues/163)



## [2.3.6](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.5...2.3.6) (2019-06-17)


### Bug Fixes

* **npm:** update dependencies ([a154af0](https://github.com/JamieMason/ImageOptim-CLI/commit/a154af09214d97d03a7b1d657cf7ff98230b709e))



## [2.3.5](https://github.com/JamieMason/ImageOptim-CLI/compare/2.3.4...2.3.5) (2019-02-02)


### Bug Fixes

* **npm:** update npm dependencies ([16897a0](https://github.com/JamieMason/ImageOptim-CLI/commit/16897a09755cc9a94b88d82f2cf46521f5f62dc9)), closes [#172](https://github.com/JamieMason/ImageOptim-CLI/issues/172)



## [2.3.4](https://github.com/JamieMason/ImageOptim-CLI/compare/2.0.4...2.3.4) (2018-12-23)


### Features

* **cli:** only output summary when savings are made ([b44c078](https://github.com/JamieMason/ImageOptim-CLI/commit/b44c07862d8b4267c5f2ef5ed896b707b70c2f4d))
* **cli:** show warning when ImageOptim.app is not installed ([44f3055](https://github.com/JamieMason/ImageOptim-CLI/commit/44f30551dfb5fc886614eca0ea85e665c87eb7f6))
* **nexe:** upgrade to node.js 10.13.0 and es2017 ([dddd9ff](https://github.com/JamieMason/ImageOptim-CLI/commit/dddd9ffecc791b654add496f4d402b0df4c01be0))



## [2.0.4](https://github.com/JamieMason/ImageOptim-CLI/compare/2.0.3...2.0.4) (2018-11-07)


### Bug Fixes

* **npm:** update dependencies ([a5b3ec8](https://github.com/JamieMason/ImageOptim-CLI/commit/a5b3ec820b2b6dff4ba20f3823b98ddee1ed6b5b))



## [2.0.3](https://github.com/JamieMason/ImageOptim-CLI/compare/2.0.2...2.0.3) (2018-06-13)


### Bug Fixes

* **npm:** update dependencies ([3e1259f](https://github.com/JamieMason/ImageOptim-CLI/commit/3e1259ff0bfe192739468cc92b0e321377f1e78f)), closes [#166](https://github.com/JamieMason/ImageOptim-CLI/issues/166)



## [2.0.2](https://github.com/JamieMason/ImageOptim-CLI/compare/2.0.0...2.0.2) (2018-05-14)


### Bug Fixes

* **cli:** rename bin to imageoptim in help output ([534c87f](https://github.com/JamieMason/ImageOptim-CLI/commit/534c87f66c71cc2173d3456e4e9a017a0bde1954))
* **nexe:** fix __dirname not behaving as expected ([05b9ca3](https://github.com/JamieMason/ImageOptim-CLI/commit/05b9ca39b17cf96016aecce4c0068463923f7b38)), closes [#163](https://github.com/JamieMason/ImageOptim-CLI/issues/163)



# [2.0.0](https://github.com/JamieMason/ImageOptim-CLI/compare/1.15.4...2.0.0) (2018-05-13)


### Features

* **core:** rewrite imageoptim-cli in node.js ([3faa790](https://github.com/JamieMason/ImageOptim-CLI/commit/3faa790f9088e5988eccb897a67111dd54d4f57d)), closes [#137](https://github.com/JamieMason/ImageOptim-CLI/issues/137) [#140](https://github.com/JamieMason/ImageOptim-CLI/issues/140) [#141](https://github.com/JamieMason/ImageOptim-CLI/issues/141) [#148](https://github.com/JamieMason/ImageOptim-CLI/issues/148) [#159](https://github.com/JamieMason/ImageOptim-CLI/issues/159)



## [1.15.4](https://github.com/JamieMason/ImageOptim-CLI/compare/1.15.3...1.15.4) (2017-11-22)


### Bug Fixes

* **osx:** add support for high sierra ([e25bfce](https://github.com/JamieMason/ImageOptim-CLI/commit/e25bfcefe3d4ae6bd8b3a823f6583228691362f0))



## [1.15.3](https://github.com/JamieMason/ImageOptim-CLI/compare/1.15.1...1.15.3) (2017-11-14)


### Bug Fixes

* **duplicates:** support multibyte characters ([43d6ce7](https://github.com/JamieMason/ImageOptim-CLI/commit/43d6ce71dfae591fafdf2ace825aad9eedbfa862))
* **osx:** add support for high sierra ([6208dc4](https://github.com/JamieMason/ImageOptim-CLI/commit/6208dc44721cd82c71f3e16e9795b3ca503c481e)), closes [#152](https://github.com/JamieMason/ImageOptim-CLI/issues/152)



## [1.15.1](https://github.com/JamieMason/ImageOptim-CLI/compare/1.14.9...1.15.1) (2017-06-29)


### Bug Fixes

* **file index:** handle cases where no files are found or passed in ([b9b4f7a](https://github.com/JamieMason/ImageOptim-CLI/commit/b9b4f7ae3c3c6960e87fdf62d18e388c3f5bd255))


### Features

* **pngquant:** binary update to 2.9.1 ([8182b51](https://github.com/JamieMason/ImageOptim-CLI/commit/8182b5103cae1d0fa7eb81d35a610d0302328be6))



## [1.14.9](https://github.com/JamieMason/ImageOptim-CLI/compare/1.14.8...1.14.9) (2017-02-05)


### Bug Fixes

* **jpegmini:** the text field in sierra has changed to a combo box ([2f711bb](https://github.com/JamieMason/ImageOptim-CLI/commit/2f711bbd5dcd9eae69368cdf161f656e1eb85c4a)), closes [#129](https://github.com/JamieMason/ImageOptim-CLI/issues/129)



## [1.14.8](https://github.com/JamieMason/ImageOptim-CLI/compare/1.11.4...1.14.8) (2016-02-09)


### Bug Fixes

* **examples:** remove shell syntax highlighting ([0344ae7](https://github.com/JamieMason/ImageOptim-CLI/commit/0344ae77be322d6c59a137d306db084f45af0e18))
* **filesystem:** remove additional trailing slash after $TMPDIR ([6b60e32](https://github.com/JamieMason/ImageOptim-CLI/commit/6b60e32f6bec1f0a20d6347652eee4c446a4f08c)), closes [#87](https://github.com/JamieMason/ImageOptim-CLI/issues/87)


### Features

* **logging:** per-file reporting is now opt-in via a --verbose flag ([4c9a489](https://github.com/JamieMason/ImageOptim-CLI/commit/4c9a489dd351dc8065ce118e04b930dedd4ceefe))
* **pngquant:** add --skip-if-larger option ([9904253](https://github.com/JamieMason/ImageOptim-CLI/commit/9904253f7e16a5ebc70f6dcc3a4798183630d316))
* **pngquant:** replace hard-coded value with -m/--min-quality param ([699d8c8](https://github.com/JamieMason/ImageOptim-CLI/commit/699d8c80ea6d767b275806c20bf7a7c27c6f5de4))



## [1.11.4](https://github.com/JamieMason/ImageOptim-CLI/compare/1.11.3...1.11.4) (2014-11-04)



## [1.11.3](https://github.com/JamieMason/ImageOptim-CLI/compare/1.11.2...1.11.3) (2014-10-31)



## [1.11.2](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.11...1.11.2) (2014-10-31)



## [1.7.11](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.10...1.7.11) (2014-01-23)



## [1.7.10](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.9...1.7.10) (2013-12-11)



## [1.7.9](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.3...1.7.9) (2013-12-10)



## [1.7.3](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.2...1.7.3) (2013-11-17)



## [1.7.2](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.1...1.7.2) (2013-11-12)



## [1.7.1](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.0...1.7.1) (2013-11-10)



# [1.7.0](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.19...1.7.0) (2013-11-10)



## [1.6.19](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.18...1.6.19) (2013-08-10)



## [1.6.18](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.15...1.6.18) (2013-07-31)



## [1.6.15](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.14...1.6.15) (2013-07-30)



## [1.6.14](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.13...1.6.14) (2013-07-16)



## [1.6.13](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.12...1.6.13) (2013-07-14)



## [1.6.12](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.11...1.6.12) (2013-07-01)



## [1.6.11](https://github.com/JamieMason/ImageOptim-CLI/compare/1.4.5...1.6.11) (2013-06-30)



## [1.4.5](https://github.com/JamieMason/ImageOptim-CLI/compare/1.4.4...1.4.5) (2013-06-19)



## [1.4.4](https://github.com/JamieMason/ImageOptim-CLI/compare/1.4.3...1.4.4) (2013-06-13)



## [1.4.3](https://github.com/JamieMason/ImageOptim-CLI/compare/1.4.2...1.4.3) (2013-06-13)



## [1.4.2](https://github.com/JamieMason/ImageOptim-CLI/compare/1.4.1...1.4.2) (2013-06-10)



## [1.4.1](https://github.com/JamieMason/ImageOptim-CLI/compare/1.3.2...1.4.1) (2013-06-10)



## [1.3.2](https://github.com/JamieMason/ImageOptim-CLI/compare/1.3.1...1.3.2) (2013-06-08)



## [1.3.1](https://github.com/JamieMason/ImageOptim-CLI/compare/1.1.5...1.3.1) (2013-05-26)



## [1.1.5](https://github.com/JamieMason/ImageOptim-CLI/compare/1.1.4...1.1.5) (2013-05-24)



## [1.1.4](https://github.com/JamieMason/ImageOptim-CLI/compare/1.1.3...1.1.4) (2013-05-22)



## [1.1.3](https://github.com/JamieMason/ImageOptim-CLI/compare/1.1.2...1.1.3) (2013-05-20)



## [1.1.2](https://github.com/JamieMason/ImageOptim-CLI/compare/1.1.1...1.1.2) (2013-05-19)



## [1.1.1](https://github.com/JamieMason/ImageOptim-CLI/compare/1.0.4...1.1.1) (2013-05-19)



## [1.0.4](https://github.com/JamieMason/ImageOptim-CLI/compare/1.0.0...1.0.4) (2013-05-18)



# [1.0.0](https://github.com/JamieMason/ImageOptim-CLI/compare/0.0.6...1.0.0) (2013-05-17)



## 0.0.6 (2013-03-03)



