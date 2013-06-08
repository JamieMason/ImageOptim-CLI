-- Open System Preferences on the Accessibility view for a user to enable GUIScript
on open_accessibility_preferences()
  tell application "System Preferences"
    activate
    set current pane to pane id "com.apple.preference.universalaccess"
  end tell
end open_accessibility_preferences
