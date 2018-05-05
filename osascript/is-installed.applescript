#!/bin/osascript

on isInstalled(bundleId)
  try
    tell application "Finder" to get application file id bundleId
    return true
  on error
    return false
  end try
end isInstalled

on run argv
  set bundleId to item 1 of argv
  return isInstalled(bundleId)
end run
