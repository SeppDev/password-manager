import { sleep } from "../util/sleep";

function getCookie(cname: String) {
  let name = cname + "=";
  let decodedCookie = decodeURIComponent(document.cookie);
  let ca = decodedCookie.split(";");
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == " ") {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}

const style = document.createElement("style");
style.textContent = `.NotDetected {display: None}`;
document.head.appendChild(style);

async function main() {
  while (true) {
    await sleep(400);
    let token = getCookie("token");

    if (token.length < 1) continue;

    browser.runtime.sendMessage({ type: "session-token", token });
  }
}

main();
