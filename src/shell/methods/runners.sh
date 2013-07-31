# (): Get a timestamp for the current time
function now {
  date +"%s"
}

# (): How long did ImageOptim.app take to process the images?
function getTimeSpent {
  let timeSpent=endTime-startTime-$isBusyIntervalLength
  echo $timeSpent
}

# ($1:dirPath): How many images are in the directory we're about to process?
function getImgCount {
  echo $(find -E "$1" -iregex $imageOptimFileTypes | wc -l)
}

# (): run applications against a directory of images
function processDirectory {
  startTime=$(now)
  imgCount=$(getImgCount "$imgPath")
  echo "Processing $imgCount images..."

  runImageAlphaOnDirectory "$imgPath"

  runJPEGmini "$imgPath"
  waitForJPEGmini

  runImageOptimOnDirectory "$imgPath"
  waitForImageOptim

  endTime=$(now)
  success "Finished in $(getTimeSpent) seconds" | xargs
}

# (): run applications against a single image
function processFiles {
  i=0;

  # store piped input so we can iterate over it more than once
  while read LINE; do
    pipedFiles[$i]="${LINE}"
    i=$((i+1))
  done

  echo "Processing $i images..."

  # Case-insensitive pattern matching
  # for use with the =~ Regular Expression matching operator
  # http://tldp.org/LDP/abs/html/bashver3.html#REGEXMATCHREF
  shopt -s nocasematch

  for file in "${pipedFiles[@]}"; do
    if [[ "$file" =~ {{imageAlphaFileTypes}} ]]; then
      runImageAlphaOnImage "$file"
    fi
  done

  for file in "${pipedFiles[@]}"; do
    if [[ "$file" =~ {{jpegMiniFileTypes}} ]]; then
      runJPEGmini "$file"
    fi
  done

  waitForJPEGmini

  for file in "${pipedFiles[@]}"; do
    if [[ "$file" =~ {{imageOptimFileTypes}} ]]; then
      runImageOptimOnImage "$file"
    fi
  done
  
  # unset case-insensitive pattern matching
  shopt -u nocasematch
  
  waitForImageOptim
}
