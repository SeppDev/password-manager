import { mount } from "svelte";
import InputButton from "../components/InputButton.svelte";

mount(InputButton, {target: document.body});

type InpuType = "username" | "email" | "password";
const id = "PASSWORD_MANAGER";

const config = {
  buttonScale: 0.6,
  maxButtonSize: 40,

  inputCheckInterval: 500,
  inputQueryInterval: 1000,
};

const found = document.getElementById(id);
if (found) {
  found.remove();
}

const shadowHost = document.createElement("div");
shadowHost.id = id;
shadowHost.style.position = "absolute";
// shadowHost.style.display = "block";

const shadowRoot = shadowHost.attachShadow({ mode: "open" });

const shadowDiv = document.createElement("div");
shadowDiv.style.height = "0px";
shadowDiv.style.width = "0px";
shadowRoot.appendChild(shadowDiv);

const button = document.createElement("button");
button.style.position = "absolute";
button.style.display = "flex";
button.style.justifyContent = "center";
button.style.alignItems = "center";
button.style.borderRadius = "1000px";
button.style.background =
  "linear-gradient(45deg, rgb(24, 24, 24), rgb(39, 0, 102))";
button.style.aspectRatio = "1";
button.style.border = "none";
button.style.cursor = "pointer";
button.style.zIndex = "1000";
button.tabIndex = -1;
shadowDiv.appendChild(button);

const icon = document.createElement("img");
icon.src = browser.runtime.getURL("src/assets/lock.svg");
icon.style.aspectRatio = "1";
icon.style.height = "60%";
button.appendChild(icon);

button.onclick = () => {
  alert("clicked");
};

let activeInput: HTMLElement | null = null;

function updateActiveInput() {
  if (!activeInput) return;

  if (shadowHost.parentElement != activeInput.parentElement) {
    activeInput.parentElement?.insertBefore(shadowHost, activeInput);
  }

  const inputBounds = activeInput.getBoundingClientRect();
  const shadowBounds = shadowHost.getBoundingClientRect();

  const buttonSize = Math.min(
    inputBounds.height * config.buttonScale,
    config.maxButtonSize
  );

  const desiredPosition = {
    x:
      inputBounds.x +
      inputBounds.width -
      buttonSize -
      buttonSize * (1 - config.buttonScale),
    y: inputBounds.y + inputBounds.height / 2 - buttonSize / 2,
  };

  const offset = {
    x: desiredPosition.x - shadowBounds.x,
    y: desiredPosition.y - shadowBounds.y,
  };

  button.style.height = `${buttonSize}px`;
  button.style.left = `${offset.x}px`;
  button.style.top = `${offset.y}px`;
}

const knownElements: Array<HTMLElement> = [];

function handleInput(input: HTMLElement) {
  if (input.tagName !== "INPUT") return;
  if (knownElements.includes(input)) return;
  knownElements.push(input);

  const inputType = input.getAttribute("type");
  if (!(inputType === "text" || inputType === "password")) return;

  if (inputType === "password" && activeInput === null) {
    activeInput = input;
    updateActiveInput();
  }

  input.addEventListener("mouseenter", () => {
    activeInput = input;
    updateActiveInput();
  });
}

document.querySelectorAll("input[type='password']").forEach((element) => {
  activeInput = element as HTMLElement;
  updateActiveInput();
});

setInterval(updateActiveInput, config.inputCheckInterval);
setInterval(() => {
  document.querySelectorAll("input").forEach(handleInput);
}, config.inputCheckInterval);
