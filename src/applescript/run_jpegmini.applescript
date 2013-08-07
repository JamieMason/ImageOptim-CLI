-- Feed a folder of images into JPEGmini
on run_jpegmini(imgPath, appName)
  try
    tell application appName

      -- start the app
      activate

      -- let it boot up
      delay 3

      -- ensure it still has focus
      activate

    end tell

    tell application "System Events"
      tell process appName

        -- Navigate to the File > Open menu
        click menu item "Open…" of menu 1 of menu bar item "File" of menu bar 1

        -- let Finder spawn the browse dialog
        delay 1

        -- command+shift+g in Finder lets us enter file paths direct
        --keystroke "g" using {command down, shift down}
        -- shift-cmd-g doesn't work on my machine, or maybe just with JPEGmini Lite
        keystroke "/"
        -- window is called JPEGmini even in the Lite app
        set value of text field 1 of sheet 1 of sheet 1 of window "JPEGmini" to imgPath

        delay 1

        -- and navigate to it
        keystroke return

        -- let Finder resolve the path
        delay 1

        -- start JPEGmini off optimising the folder
        click button "Open" of sheet 1 of window "JPEGmini" -- window is called JPEGmini even in the Lite app

        -- report success
        return true

      end tell
    end tell

    -- report success
    return true

    on error error_message

      -- report failure
      return false

  end try
end run_jpegmini