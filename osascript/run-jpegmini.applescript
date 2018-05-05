#!/bin/osascript

on runJPEGmini(jpegDirectory, jpegMiniAppName)
  (* OPEN AND FOCUS JPEGMINI *)
  tell application jpegMiniAppName
    activate
    delay 3
    activate
  end tell
  (* SPAWN THE FILE > OPEN MENU *)
  tell application "System Events"
    keystroke "o" using {command down}
    delay 3
    keystroke "g" using {command down, shift down}
    delay 2
  end tell
  (* NAVIGATE TO OUR FOLDER OF IMAGES *)
  tell application "System Events"
    tell process jpegMiniAppName
      (* < SIERRA, FILE PATH SELECTOR IS TEXT INPUT *)
      if text field 1 of sheet 1 of sheet 1 of window 1 exists then
        set value of text field 1 of sheet 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of text field 1 of sheet 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 1
          else
            exit repeat
          end if
        end repeat
      end if
      (* = SIERRA, FILE PATH SELECTOR IS COMBO BOX *)
      if combo box 1 of sheet 1 of sheet 1 of window 1 exists then
        set value of combo box 1 of sheet 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of combo box 1 of sheet 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 1
          else
            exit repeat
          end if
        end repeat
      end if
      (* >= HIGH SIERRA *)
      if combo box 1 of sheet 1 of window 1 exists then
        set value of combo box 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of combo box 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 1
          else
            exit repeat
          end if
        end repeat
      end if
      -- give Finder time to resolve the path
      delay 2
      keystroke return
      delay 2
      keystroke return
      -- start optimising (>= Yosemite)
      -- click button "Go" of sheet 1 of window 1
      -- start optimising (<= Mavericks)
      -- click button "Open" of sheet 1 of window 1
    end tell
  end tell
  (* WAIT FOR JPEGMINI TO FINISH RUNNING *)
  tell application "System Events"
    set timesIdle to 0
    repeat
      -- get all process information | filtered to JPEGmini
      set getRawProcess to "ps aux | grep '/Applications/" & jpegMiniAppName & "'"
      -- filter out JPEGmini grep | get column 3 of output (% CPU)
      set filterRawProcess to "grep -v grep | awk '{print $3}'"
      -- store above pipe chain in a variable
      set getRawCpu to "RAWCPU=$(" & getRawProcess & " | " & filterRawProcess & ")"
      -- round that variable to a whole number
      set outputRoundedCpu to "$(printf \"%.0f\" $(echo \"scale=2;$RAWCPU\" | bc))"
      -- join the two commands and echo it out to applescript
      set getCpuPercent to getRawCpu & " && echo " & outputRoundedCpu
      -- get raw terminal string output
      set cpuPercent to (do shell script getCpuPercent) as number
      -- give the app a little time to work
      delay 0.5
      -- if the app is idle
      if (cpuPercent) < 1 then
        -- increment number of times we've found the app consecutively idle
        set timesIdle to timesIdle + 1
        -- if it's been idle for long enough we can exit
        if (timesIdle) > 5 then
          exit repeat
        end if
      end if
      -- (implied else: by not exiting we repeat again)
    end repeat
    delay 0.5
  end tell
end runJPEGmini

on run argv
  set jpegDirectory to item 1 of argv
  set jpegMiniAppName to item 2 of argv
  return runJPEGmini(jpegDirectory, jpegMiniAppName)
end run
