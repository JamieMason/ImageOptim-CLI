# (): Display usage information
function usage {
  echo "Usage: imageOptim [options]"
  echo ""
  echo "Options:"
  echo ""
  echo "  -d, --directory     directory of images to process"
  echo "  -a, --image-alpha   pre-process PNGs with ImageAlpha.app (http://pngmini.com)"
  echo "  -j, --jpeg-mini     process JPGs with JPEGmini.app (https://itunes.apple.com/us/app/jpegmini/id498944723"
  echo "  -q, --quit          quit ImageOptim.app when complete"
  echo "  -h, --help          output usage information"
  echo "  -e, --examples      output usage examples"
  echo "  -v, --version       output the version number"
}

# (): Display usage examples
function examples {
  echo "Examples:"
  echo ""
  echo "Run ImageAlpha, ImageOptim & JPEGmini"
  echo "$ imageOptim -j -a -d path/to/images"
  echo "$ imageOptim --jpeg-mini --image-alpha --directory path/to/images"
  echo ""
  echo "Run ImageAlpha & ImageOptim"
  echo "$ imageOptim -a -d path/to/images"
  echo "$ imageOptim --image-alpha --directory path/to/images"
  echo ""
  echo "Run ImageOptim"
  echo "$ imageOptim -d path/to/images"
  echo "$ imageOptim --directory path/to/images"
  echo ""
  echo "Run ImageAlpha, ImageOptim, JPEGmini & quit them when finished"
  echo "$ imageOptim -j -q -a -d path/to/images"
  echo "$ imageOptim --jpeg-mini --quit --image-alpha --directory path/to/images"
  echo ""
  echo "Run ImageAlpha, ImageOptim & quit them when finished"
  echo "$ imageOptim -q -a -d path/to/images"
  echo "$ imageOptim --quit --image-alpha --directory path/to/images"
  echo ""
  echo "Run ImageOptim & quit it when finished"
  echo "$ imageOptim -q -d path/to/images"
  echo "$ imageOptim --quit --directory path/to/images"
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
function waitFor {
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

# (): Send all images to ImageOptim.app for processing
function populateImageOptimQueue {
  find -E "$imgPath" -regex $imageOptimFileTypes -print0 | while IFS= read -r -d $'\0' img; do
    open -g -a ImageOptim.app "$img"
  done
}

# (): Send all images to ImageOptim.app for processing
function populateImageAlphaQueue {
  find -E "$imgPath" -regex $imageAlphaFileTypes -print0 | while IFS= read -r -d $'\0' img; do
    open -g -a ImageAlpha.app "$img"
  done
}

# (): if an override is not set, get path to this executable
function initCliPath {
  if [ "false" == $cliPath ]; then
    cliPath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
  fi
}

# (): quit if -d, --directory option does not resolve to a directory
function validateImageDirectory {
  if [ ! -d "$imgPath" ]; then
    error "Could not find directory $imgPath"
  fi
}

# (): quit if ImageOptim should be run but is not installed
function validateImageOptim {
  if [ "true" == $runImageOptim ]; then
    if [ "$imageOptimAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $imageOptimAppBundleId` ]; then
      error "$imageOptimAppFileName is not installed (http://imageoptim.com)"
    fi
  fi
}

# (): quit if ImageAlpha should be run but is not installed
function validateImageAlpha {
  if [ "true" == $runImageAlpha ]; then
    if [ "$imageAlphaAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $imageAlphaAppBundleId` ]; then
      error "$imageAlphaAppFileName is not installed (http://pngmini.com)"
    fi
  fi
}

# (): quit if ImageAlpha should be run but is not installed or cannot run
function validateJpegMini {
  if [ "true" == $runJPEGmini ]; then
    if [ "$jpegMiniAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $jpegMiniAppBundleId` ]; then
      error "$jpegMiniAppFileName is not installed (https://itunes.apple.com/us/app/jpegmini/id498944723)"
    fi
    if [ "true" != `osascript "$cliPath/imageOptimAppleScriptLib" has_gui_script` ]; then
      error "To automate JPEGmini we need to enable GUI Scripting, check 'Enable access for assistive devices' under Accessibility in System Preferences, then run ImageOptim-CLI again"
    fi
  fi
}
