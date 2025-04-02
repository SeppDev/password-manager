<div
  style="aspect-ratio: 1 / 1; display: flex; align-items: center; justify-content: center; height: 100%; width: 100%"
>
    <span class="loader"></span>

  <style>
    .loader {
      aspect-ratio: 1 / 1;
      height: 100%;
      border: 4px solid rgb(200,200,200);
      border-bottom-color: transparent;
      border-radius: 50%;
      display: inline-block;
      box-sizing: border-box;
      animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
      0% {
        transform: rotate(0deg);
      }
      100% {
        transform: rotate(360deg);
      }
    }
  </style>
</div>
