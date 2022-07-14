<script lang="ts">
    let count: number = 0;
    const increment = () => {
        count += 1;
    };
</script>

<button
    on:click={increment}
    class="rounded bg-slate-200 hover:bg-slate-300 m-2 py-1 w-32 transition-all"
>
    count is {count}
</button>
