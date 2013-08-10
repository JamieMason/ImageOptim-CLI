# ($1:appName): Get the number of processes in use by an Application
function countProcesses {
  printf $(ps -aef | grep  "[${1:0:1}]${1:1}.app" | wc -l)
}

# ($1:appName): Sleep until app is done optimising images
function waitForApp {
  # wait for App to spawn a few processes
  sleep 2
  # wait until those processes have completed
  while [[ $(countProcesses $1) > "1" ]]; do
    sleep $isBusyIntervalLength
  done
}

# ($1:appName):
function waitForPornelApp {
  waitForApp $1
  if [ "true" == $quitOnComplete ]; then
    osascript -e "tell application \"$1\" to quit"
  fi
}

# ():
function waitForImageOptim {
  if [ "true" == $useImageOptim ]; then
    waitForPornelApp $imageOptimAppName
  fi
}

# ():
function waitForJPEGmini {
  if [ "true" == $useJPEGmini ]; then
    sleep 1
    `osascript "$cliPath/imageOptimAppleScriptLib" wait_for "$jpegMiniAppName"` > /dev/null 2>&1
    if [ "true" == $quitOnComplete ]; then
      osascript -e "tell application \"$jpegMiniAppName\" to quit"
    fi
  fi
}
