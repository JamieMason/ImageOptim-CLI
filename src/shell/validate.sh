# handle override
if [ "false" == $cliPath ]; then
  cliPath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
fi

# quit if imgPath is not a directory
if [ ! -d "$imgPath" ]; then
  error "Could not find directory $imgPath"
fi

# quit if ImageOptim is not installed
if [ "$imageOptimAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $imageOptimAppBundleId` ]; then
  error "$imageOptimAppFileName is not installed (http://imageoptim.com)"
fi

if [ "true" == $runImageAlpha ]; then
  if [ "$imageAlphaAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $imageAlphaAppBundleId` ]; then
    error "$imageAlphaAppFileName is not installed (http://pngmini.com)"
  fi
fi

if [ "true" == $runJPEGmini ]; then
  if [ "$jpegMiniAppFileName" != `osascript "$cliPath/imageOptimAppleScriptLib" has_app_installed $jpegMiniAppBundleId` ]; then
    error "$jpegMiniAppFileName is not installed (https://itunes.apple.com/us/app/jpegmini/id498944723)"
  fi
  if [ "1" != `osascript "$cliPath/imageOptimAppleScriptLib" has_gui_script` ]; then
    error "To automate JPEGmini we need to enable GUI Scripting, check 'Enable access for assistive devices' under Accessibility in System Preferences, then run ImageOptim-CLI again"
  fi
fi
