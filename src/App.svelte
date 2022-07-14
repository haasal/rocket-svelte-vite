<script lang="ts">
    let count: number = 0;
    const increment = () => {
        count += 1;
    };
</script>

<main class="flex justify-center items-center h-screen">
    <button on:click={increment} class="btn">
        COUNT IS {count}
    </button>
</main>
