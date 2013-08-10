-- Watch app until it's finished working with our folder
on wait_for(processName)
  tell application "System Events"
    repeat

      -- get a process id we can pass to `ps`
      set PID to unix id of process processName

      -- filter active process output to just JPEGmini, then to just the % CPU
      set getCpuPercent to "ps aux | grep " & PID & " | grep -v grep | awk '{print $3}'"

      -- convert the terminal's string output to a number
      set cpuPercent to (do shell script getCpuPercent)
      round cpuPercent rounding toward zero

      -- give the app a little time to work
      delay 2

      -- if the app is idle
      if (cpuPercent) < 1 then

        -- we think it's finished
        exit repeat

      end if

      -- (implied else: by not exiting we repeat again)

    end repeat
  end tell
end wait_for
