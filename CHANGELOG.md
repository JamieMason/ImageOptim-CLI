# Change Log

**1.7.11** — Jan. 23, 2014 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.10...1.7.11)

+ Add FAQs and related projects to README.

**1.7.10** — Dec. 11, 2013 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.9...1.7.10)

+ Expand paths to avoid issues with some formats of relative paths (#65).

**1.7.9** — Dec. 10, 2013 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.3...1.7.9)

+ Added support for JPEGmini Lite (#41).
+ Changed CLI to exit instead of displaying a warning if the `--directory` option is not valid.
+ Fixed issue in Mavericks where JPEGmini would open but not select any files (#60).
+ Changed to run ImageOptim in CLI mode instead of using AppleScript, this removes support for Leopard and older (#61, #62, 53).
+ Fix minor formatting issues in Terminal output.
+ Ensure trailing slash on $TMPDIR.

**1.7.3** — Nov. 17, 2013 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.2...1.7.3)

+ Applied bug fix for checking whether ImageOptim is installed.

**1.7.2** — Nov. 12, 2013 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.7.1...1.7.2)

+ Applied bug fix for checking whether ImageOptim is installed.

**1.7.1** — Nov. 10, 2013 — [Diff](https://github.com/JamieMason/ImageOptim-CLI/compare/1.6.19...1.7.1)

+ Started keeping this CHANGELOG.md.
+ Improved performance when automating JPEGmini with files received via stdin.
+ Added a colourful summary of file sizes before and after optimisation — plus what percentage of the original file was removed:

        images/Totoro.png was: 95.072kb now: 31.501kb saving: 63.571kb (66.00%)

+ If [ImageMagick](http://www.imagemagick.org) is installed, the summary will also include what percentage of the original file's quality was retained during optimisation.

        images/Totoro.png was: 95.072kb now: 31.501kb saving: 63.571kb (66.00%) quality: 96.06224%

+ ImageAlpha's improved pngquant is run over multiple cores for better performance.
+ Fixed issue where JPEGmini can display a prompt regarding it wanting to quit.
+ Fixed issue related to how we observe JPEGmini's system process, affecting only some localisations (many thanks to [@glitchtracker](https://github.com/glitchtracker) for his help).
