-- Checks prefs plist for the paid key. Returns true or false
on has_paid_for_jpegmini()
  try
    
    set _username to system attribute "USER"
    if _username is "" then
      set _username to do shell script "whoami"
    end if
    
    if _username is "root" then
      set _username to do shell script "who | grep console | awk '{print $1}'"
    end if
    
    set _fileName to "/Users/" & _username & "/Library/Containers/com.icvt.JPEGminiLite/Data/Library/Preferences/com.icvt.JPEGminiLite.plist"
    set theTmpFile to (path to temporary items as string) & "test.plist"
    
    do shell script "cp " & _fileName & " " & POSIX path of theTmpFile
    do shell script "plutil -convert xml1 " & POSIX path of theTmpFile
    
    set paid to do shell script "cat " & POSIX path of theTmpFile & "| grep -A1 did-upgrade-to-full | grep -v key | sed -E 's/<//g' | sed -E 's/\\/>//g' | xargs"
    
    if (paid is equal to "") then
      set paid to "false"
    end if
    
    return paid
  on error
    return "false"
  end try
end has_paid_for_jpegmini
