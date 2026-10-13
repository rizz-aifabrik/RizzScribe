#!/bin/bash
# Uninstalls RizzScribe: first removes its macOS permissions (Microphone,
# Accessibility, Input Monitoring, ...) and saved data, then deletes the app.
#
#   ./rizzscribe-cleanup.sh
ID="com.aifabrik.rizzdictation"
APP="/Applications/RizzScribe.app"

# 1. Quit the app so nothing is holding files open.
pkill -x handy 2>/dev/null
sleep 1

# 2. Remove permissions and data while the app still exists.
tccutil reset All "$ID" >/dev/null 2>&1
rm -rf "$HOME/Library/Application Support/$ID" \
       "$HOME/Library/Caches/$ID" \
       "$HOME/Library/Preferences/$ID.plist" \
       "$HOME/Library/WebKit/$ID" \
       "$HOME/Library/Logs/$ID" \
       "$HOME/Library/Saved Application State/$ID.savedState"

# 3. Delete the app last.
rm -rf "$APP"
echo "RizzScribe permissions, data and app removed."
