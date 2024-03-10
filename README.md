# DefaultLinker

Opens external links from installed Chrome apps in the default browser.

## Installation

1. Install extension

    1. Package extension

        ```
        $ cd ext/ && zip ext.zip manifest.json icon.png sw.js
        ```

    2. Open Chrome `chrome://extensions` and load the zip file.

    3. Note the extension id.

2. Install native app

    1. Build the extension

        ```
        $ cd app/ && cargo build --release
        ```

    2. Register native app:

        ```
        $ default_linker[.exe] --install <extension_id>
        ```

        More details: https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging#native-messaging-host-location
