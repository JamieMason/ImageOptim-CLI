-- handle input from terminal
on run argv

  -- the first option should always be the command to run
  set command to item 1 of argv

  -- in the case of JPEGmini, we need to know if GUIScript is enabled...
  if command is "has_app_installed" then

    -- we expect the 2nd argument to be the application's bundle id
    return has_app_installed(item 2 of argv)

  -- in the case of JPEGmini
  else if command is "has_gui_script" then

    -- we need to know if GUIScript is enabled
    return has_gui_script()

  -- process a directory using JPEGmini
  else if command is "run_jpegmini" then

    -- we expect the 2nd argument to be the path
    -- we expect the 3rd argument to be the app name
    -- optimise the directory of images
    run_jpegmini(item 2 of argv, item 3 of argv)

  -- wait for an app to finish running
  else if command is "wait_for" then

    -- we expect the 2nd argument to be the app name
    -- keep the shell script waiting until the app has finished
    wait_for(item 2 of argv)

   -- Check prefs plist for the paid key
  else if command is "has_paid_for_jpegmini" then
    return has_paid_for_jpegmini()
  end if
end run
