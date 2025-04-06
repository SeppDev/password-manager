const storage = {
    session_token: "",
    authenticated: false,
};

export type Storage = typeof storage;

function sendStorage() {
    browser.runtime.sendMessage({
        type: "storage-update",
        storage,
    });
}

browser.runtime.onMessage.addListener((msg) => {
    if (msg.type === "session-token") {
        storage.session_token = msg.token;
    } else if (msg.type === "sync-storage") {
        sendStorage();
    }
});

browser.runtime.onConnect.addListener((port) => {
    if (port.name !== "popup-connection") return;
    sendStorage();
});
