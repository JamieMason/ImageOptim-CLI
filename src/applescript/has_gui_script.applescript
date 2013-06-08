-- Return true is GUIScript is enabled, or false
on has_gui_script()
  tell application "System Events"
    set isEnabled to UI elements enabled
  end tell
  return isEnabled as boolean
end has_gui_script
