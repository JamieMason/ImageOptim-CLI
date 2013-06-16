if [ "directory" == $runMode ]; then
  processDirectory
elif [ "file" == $runMode ]; then
  processImage
fi
