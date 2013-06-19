# (): Display usage information
function usage {
  echo "Usage: imageOptim [options]"
  echo ""
  echo "Options:"
  echo ""
  echo "  -d, --directory     directory of images to process"
  echo "  -a, --image-alpha   pre-process PNGs with ImageAlpha.app (http://pngmini.com)"
  echo "  -j, --jpeg-mini     post-process JPGs with JPEGmini.app (https://itunes.apple.com/us/app/jpegmini/id498944723"
  echo "  -q, --quit          quit ImageOptim.app when complete"
  echo "  -h, --help          output usage information"
  echo "  -e, --examples      output usage examples"
  echo "  -v, --version       output the version number"
}

# (): Display usage examples
function examples {
  echo "Examples:"
  echo ""
  echo "Run ImageAlpha, ImageOptim, JPEGmini, then quit"
  echo "$ imageOptim --jpeg-mini --image-alpha --quit --directory path/to/images"
  echo "$ imageOptim -j -a -q -d path/to/images"
  echo ""
  echo "Run ImageOptim only"
  echo "$ imageOptim --directory path/to/images"
  echo "$ imageOptim -d path/to/images"
}

# ($1:message): Display a red error message and quit
function error {
  printf "\e[31m✘ $1"
  echo "\033[0m"
  exit 1
}

# ($1:message): Display a message in green with a tick by it
function success {
  printf "\e[32m✔ ${1}"
  echo "\033[0m"
}

# (): Get a timestamp for the current time
function now {
  date +"%s"
}

# ($1:appName): Get the number of processes in use by an Application
function countProcesses {
  printf $(ps -aef | grep  "[${1:0:1}]${1:1}" | wc -l)
}

# ($1:appName): Sleep until app is done optimising images
function waitForApp {
  # wait for App to spawn a few processes
  sleep 2
  # wait until those processes have completed
  while [[ "$(countProcesses "$1")" > "1" ]]; do
    sleep $isBusyIntervalLength
  done
}

# (): How long did ImageOptim.app take to process the images?
function getTimeSpent {
  let timeSpent=endTime-startTime-$isBusyIntervalLength
  echo $timeSpent
}

# (): How many images are in the directory we're about to process?
function getImgCount {
  echo $(find -E "$imgPath" -iregex $imageOptimFileTypes | wc -l)
}

# ($1:appFileName, $2:imageFilePath):
function addImageToQueue {
  open -g -a $1 "$2"
}

# ($1:fileTypes, $2:appFileName): Queue direcory of images
function addDirectoryToQueue {
  find -E "$imgPath" -regex $1 -print0 | while IFS= read -r -d $'\0' img; do
    addImageToQueue $2 "$img"
  done
}

# ($1:appShouldBeRun, $2:appName, $3:fileTypes, $4:appFileName):
function runPornelAppOnDirectory {
  if [ "true" == $1 ]; then
    addDirectoryToQueue $3 $4
    waitForApp $2
    if [ "true" == $quitOnComplete ]; then
      osascript -e "tell application \"$2\" to quit"
    fi
  fi
}

# ():
function runImageOptimOnDirectory {
  runPornelAppOnDirectory $useImageOptim $imageOptimAppName $imageOptimFileTypes $imageOptimAppFileName
}

# ():
function runImageAlphaOnDirectory {
  runPornelAppOnDirectory $useImageAlpha $imageAlphaAppName $imageAlphaFileTypes $imageAlphaAppFileName
}

# ($1:appShouldBeRun, $2:appName, $3:fileTypes, $4:appFileName, $5:image):
function runPornelAppOnImage {
  if [ "true" == $1 ]; then
    addImageToQueue $4 "$5"
    waitForApp $2
    if [ "true" == $quitOnComplete ]; then
      osascript -e "tell application \"$2\" to quit"
    fi
  fi
}

# ($1:image):
function runImageOptimOnImage {
  runPornelAppOnImage $useImageOptim $imageOptimAppName $imageOptimFileTypes $imageOptimAppFileName "$1"
}

# ($1:image):
function runImageAlphaOnImage {
  runPornelAppOnImage $useImageAlpha $imageAlphaAppName $imageAlphaFileTypes $imageAlphaAppFileName "$1"
}

