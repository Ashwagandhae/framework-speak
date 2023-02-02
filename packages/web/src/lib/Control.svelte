<script lang="ts">
  export let value: number;
  export let min: number;
  export let max: number;
  export let step: number;
  export let label: string;
</script>

<div class="control">
  <div class="top">
    <label for={label}>{label}</label>
    <span>
      {value}
    </span>
  </div>
  <input type="range" {min} {max} {step} bind:value />
</div>

<style>
  .control {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 0.5rem;
  }

  .top {
    display: flex;
    justify-content: space-between;
  }
</style>
