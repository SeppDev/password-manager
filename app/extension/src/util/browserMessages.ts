export class BrowserMessages<T = void> {
  readonly channel: String;
  readonly isContentScript: boolean;

  constructor(channel: String, isContentScript?: boolean) {
    this.channel = channel;
    this.isContentScript = isContentScript === true;
  }

  async sendMessage(value: T, tabId?: number) {
    try {
      if (tabId) {
        browser.tabs.sendMessage(tabId, {
          channel: this.channel,
          value,
        });
        return;
      }

      if (this.isContentScript) {
        const tabs = await browser.tabs.query({});
        tabs.map(async (tab) => {
          if (!tab.id) return;
          browser.tabs.sendMessage(tab.id, {
            channel: this.channel,
            value,
          });
        });
      } else {
        await browser.runtime.sendMessage({ channel: this.channel, value });
      }
    } catch {
      console.warn(`Failed to send ${value} to ${this.channel}`);
      // console.warn(`Receiving end for ${this.channel} does not exist`);
    }
  }

  async onMessage(callback: (value: T) => Promise<void> | void) {
    browser.runtime.onMessage.addListener(async (message: any) => {
      if (message.channel !== this.channel) return;
      await callback(message.value);
    });
  }
}
