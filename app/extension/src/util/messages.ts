async function sendMessage<T>(channel: String, content: T) {
    await browser.runtime.sendMessage({ channel, content });
}

async function onMessage<T>(type: String, callback: (content: T) => void) {}

export class BrowserMessages<T, R = void> {
    readonly channel: String;

    constructor(channel: String) {
        this.channel = channel;
    }

    async sendMessage(content: T): Promise<R> {
        return await browser.runtime.sendMessage({ channel: this.channel, content });
    }
    sendMessageSync(content: T): Promise<R> {
        return browser.runtime.sendMessage({ channel: this.channel, content });
    }

    async onMessage(callback: (content: T) => Promise<R>) {
        browser.runtime.onMessage.addListener(async (message: any) => {
            if (message.channel !== this.channel) return;
            if (!message.content) return;
            
            return await callback(message.content)
        })
    }
}
