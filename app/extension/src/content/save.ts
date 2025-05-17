import { mount } from "svelte";
import Prompt from "./Prompt.svelte";
import tailwindStyles from "../tailwind.css?inline";

const id = "PASSWORD_MANAGER_SAVE";

const config = {
  padding: 20,
  updateLocationInterval: 100,
};

const found = document.getElementById(id);
if (found) {
  found.remove();
}

const shadowHost = document.createElement("span");
shadowHost.id = id;
shadowHost.style.position = "absolute";
shadowHost.style.zIndex = "10000";

const shadowRoot = shadowHost.attachShadow({ mode: "open" });
const style = document.createElement("style");
style.textContent = tailwindStyles;
shadowRoot.appendChild(style);

const offsetDiv = document.createElement("span");
offsetDiv.style.position = "absolute";
offsetDiv.style.zIndex = "100000";

shadowRoot.appendChild(offsetDiv);

mount(Prompt, { target: offsetDiv });
document.body.appendChild(shadowHost);

function updateLocation() {
  const documentBounds = document.body.getBoundingClientRect();
  const shadowBounds = shadowHost.getBoundingClientRect();

  const desiredPosition = {
    x:
      documentBounds.right -
      offsetDiv.getBoundingClientRect().width -
      config.padding,
    y: config.padding,
  };

  const offset = {
    x: desiredPosition.x - shadowBounds.x,
    y: desiredPosition.y - shadowBounds.y,
  };

  offsetDiv.style.left = `${offset.x}px`;
  offsetDiv.style.top = `${offset.y}px`;
}
updateLocation();
setInterval(updateLocation, config.updateLocationInterval);
