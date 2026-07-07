#!/bin/osascript

(* UI conditions are polled every 0.2s up to a timeout, so the automation
   moves on as soon as JPEGmini is ready instead of sleeping fixed amounts.
   On timeout each step proceeds anyway, as the fixed delays used to. *)

on waitUntilFrontmost(appName, timeoutSeconds)
  set waited to 0
  repeat
    tell application "System Events"
      if (exists process appName) then
        if frontmost of process appName then return true
      end if
    end tell
    if waited >= timeoutSeconds then return false
    tell application appName to activate
    delay 0.2
    set waited to waited + 0.2
  end repeat
end waitUntilFrontmost

on openDialogExists(appName)
  tell application "System Events"
    tell process appName
      return (exists sheet 1 of window 1)
    end tell
  end tell
end openDialogExists

on waitUntilOpenDialogExists(appName, timeoutSeconds)
  set waited to 0
  repeat
    if my openDialogExists(appName) then return true
    if waited >= timeoutSeconds then return false
    delay 0.2
    set waited to waited + 0.2
  end repeat
end waitUntilOpenDialogExists

on goToInputExists(appName)
  tell application "System Events"
    tell process appName
      if (exists text field 1 of sheet 1 of sheet 1 of window 1) then return true
      if (exists combo box 1 of sheet 1 of sheet 1 of window 1) then return true
      if (exists combo box 1 of sheet 1 of window 1) then return true
      if (exists text field 1 of sheet 1 of window 1) then return true
      return false
    end tell
  end tell
end goToInputExists

on waitUntilGoToInput(appName, timeoutSeconds, shouldExist)
  set waited to 0
  repeat
    if my goToInputExists(appName) is shouldExist then return true
    if waited >= timeoutSeconds then return false
    delay 0.2
    set waited to waited + 0.2
  end repeat
end waitUntilGoToInput

on runJPEGmini(jpegDirectory, jpegMiniAppName)
  (* OPEN AND FOCUS JPEGMINI *)
  tell application jpegMiniAppName to activate
  my waitUntilFrontmost(jpegMiniAppName, 10)
  (* SPAWN THE FILE > OPEN MENU *)
  tell application "System Events"
    keystroke "o" using {command down}
  end tell
  my waitUntilOpenDialogExists(jpegMiniAppName, 10)
  (* SPAWN THE GO TO FOLDER SHEET *)
  tell application "System Events"
    keystroke "g" using {command down, shift down}
  end tell
  my waitUntilGoToInput(jpegMiniAppName, 10, true)
  (* NAVIGATE TO OUR FOLDER OF IMAGES *)
  tell application "System Events"
    tell process jpegMiniAppName
      (* < SIERRA, FILE PATH SELECTOR IS TEXT INPUT *)
      if text field 1 of sheet 1 of sheet 1 of window 1 exists then
        set value of text field 1 of sheet 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of text field 1 of sheet 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 0.2
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
            delay 0.2
          else
            exit repeat
          end if
        end repeat
      end if
      (* HIGH SIERRA *)
      if combo box 1 of sheet 1 of window 1 exists then
        set value of combo box 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of combo box 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 0.2
          else
            exit repeat
          end if
        end repeat
      end if
      (* >= MONTEREY *)
      if text field 1 of sheet 1 of window 1 exists then
        set value of text field 1 of sheet 1 of window 1 to jpegDirectory
        repeat
          if (value of text field 1 of sheet 1 of window 1) is not equal to jpegDirectory then
            delay 0.2
          else
            exit repeat
          end if
        end repeat
      end if
      -- give the file browser a beat to resolve the typed path
      delay 0.5
      keystroke return
    end tell
  end tell
  (* CONFIRM THE GO TO FOLDER SHEET CLOSED, THEN START OPENING *)
  my waitUntilGoToInput(jpegMiniAppName, 5, false)
  tell application "System Events"
    keystroke return
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
