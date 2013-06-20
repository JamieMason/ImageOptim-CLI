initCliPath
validateImgPath
validateImageOptim
validateImageAlpha
validateJpegMini

if [ "directory" == $runMode ]; then
  processDirectory
elif [ "stdin" == $runMode ]; then
  processFiles
fi
