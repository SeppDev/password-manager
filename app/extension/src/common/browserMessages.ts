export class BrowserMessages<T = void, R = void> {
    readonly channel: String;

    constructor(channel: String) {
        this.channel = channel;
    }

    async sendMessage(value: T): Promise<R> {
        return await browser.runtime.sendMessage({ channel: this.channel, value });
    }

    async onMessage(callback: (value: T) => Promise<R>) {
        browser.runtime.onMessage.addListener(async (message: any) => {
            if (message.channel !== this.channel) return;
            if (!message.value) return;
            
            return await callback(message.content)
        })
    }
}
