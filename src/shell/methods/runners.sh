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

# ($1:dirPath): The total size of the images in the directory we're about to process
function getImagesSizeTotal {
  # added quotemeta and xargs in case files have spaces/special chars
  echo $(find -E "$1" -iregex $imageOptimFileTypes | perl -lne 'print quotemeta' | xargs ls | du -k | awk '{ print $1}')
}

# ($1:startSize, $2:endSize: Output savings report
function printReport {

  local startSize="${1:-0}"
  local endSize="${2:-0}"
  local saving=$(echo "(1 - ($endSize / $startSize)) * 100" | bc -l | sed -e 's/^\./0./' )

  if [ ${saving/\.*} -gt 0 ]; then
    printf "Optimised %'d KB to %'d KB (saving %2.2f%%)\n" "$startSize" "$endSize" "$saving"
  else
    printf "No savings. Start: %'d KB End: %'d KB\n" "$startSize" "$endSize" 
  fi
}

# (): run applications against a directory of images
function processDirectory {
  startTime=$(now)
  imgCount=$(getImgCount "$imgPath")
  startSize=$(getImagesSizeTotal "$imgPath")
  echo "Processing $imgCount images..."

  runImageAlphaOnDirectory "$imgPath"

  runJPEGmini "$imgPath"
  waitForJPEGmini

  runImageOptimOnDirectory "$imgPath"
  waitForImageOptim

  endSize=$(getImagesSizeTotal "$imgPath")
  printReport "$startSize" "$endSize"

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
