<script>
  import { save } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";

  let url = "";

  function handleClick() {
    invoke("video_info", {
      url,
    }).then((res) => {
      console.log(res);
    });
    save({ title: "Save Video", defaultPath: "inari.mp4" }).then((path) => {
      console.log(path);
    });
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
    <button class="btn" on:click={handleClick}>Download</button>
  </div>
</main>
