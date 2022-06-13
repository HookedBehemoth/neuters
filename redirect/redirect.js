let host = "https://neuters.de";

function redirectReuters(url) {
    const path = url.pathname;
    if (path.startsWith('/article/')
        || path.startsWith('/pf/')
        || path.startsWith('/arc/')
        || path.startsWith('/resizer/')) {
        return null;
    } else if (path.endsWith('/')) {
        return `${host}${url.pathname}`;
    } else {
        return `${host}${url.pathname}/`;
    }
}

// I hate chromium I hate chromium I hate chromium I hate chromium I hate chromium I hate chromium
if (chrome.webRequest) {
    chrome.webRequest.onBeforeRequest.addListener(
        (details) => {
            console.debug(details);
            console.debug(`Redirecting ${details.url}...`);
            const url = new URL(details.url);
            return { redirectUrl: redirectReuters(url) };
        },
        {
            urls: [
                "*://reuters.com/*",
                "*://www.reuters.com/*"
            ],
        },
        ["blocking"]
    );
} else {
    console.debug(`Redirecting ${window.location}...`);
    const url = new URL(window.location);
    window.location = redirectReuters(url);
}
