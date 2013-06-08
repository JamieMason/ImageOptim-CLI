-- Using the Application's bundle id, get the .app file name if installed
-- eg. "net.pornel.ImageOptim" -> "ImageOptim.app"
on has_app_installed(appFileId)
  try
    tell application "Finder"
      return name of application file id appFileId
    end tell
  on error err_msg number err_num
    return "NOT_INSTALLED"
  end try
end has_app_installed
