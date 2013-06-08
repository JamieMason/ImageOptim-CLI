-- handle input from terminal
on run argv

  -- the first option should always be the command to run
  set command to item 1 of argv

  -- in the case of JPEGmini, we need to know if GUIScript is enabled...
  if command is "has_app_installed" then

    -- we expect the 2nd argument to be the application's bundle id
    set bundleId to item 2 of argv

    return has_app_installed(bundleId)

  -- in the case of JPEGmini, we need to know if GUIScript is enabled...
  else if command is "has_gui_script" then
    return has_gui_script()

  -- ...and if it isn't, help the user find where to go enable it
  else if command is "open_accessibility_preferences" then
    open_accessibility_preferences()

  -- process a directory using JPEGmini
  else if command is "run_jpegmini" then

    -- we expect the 2nd argument to be the path
    set imgPath to item 2 of argv

    -- keep the shell script waiting until JPEGmini has finished
    run_jpegmini(imgPath)
    wait_for("JPEGmini")

  end if
end run
