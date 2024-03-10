# Open app links in default browser

Opens external links from installed Chrome apps in the default browser.

## Installation

1. Install extension

    1. Package extension

        ```
        TODO
        $ zip ext/...
        ```

    2. Open Chrome `chrome://extensions` and load the zip file.

    3. Note the extension id.

2. Install native app

    1. Build the extension

        ```
        $ cargo build --release
        ```

    2. Register native app:

        ```
        $ open-app-links-in-default-browser[.exe] --install <extension_id>
        ```

        More details: https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging#native-messaging-host-location
