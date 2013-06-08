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
  echo "Run ImageOptim"
  echo "$ imageOptim -d path/to/images"
  echo "$ imageOptim --directory path/to/images"
  echo ""
  echo "Run ImageOptim then quit it when finished"
  echo "$ imageOptim -q -d path/to/images"
  echo "$ imageOptim --quit --directory path/to/images"
  echo ""
  echo "Run ImageAlpha then ImageOptim"
  echo "$ imageOptim -a -d path/to/images"
  echo "$ imageOptim --image-alpha --directory path/to/images"
  echo ""
  echo "Run ImageAlpha then ImageOptim then quit ImageOptim when finished"
  echo "$ imageOptim -q -a -d path/to/images"
  echo "$ imageOptim --quit --image-alpha --directory path/to/images"
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
