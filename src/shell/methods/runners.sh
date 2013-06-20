# (): Get a timestamp for the current time
function now {
  date +"%s"
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
