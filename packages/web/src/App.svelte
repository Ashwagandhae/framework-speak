<script lang="ts">
  import { FrameworkReplacer } from 'vite-wasm-functions';
  import Control from './lib/Control.svelte';

  let input: string = 'Change your point of view';
  let temperature: number = 0.2;
  let stars: number = 1000;
  export let wasmPromise: Promise<any>;

  let colorThemeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  document.body.classList.toggle('dark', colorThemeMediaQuery.matches);
  colorThemeMediaQuery.addEventListener('change', (e) => {
    document.body.classList.toggle('dark', e.matches);
  });

  let replacer: FrameworkReplacer | undefined = undefined;
  let loading = true;
  wasmPromise.then(() => {
    replacer = new FrameworkReplacer();
    loading = false;
  });
  $: output = replacer?.replace(input, temperature, stars);
</script>

<svelte:head>
  <title>framework-speak.js</title>
</svelte:head>
<main class:loading>
  <section class="top">
    <div class="info">
      <h1>framework-speak.js</h1>
      <p>
        Replace english words with phonetically similar JavaScript frameworks,
        to show your JavaScript skills.
      </p>
      <div class="links">
        <a href="https://github.com/Ashwagandhae/framework-speak.js">GitHub</a>
      </div>
    </div>
    <div class="controls">
      <Control
        label="Temperature"
        bind:value={temperature}
        min={0}
        max={5}
        step={0.01}
      />
      <Control
        label="Minimum stars"
        bind:value={stars}
        min={0}
        max={250_000}
        step={10}
      />
    </div>
  </section>
  {#if loading}
    <section class="loading">
      <div>Loading WebAssembly...</div>
    </section>
  {:else}
    <section class="input">
      <!-- svelte-ignore a11y-autofocus -->
      <!-- we can autofocus because this is the main control of the site -->
      <!-- google uses autofocus in their search bar aswell -->
      <textarea bind:value={input} placeholder="Type here" autofocus />
    </section>
    <section class="output">
      <div class="text">
        {#if output.length > 0}
          {#each output as replacement}
            <span class:replace={replacement.replace}>{replacement.word}</span>
          {/each}
        {:else}
          <span class="placeholder">Translation here</span>
        {/if}
      </div>
    </section>
  {/if}
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  :global(body) {
    --back: hsl(207, 26%, 88%);
    --fore: hsl(207, 26%, 95%);
    --fore-alt: hsl(207, 26%, 98%);
    --text-strong: hsl(200, 25%, 18%);
    --text: hsl(200, 25%, 30%);
    --text-weak: hsl(200, 10%, 50%);
    --text-accent: hsl(207, 70%, 50%);

    --pad: 0.8rem;
    --radius: 0.8rem;
  }
  :global(body.dark) {
    --back: hsl(207, 25%, 17%);
    --fore: hsl(205, 35%, 23%);
    --fore-alt: hsl(205, 22%, 28%);
    --text-strong: hsl(201, 20%, 90%);
    --text: hsl(201, 20%, 80%);
    --text-weak: hsl(201, 20%, 60%);
    --text-accent: hsl(207, 100%, 70%);
  }
  :global(body) {
    background: var(--back);
    margin: 0;
    box-sizing: border-box;
    color: var(--text);
  }
  h1 {
    font-weight: 600;
    margin: 0;
    color: var(--text-strong);
    font-size: 3rem;
  }

  main {
    width: 100vw;
    height: 100vh;
    padding: var(--pad);
    box-sizing: border-box;
    gap: var(--pad);
    display: grid;
    grid-template-columns: calc(50% - var(--pad) / 2) calc(50% - var(--pad) / 2);
    grid-template-rows: min-content 1fr;
    grid-template-areas:
      'info info'
      'input output';
  }
  main.loading {
    grid-template-areas:
      'info info'
      'loading loading';
  }
  @media (max-width: 800px) {
    main {
      grid-template-columns: 1fr;
      grid-template-rows: min-content calc(50vh - var(--pad) * 1.5) calc(
          50vh - var(--pad) * 1.5
        );
      height: auto;
      grid-template-areas:
        'info'
        'input'
        'output';
    }
    main.loading {
      grid-template-areas:
        'info'
        'loading'
        'loading';
    }
    h1 {
      font-size: 2rem;
    }
  }
  p {
    margin: 0;
  }
  section {
    height: 100%;
    width: auto;
    box-sizing: border-box;
    border-radius: var(--radius);
    padding: var(--pad);
    overflow: scroll;
  }
  section.top {
    grid-area: info;
    background: none;
  }
  section.input {
    grid-area: input;
    background: var(--fore);
  }
  section.output {
    grid-area: output;
    background: var(--fore-alt);
    color: var(--text-strong);
  }
  section.loading {
    grid-area: loading;
    background: var(--fore);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
  }

  textarea {
    border: none;
    resize: none;
    outline: none;
    padding: 0;
  }
  .text,
  textarea {
    display: block;
    font-family: inherit;
    font-size: 1.5rem;
    line-height: 1.5em;
    overflow-wrap: break-word;
    white-space: pre-line;
    width: 100%;
    height: 100%;
    color: inherit;
    background: none;
    overflow: scroll;
  }
  textarea::placeholder,
  .placeholder {
    color: var(--text-weak);
  }

  section.top {
    display: flex;
    flex-direction: row;
    /* push to either ends */
    justify-content: space-between;

    gap: var(--pad);
  }
  .controls {
    display: flex;
    flex-direction: column;
    gap: var(--pad);
    width: min(25rem, 100%);
    align-items: center;
  }
  @media (max-width: 800px) {
    section.top {
      flex-direction: column;
      align-items: center;
    }
  }

  span {
    color: var(--text);
  }
  span.replace {
    color: var(--text-accent);
  }
  .links {
    display: flex;
    flex-direction: row;
    gap: var(--pad);
  }
  .links a {
    color: var(--text-accent);
    text-decoration: none;
  }
  .links a:hover {
    text-decoration: underline;
  }
</style>
