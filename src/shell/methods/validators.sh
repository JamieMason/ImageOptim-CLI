# (): if an override is not set, get path to this executable
function initCliPath {
  if [ "false" == $cliPath ]; then
    cliPath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
  fi
}

# (): quit if -d, --directory does not resolve
function validateImgPath {
  if [ "directory" == $runMode ] && [ ! -d "$imgPath" ]; then
    error "{{invalidDirectoryMsg}}"
  fi
}

# ($1:appBundleId): eg. "net.pornel.ImageAlpha" -> "ImageAlpha.app" or "NOT_INSTALLED"
function getAppFileNameByBundleId {
  echo `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $1`
}

# ($1:appFileName, $2:appBundleId): -> "true" or "false"
function appIsInstalled {
  if [ $1 == $(getAppFileNameByBundleId $2) ]; then
    echo "true"
  else
    echo "false"
  fi
}

# (): -> "true" or "false"
function imageOptimIsInstalled {
  echo $(appIsInstalled $imageOptimAppFileName $imageOptimAppBundleId)
}

# (): -> "true" or "false"
function imageAlphaIsInstalled {
  echo $(appIsInstalled $imageAlphaAppFileName $imageAlphaAppBundleId)
}

# (): -> "true" or "false"
function jpegMiniIsInstalled {
  if [ "true" == $(appIsInstalled $jpegMiniAppFileName $jpegMiniAppBundleId) ] || [ "true" == $(appIsInstalled $jpegMiniAppFileName $jpegMiniAppRetailBundleId) ]; then
    echo "true"
  else
    echo "false"
  fi
}

# (): -> "true" or "false"
function guiScriptIsEnabled {
  echo `osascript "$cliPath/imageOptimAppleScriptLib" has_gui_script`
}

# ($1:appShouldBeRun, $2:appIsInstalled, $3:isNotInstalledMsg):
function errorIfNotInstalled {
  if [ "true" == $1 ] && [ "false" == $2 ]; then
    error "$3"
  fi
}

# (): quit if ImageOptim should be run but is not installed
function validateImageOptim {
  errorIfNotInstalled $useImageOptim $(imageOptimIsInstalled) "{{imageOptimNotInstalledMsg}}"
}

# (): quit if ImageAlpha should be run but is not installed
function validateImageAlpha {
  errorIfNotInstalled $useImageAlpha $(imageAlphaIsInstalled) "{{imageAlphaNotInstalledMsg}}"
}

# (): quit if ImageAlpha should be run but is not installed or cannot run
function validateJpegMini {

  # if we're not running JPEGmini then it's all good
  if [ "false" == $useJPEGmini ]; then
    return 0
  fi

  # if we are and it's not installed
  if [ "false" == $(jpegMiniIsInstalled) ]; then
    error "{{jpegMiniNotInstalledMsg}}"
  fi

  # if we are, it's installed but GUIScript is not available
  if [ "false" == $(guiScriptIsEnabled) ]; then
    error "{{guiScriptIsDisabledMsg}}"
  fi

}
