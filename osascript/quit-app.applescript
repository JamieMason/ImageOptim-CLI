#!/bin/osascript

on quitApp(appName)
  tell application appName to quit
end quitApp

on run argv
  set appName to item 1 of argv
  return quitApp(appName)
end run
