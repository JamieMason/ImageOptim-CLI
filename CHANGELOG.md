# Change Log

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
