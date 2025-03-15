<script lang="ts">
  import Loader from "./Loader.svelte";
  import LockIcon from "../assets/LockIcon.svelte";
  import AccountsList from "../menu/AccountsList.svelte";
  import "../tailwind.css";

  type Page = "loading" | "notsigned" | "home";

  let visible = false;
  let page: Page = "loading";

  const accounts = ["John", "Apple", "Strawberry"];

  function clicked(button: HTMLElement) {
    visible = !visible;
  }

  function selected(name: String) {
    visible = false;
  }

  setTimeout(() => {
    page = "home";
    visible = true;
  }, 500);
</script>

<div class="w-full h-full">
  <style>
    @import "tailwindcss";
  </style>

  <button
    onclick={(event) => clicked(event.currentTarget)}
    class="w-full h-full rounded-full cursor-pointer bg-blue-950 hover:outline-2 hover:outline-blue-600"
    aria-label=" "
  >
    <LockIcon />
  </button>

  <div class="h-1"></div>

  {#if visible}
    <div class="absolute right-0 bg-blue-950 min-w-50 min-h-20 rounded-xl">
      {#if page === "loading"}
        <div class="flex items-center justify-center h-20">
          <div class="h-8">
            <Loader />
          </div>
        </div>
      {:else if page === "notsigned"}
        <p>Please sign in first</p>
      {:else if page == "home"}
        <AccountsList {accounts} {selected} />
      {/if}
    </div>
  {/if}
</div>
