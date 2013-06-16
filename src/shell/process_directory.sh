# (): run applications against a directory of files
function processDirectory {
  startTime=$(now)
  echo "Processing $(getImgCount) images..."
  runImageAlphaOnDirectory
  runImageOptimOnDirectory
  runJPEGminiOnDirectory
  endTime=$(now)
  success "Finished in $(getTimeSpent) seconds" | xargs
}
