const appWindows = new Set();
const appTabs = new Set();
chrome.windows.onCreated.addListener(
    (window) => void appWindows.add(window.id),
    { windowTypes: ["app"] },
);
chrome.tabs.onCreated.addListener((tab) => {
    if (appWindows.has(tab.windowId)) {
        appTabs.add(tab.id);
    }
});
chrome.tabs.onRemoved.addListener((tabId, removeInfo) => {
    appTabs.delete(tabId);
    if (removeInfo.isWindowClosing) {
        appWindows.delete(removeInfo.windowId);
    }
});
chrome.tabs.query({ windowType: "app" }).then((tabs) => {
    for (const tab of tabs) {
        appWindows.add(tab.windowId);
        appTabs.add(tab.id);
    }
});

chrome.webNavigation.onCreatedNavigationTarget.addListener((details) => {
    if (appTabs.has(details.sourceTabId)) {
        console.log(
            "External URL opened from app --> open in default browser",
            details.url,
        );
        chrome.tabs.remove(details.tabId);
        chrome.runtime.sendNativeMessage(
            "me.kuehle.default_linker",
            { url: details.url },
            (response) => {
                console.log(response);
            },
        );
    }
});
