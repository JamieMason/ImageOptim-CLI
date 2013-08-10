# see https://github.com/pornel/ImageOptim/issues/24
# the first file to be added is not processed if ImageOptim
# is not already running
function startImageOptim {
  if [ $(countProcesses $imageOptimAppName) -eq 0 ]; then
    echo "ImageOptim not running - starting"
    open -g -a $imageOptimAppFileName
    sleep 1
    imageOptimIsRunning="true"
  fi
}

# ($1:appFileName, $2:imageFilePath):
function addImageToQueue {

  if [[ $imageOptimIsRunning == "false" ]] && [[ "$1" == "$imageOptimAppFileName" ]]; then
    startImageOptim
  fi

  open -g -a $1 "$2"
}

# ($1:fileTypes, $2:appFileName, $3:dirPath): Queue direcory of images
function addDirectoryToQueue {
  find -E "$3" -iregex $1 -print0 | while IFS= read -r -d $'\0' imgPath; do
    addImageToQueue $2 "$imgPath"
  done
}

# ($1:appShouldBeRun, $2:appName, $3:fileTypes, $4:appFileName, $5:dirPath):
function runPornelAppOnDirectory {
  if [ "true" == $1 ]; then
    echo $2
    addDirectoryToQueue $3 $4 "$5"
  fi
}

# ($1:dirPath):
function runImageOptimOnDirectory {
  runPornelAppOnDirectory $useImageOptim $imageOptimAppName $imageOptimFileTypes $imageOptimAppFileName "$1"
}

# ($1:dirPath):
function runImageAlphaOnDirectory {
  if [ "true" == $useImageAlpha ]; then
    echo "ImageAlpha..."
    find -E "$1" -iregex '{{imageAlphaFileTypes}}' -print0 | while IFS= read -r -d $'\0' img; do
      runImageAlphaOnImage "$img"
    done
  fi
}

# ($1:appShouldBeRun, $2:appName, $3:fileTypes, $4:appFileName, $5:image):
function runPornelAppOnImage {
  if [ "true" == $1 ]; then
    addImageToQueue $4 "$5"
  fi
}

# ($1:image):
function runImageOptimOnImage {
  runPornelAppOnImage $useImageOptim $imageOptimAppName $imageOptimFileTypes $imageOptimAppFileName "$1"
}

# ($1:image):
function runImageAlphaOnImage {

  # would like to specify the extension here, but pngquant ignores case
  # if the extension is .PNG, then this command creates a new file
  # the new file is not passed to imageOptim
  # waiting on https://github.com/pornel/pngquant/issues/45
  # fix coming soon: https://github.com/pornel/pngquant/commit/057eb1ee9e9890a74ea363ae367df16feaa4742b
  #extension="${1##*.}"

  /Applications/ImageAlpha.app/Contents/Resources/pngquant --ext=.png --force --speed=1 --quality=75-100 -- "$1"
}

# ($1:path):
function runJPEGmini {
  if [ "true" == $useJPEGmini ]; then
    echo "JPEGmini..."
    `osascript "$cliPath/imageOptimAppleScriptLib" run_jpegmini "$1" $jpegMiniAppName` > /dev/null 2>&1
  fi
}
