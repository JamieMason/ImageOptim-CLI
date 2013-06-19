# current version of ImageOptim-CLI from package.json
version="{{version}}"

undefinedRunMode="{{undefinedRunMode}}"

# to be set to "stdin" or "directory" based on cli options or piped input
runMode="stdin"

# path to image directory we should optimise
imgPath="{{imgPath}}"

# path to this executable
cliPath="{{cliPath}}"

# run ImageOptim.app?
useImageOptim="{{useImageOptim}}"

# run ImageAlpha.app before ImageOptim.app?
useImageAlpha="{{useImageAlpha}}"

# run JPEGmini.app after ImageOptim.app?
useJPEGmini="{{useJPEGmini}}"

# quit ImageOptim.app after processing images?
quitOnComplete="{{quitOnComplete}}"

# how long we will wait before checking again if ImageOptim.app is busy
isBusyIntervalLength={{isBusyIntervalLength}}

# ImageAlpha
imageAlphaFileTypes="{{imageAlphaFileTypes}}"
imageAlphaAppBundleId="{{imageAlphaAppBundleId}}"
imageAlphaAppName="{{imageAlphaAppName}}"
imageAlphaAppFileName="{{imageAlphaAppFileName}}"

# ImageOptim
imageOptimFileTypes="{{imageOptimFileTypes}}"
imageOptimAppBundleId="{{imageOptimAppBundleId}}"
imageOptimAppName="{{imageOptimAppName}}"
imageOptimAppFileName="{{imageOptimAppFileName}}"

# JPEGmini
jpegMiniFileTypes="{{jpegMiniFileTypes}}"
jpegMiniAppBundleId="{{jpegMiniAppBundleId}}"
jpegMiniAppName="{{jpegMiniAppName}}"
jpegMiniAppFileName="{{jpegMiniAppFileName}}"
