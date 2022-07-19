<script lang="ts">
    import svelteLogo from "./assets/svelte.svg";
    import Counter from "./lib/Counter.svelte";
</script>

<main>
    <div>
        <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo" alt="Vite Logo" />
        </a>
        <a href="https://svelte.dev" target="_blank">
            <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
        </a>
    </div>
    <h1>Vite + Svelte</h1>

    <div class="card">
        <Counter />
    </div>

    <p>
        Check out <a
            href="https://github.com/sveltejs/kit#readme"
            target="_blank">SvelteKit</a
        >, the official Svelte app framework powered by Vite!
    </p>

    <p class="read-the-docs">
        Click on the Vite and Svelte logos to learn more
    </p>

    <p>
        {#await fetch("/api/hello")}
            Waiting for rocket server response... <br />
            (Works only if rocket server was started. Make sure your run
            <code>npm run dev:serve</code> )
        {:then resp}
            {#await resp.text() then value}
                The Rocket server greets you: "{value
                    ? value
                    : "EMPTY RESPONSE"}"
            {/await}
        {:catch error}
            Some error occured when fetching from rust server: <br />
            "{error}"
        {/await}
    </p>
</main>

<style>
    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
    }
    .logo:hover {
        filter: drop-shadow(0 0 2em #646cffaa);
    }
    .logo.svelte:hover {
        filter: drop-shadow(0 0 2em #ff3e00aa);
    }
    .read-the-docs {
        color: #888;
    }
</style>