# ($1:path):
function runJPEGmini {
  if [ "true" == $useJPEGmini ]; then
    `osascript "$cliPath/imageOptimAppleScriptLib" run_jpegmini "$1" $jpegMiniAppName` > /dev/null 2>&1
    sleep 1
    `osascript "$cliPath/imageOptimAppleScriptLib" wait_for $jpegMiniAppName` > /dev/null 2>&1
    if [ "true" == $quitOnComplete ]; then
      osascript -e "tell application \"$jpegMiniAppName\" to quit"
    fi
  fi
}

# (): if an override is not set, get path to this executable
function initCliPath {
  if [ "false" == $cliPath ]; then
    cliPath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
  fi
}

# (): quit if -d, --directory does not resolve
function validateImgPath {
  if [ "directory" == $runMode ] && [ ! -d "$imgPath" ]; then
    error "{{invalidDirectoryMsg}}"
  fi
}

# ($1:appBundleId): eg. "net.pornel.ImageAlpha" -> "ImageAlpha.app" or "NOT_INSTALLED"
function getAppFileNameByBundleId {
  echo `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $1`
}

# ($1:appFileName, $2:appBundleId): -> "true" or "false"
function appIsInstalled {
  if [ $1 == $(getAppFileNameByBundleId $2) ]; then
    echo "true"
  else
    echo "false"
  fi
}

# (): -> "true" or "false"
function imageOptimIsInstalled {
  echo $(appIsInstalled $imageOptimAppFileName $imageOptimAppBundleId)
}

# (): -> "true" or "false"
function imageAlphaIsInstalled {
  echo $(appIsInstalled $imageAlphaAppFileName $imageAlphaAppBundleId)
}

# (): -> "true" or "false"
function jpegMiniIsInstalled {
  if [ "true" == $(appIsInstalled $jpegMiniAppFileName $jpegMiniAppBundleId) ] || [ "true" == $(appIsInstalled $jpegMiniAppFileName $jpegMiniAppRetailBundleId) ]; then
    echo "true"
  else
    echo "false"
  fi
}

# (): -> "true" or "false"
function guiScriptIsEnabled {
  echo `osascript "$cliPath/imageOptimAppleScriptLib" has_gui_script`
}

# ($1:appShouldBeRun, $2:appIsInstalled, $3:isNotInstalledMsg):
function errorIfNotInstalled {
  if [ "true" == $1 ] && [ "false" == $2 ]; then
    error "$3"
  fi
}

# (): quit if ImageOptim should be run but is not installed
function validateImageOptim {
  errorIfNotInstalled $useImageOptim $(imageOptimIsInstalled) "{{imageOptimNotInstalledMsg}}"
}

# (): quit if ImageAlpha should be run but is not installed
function validateImageAlpha {
  errorIfNotInstalled $useImageAlpha $(imageAlphaIsInstalled) "{{imageAlphaNotInstalledMsg}}"
}

# (): quit if ImageAlpha should be run but is not installed or cannot run
function validateJpegMini {

  # if we're not running JPEGmini then it's all good
  if [ "false" == $useJPEGmini ]; then
    return 0
  fi

  # if we are and it's not installed
  if [ "false" == $(jpegMiniIsInstalled) ]; then
    error "{{jpegMiniNotInstalledMsg}}"
  fi

  # if we are, it's installed but GUIScript is not available
  if [ "false" == $(guiScriptIsEnabled) ]; then
    error "{{guiScriptIsDisabledMsg}}"
  fi

}

# (): run applications against a directory of images
function processDirectory {
  startTime=$(now)
  echo "Processing $(getImgCount) images..."
  runImageAlphaOnDirectory
  runImageOptimOnDirectory
  runJPEGmini "$imgPath"
  endTime=$(now)
  success "Finished in $(getTimeSpent) seconds" | xargs
}

# (): run applications against a single image
function processFiles {

  # @TODO: seperate queuing from waiting on apps to finish
  # so we can first queue up many files, then afterwards start waiting

  while read LINE; do
    echo "Processing $LINE..."
    if [ "" != "`echo "$LINE" | grep -E '{{imageAlphaFileTypes}}'`" ]; then
      runImageAlphaOnImage "$LINE"
    elif [ "" != "`echo "$LINE" | grep -E '{{imageOptimFileTypes}}'`" ]; then
      runImageOptimOnImage "$LINE"
    elif [ "" != "`echo "$LINE" | grep -E '{{jpegMiniFileTypes}}'`" ]; then
      runJPEGmini "$LINE"
    else
      echo "Ignored: $LINE"
    fi
    success "Finished processing $LINE"
  done
}
