# begin
startTime=$(now)

echo "Processing $(getImgCount) images..."

if [ "true" == $runImageAlpha ]; then
  $(populateImageAlphaQueue)
  $(waitFor $imageAlphaAppName)
  if [ "true" == $quitOnComplete ]; then
    osascript -e "tell application \"$imageAlphaAppName\" to quit"
  fi
fi

if [ "true" == $runImageOptim ]; then
  $(populateImageOptimQueue)
  $(waitFor $imageOptimAppName)
  if [ "true" == $quitOnComplete ]; then
    osascript -e "tell application \"$imageOptimAppName\" to quit"
  fi
fi

if [ "true" == $runJPEGmini ]; then
  `osascript "$cliPath/imageOptimAppleScriptLib" run_jpegmini $imgPath $jpegMiniAppName` > /dev/null 2>&1
  `osascript "$cliPath/imageOptimAppleScriptLib" wait_for $jpegMiniAppName` > /dev/null 2>&1
  if [ "true" == $quitOnComplete ]; then
    osascript -e "tell application \"$jpegMiniAppName\" to quit"
  fi
fi

endTime=$(now)

success "Finished in $(getTimeSpent) seconds" | xargs
