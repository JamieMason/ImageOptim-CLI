# begin
startTime=$(now)

echo "Processing $(getImgCount) images..."

if [ "true" == $runImageAlpha ]; then
  $(populateImageAlphaQueue)
  $(waitFor "ImageAlpha")
  osascript -e 'tell application "ImageAlpha" to quit'
fi

$(populateImageOptimQueue)
$(waitFor "ImageOptim")

if [ "true" == $runJPEGmini ]; then
  `osascript "$cliPath/imageOptimAppleScriptLib" run_jpegmini $imgPath $jpegMiniAppName` > /dev/null 2>&1
  `osascript "$cliPath/imageOptimAppleScriptLib" wait_for $jpegMiniAppName` > /dev/null 2>&1
fi

endTime=$(now)

if [ "true" == $quitOnComplete ]; then
  osascript -e 'tell application "ImageOptim" to quit'
fi

success "Finished in $(getTimeSpent) seconds" | xargs
