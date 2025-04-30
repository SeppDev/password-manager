export class BrowserMessages<T = void> {
  readonly channel: String;

  constructor(channel: String) {
    this.channel = channel;
  }

  async sendMessage(value: T) {
    await browser.runtime.sendMessage({ channel: this.channel, value });
  }

  async onMessage(callback: (value: T) => Promise<void> | void) {
    browser.runtime.onMessage.addListener(async (message: any) => {
      if (message.channel !== this.channel) return;
      if (!message.value) return;

      await callback(message.value);
    });
  }
}
