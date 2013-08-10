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

# (): eg. Checks prefs plist for the paid key. Returns true or false
function haveTheyPaidForJPEGMini {
  echo `osascript "$cliPath/imageOptimAppleScriptLib" has_paid_for_jpegmini`
}

# ($1:appFileName, $2:appBundleId): -> "true" or "false"
function appIsInstalled {
  if [ $1 == $(getAppFileNameByBundleId $2) ]; then
    echo "true"
  else
    echo "false"
  fi
}

# ($1:appFName, $2:isJPEGMini): -> "true" or "false" or "lite"
# this uses the app name, not bundleID, so it caters for
# full app, lite app and retail app
# probably don't need the isJPEGMini flag unless getAppFileNameByBundleId is deprecated
function appIsInstalledNOAS {
  
  isJPEGMini="${2:-false}"
  appPath=$(system_profiler SPApplicationsDataType | grep Applications | grep "$1" | grep Location |  cut -d ":" -f2- | xargs)

  if [ ${#appPath} -eq 0 ]; then
    echo "false"
  else
    if [ "true" == $isJPEGMini ]; then
      jpegMiniAppFileName=$(basename "$appPath")

      if [[ "$jpegMiniAppFileName" =~ .*(Lite)\.app$ ]]; then
        echo "lite"
      fi
    else
      echo "true"
    fi
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

  retVal=$(appIsInstalledNOAS $jpegMiniAppName "true")

  if [[ "true" == $retVal ]]; then      
    echo "true"
  elif [[ "lite" == $retVal ]]; then
    echo "lite"
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

# (): quit if JPEGMini should be run but is not installed or cannot run
function validateJpegMini {

  # if we're not running JPEGmini then it's all good
  if [ "false" == $useJPEGmini ]; then
    return 0
  fi

  retVal=$(jpegMiniIsInstalled)
  # if we are and it's not installed
  if [[ "false" == $retVal ]]; then
    error "{{jpegMiniNotInstalledMsg}}"
  fi

  if [[ "lite" == $retVal ]]; then
      jpegMiniAppName="JPEGmini Lite"

      # not sure if you want to do this?
      paid=$(haveTheyPaidForJPEGMini)
      if [[ "false" == $paid ]]; then
        error "{{jpegMiniNotPurchasedMsg}}"
      fi
  fi
  # if we are, it's installed but GUIScript is not available
  if [ "false" == $(guiScriptIsEnabled) ]; then
    error "{{guiScriptIsDisabledMsg}}"
  fi
}

