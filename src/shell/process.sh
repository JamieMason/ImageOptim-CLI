if [ "directory" == $runMode ]; then
  processDirectory
elif [ "stdin" == $runMode ]; then
  processFiles
fi
