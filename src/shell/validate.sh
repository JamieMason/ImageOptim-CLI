# quit if imgPath is not a directory
if [ ! -d "$imgPath" ]; then
  error "Could not find directory $imgPath"
fi

# quit if ImageOptim is not installed
if [ "ImageOptim.app" != `osascript "$cliPath/imageOptimAppleScriptLib" 'hasAppInstalled' 'net.pornel.ImageOptim'` ]; then
  error "ImageOptim.app is not installed (http://imageoptim.com)"
fi

if [ "true" == $runImageAlpha ]; then
  if [ "ImageAlpha.app" != `osascript "$cliPath/imageOptimAppleScriptLib" hasAppInstalled net.pornel.ImageAlpha` ]; then
    error "ImageAlpha.app is not installed (http://pngmini.com)"
  fi
fi

if [ "true" == $runJPEGmini ]; then
  if [ "JPEGmini.app" != `osascript "$cliPath/imageOptimAppleScriptLib" hasAppInstalled com.icvt.JPEGmini` ]; then
    error "JPEGmini.app is not installed (https://itunes.apple.com/us/app/jpegmini/id498944723)"
  fi
  if [ "1" != `osascript "$cliPath/JPEGmini" check` ]; then
    `osascript "$cliPath/imageOptimAppleScriptLib" hasGuiScript`
    error "To automate JPEGmini we need to enable GUI Scripting, check 'Enable access for assistive devices' under Accessibility in System Preferences, then run ImageOptim-CLI again"
  fi
fi
