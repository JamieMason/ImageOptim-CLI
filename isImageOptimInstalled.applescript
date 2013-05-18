try
  tell application "Finder"
    return name of application file id "net.pornel.ImageOptim"
  end tell
on error err_msg number err_num
  return null
end try
