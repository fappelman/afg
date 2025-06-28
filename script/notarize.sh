#!/usr/bin/env bash

# Setup
input_bin=$1

team_id=${CERT_ID}
identity="Developer ID Application: Fred Appelman (${team_id})"

# To set up notarization:
#   1. Get your Team ID: https://developer.apple.com/account
#   2. Create an app-specific password: https://appleid.apple.com/account/manage/section/security
#   3. Create the Keychain Profile: /usr/bin/xcrun notarytool store-credentials --apple-id 'EMAIL' --team-id 'TEAM ID'
keychain_profile='Notarization Apple' # The name you set for the Keychain Profile

# Codesign (your specific flags may differ)
 /usr/bin/codesign --timestamp --force --options runtime --sign "${identity}" "${input_bin}"

# Zip, as raw binaries cannot be notarised
/usr/bin/ditto -ck "${input_bin}" "$2"

# Notarize
/usr/bin/xcrun notarytool submit "$2" --keychain-profile "${keychain_profile}" --wait
