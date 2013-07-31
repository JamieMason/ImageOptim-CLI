# current version of ImageOptim-CLI from package.json
version="{{version}}"

# to be set to "stdin" or "directory" based on cli options or piped input
runMode="stdin"

# path to image directory we should optimise
imgPath="{{imgPath}}"

# path to this executable
cliPath="{{cliPath}}"

# "true"|"false"
useImageOptim="{{useImageOptim}}"
useImageAlpha="{{useImageAlpha}}"
useJPEGmini="{{useJPEGmini}}"
imageOptimIsRunning="{{imageOptimIsRunning}}"

# quit apps after processing images?
quitOnComplete="{{quitOnComplete}}"

# how long we will wait before checking again if an app is busy
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
jpegMiniAppRetailBundleId="{{jpegMiniAppRetailBundleId}}"
jpegMiniAppName="{{jpegMiniAppName}}"
jpegMiniAppFileName="{{jpegMiniAppFileName}}"
