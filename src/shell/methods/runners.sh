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
  waitForImageAlpha

  runImageOptimOnDirectory "$imgPath"
  waitForImageOptim

  runJPEGmini "$imgPath"
  waitForJPEGmini

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

  for file in "${pipedFiles[@]}"; do
    if [ "" != "`echo "$file" | grep -E '{{imageAlphaFileTypes}}'`" ]; then
      echo "{{imageAlphaAppName}}: $file"
      runImageAlphaOnImage "$file"
    fi
  done

  waitForImageAlpha

  for file in "${pipedFiles[@]}"; do
    if [ "" != "`echo "$file" | grep -E '{{imageOptimFileTypes}}'`" ]; then
      echo "{{imageOptimAppName}}: $file"
      runImageOptimOnImage "$file"
    fi
  done

  waitForImageOptim

  for file in "${pipedFiles[@]}"; do
    if [ "" != "`echo "$file" | grep -E '{{jpegMiniFileTypes}}'`" ]; then
      echo "{{jpegMiniAppName}}: $file"
      runJPEGmini "$file"
    fi
  done

  waitForJPEGmini
}
