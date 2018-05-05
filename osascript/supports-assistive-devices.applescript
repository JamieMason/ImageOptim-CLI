#!/bin/osascript

on supportsAssistiveDevices()
  try
    tell application "System Events" to get UI elements enabled
  on error
    return "ERROR_GUISCRIPT_UNREADABLE"
  end try
end supportsAssistiveDevices

on run argv
  return supportsAssistiveDevices()
end run
