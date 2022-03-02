let host = "https://boxcat.site";

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

browser.webRequest.onBeforeRequest.addListener(
    (details) => {
        console.debug(details);
        console.log(`Redirecting ${details.url}...`);
        const url = new URL(details.url);
        let redirect;
        redirect = { redirectUrl: redirectReuters(url) };
        if (redirect && redirect.redirectUrl) {
            console.info("Details", details);
        }
        return redirect;
    },
    {
        urls: [
            "*://reuters.com/*",
            "*://www.reuters.com/*"
        ],
    },
    ["blocking"]
);
