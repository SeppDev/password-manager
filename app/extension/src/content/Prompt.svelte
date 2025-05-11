<script lang="ts">
    import Button from "../components/Button.svelte";
</script>

<div
    class="bg-neutral-900 outline-1 outline-black h-30 w-60 rounded-xl flex flex-col"
>
    <div class="grow px-3 py-2 text-white">
        <p>Save account</p>
    </div>
    <div class="flex p-2 space-x-2">
        <Button compact text="Cancel" />
        <Button fill_width compact text="Save" />
    </div>
</div>
