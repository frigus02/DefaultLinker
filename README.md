# DefaultLinker

Opens external links from installed Chrome apps in the default browser.

## Installation

Either download native app and extension from releases page or [build it yourself](#build). Then:

1. Open Chrome `chrome://extensions` and drag and drop the extension zip file to install it. Note the generated extension id.

1. Register native app. Executing it with `--install` writes a manifest file to the correct location. On Windows it prints an additional command you need to run to register the manifest file in the registry.

    ```
    $ default_linker[.exe] --install <extension_id>
    ```

    More details: https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging#native-messaging-host-location

## Build

### Native application

```
$ cd app/ && cargo build --release
```

### Extension

```
$ cd ext/ && zip ext.zip manifest.json icon.png sw.js
```
