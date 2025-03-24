import { mount } from "svelte";
import InputButton from "../../src/components/InputButton.svelte";

const id = "PASSWORD_MANAGER";

const config = {
    buttonScale: 0.7,
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
shadowHost.style.zIndex = "100";

const shadowRoot = shadowHost.attachShadow({ mode: "open" });

const offsetDiv = document.createElement("div");
offsetDiv.style.position = "absolute";
offsetDiv.style.zIndex = "1000";
offsetDiv.style.width = "100%";
offsetDiv.style.height = "100%";
offsetDiv.style.zIndex = "10000";

shadowRoot.appendChild(offsetDiv);

mount(InputButton, { target: offsetDiv });

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

    offsetDiv.style.height = `${buttonSize}px`;
    offsetDiv.style.width = `${buttonSize}px`;

    offsetDiv.style.left = `${offset.x}px`;
    offsetDiv.style.top = `${offset.y}px`;
}

const knownElements: Array<HTMLElement> = [];

function handleInput(input: HTMLElement) {
    if (knownElements.includes(input)) return;
    knownElements.push(input);


    const inputType = input.getAttribute("type");
    if (!activeInput) {
        activeInput = input;
        updateActiveInput();
    }

    input.addEventListener("mouseenter", () => {
        activeInput = input;
        updateActiveInput();
    });
}

function handleElements(elements: NodeListOf<HTMLElement>) {
    elements.forEach((element) => handleInput(element));
}

setInterval(updateActiveInput, config.inputCheckInterval);

handleElements(document.querySelectorAll("input[type='password']"));
setInterval(() => {
    handleElements(document.querySelectorAll("input[type='text']"));
    handleElements(document.querySelectorAll("input[type='email']"));
    handleElements(document.querySelectorAll("input[type='password']"));
}, config.inputCheckInterval);