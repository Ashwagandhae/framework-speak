<script lang="ts">
  import { FrameworkReplacer } from 'vite-wasm-functions';

  const replacer = new FrameworkReplacer();
  let input: string = '';
  let temperature: number = 0.2;
  $: output = replacer.replace(input, temperature);
</script>

<main>
  <h1>framework-speak.js</h1>
  <label for="temp">Temperature</label>
  <p>
    {temperature}
  </p>
  <input type="range" min="0" max="1" step="0.01" bind:value={temperature} />

  <textarea bind:value={input} placeholder="Type here" rows="10" cols="50" />
  <p>{output}</p>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  textarea {
    font-family: inherit;
  }
</style>
