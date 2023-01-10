<script lang="ts">
  import { save } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";

  let url = "";
  let loading = false;
  let errMsg = "";

  type VideoInfo = {
    title: string;
  };

  async function handleClick() {
    try {
      loading = true;
      if (!url) return;
      const { title } = await invoke<Partial<VideoInfo>>("video_info", {
        url,
      }).catch(() => ({ title: undefined }));
      if (!title) return (errMsg = "Video Not Found");
      const path = await save({
        title: "Save Video",
        defaultPath: `${title.replace(/[\\\/:\*\?\"<>\|]/g, ".")}.mp4`,
      });

      if (!path) return (errMsg = "No path selected");

      await invoke("download_video", { url, path });
    } finally {
      loading = false;
    }
  }
</script>

<main
  class="w-screen h-screen bg-slate-200 flex justify-center items-center flex-col gap-2"
>
  <h1 class="text-3xl font-bold">JUST Download Youtube</h1>
  <div class="flex w-full max-w-md gap-2">
    <input
      class="input input-bordered w-full"
      placeholder="Youtube URL"
      bind:value={url}
    />
    <button
      class={loading ? "loading btn" : "btn"}
      disabled={loading}
      on:click={handleClick}>Download</button
    >
  </div>
  {#if errMsg}
    <p class="text-error font-bold">{errMsg}</p>
  {/if}
</main>
