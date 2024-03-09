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

    1. Enter extension id

        ```
        TODO
        $ edit app/manifest.json and change extension id with the one from step 1.3.
        ```

    2. Build the extension

        ```
        $ cargo build --release
        ```

    3. Note the binary path (somewhere in app/target/release/app(.exe)

    4. Enter binary path

        ```
        TODO
        $ edit app/manifest.json and change path to the one from step 2.3.
        ```

    5. Register native app: https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging#native-messaging-host-location

        On Windows:

        ```
        TODO
        $ edit app/install.bat and change manifest path
        $ ./app/install.bat
        ```

        Linux & macOS:

        Copy manifest to correct location
