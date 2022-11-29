<script lang="ts">
  import { Router, Route, navigate } from "svelte-routing";
  import Editor from "./Editor.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import {create_site, list_sites, load_site} from "tauri-plugin-site-api";
  import type { Site } from 'tauri-plugin-site-api';
  
  let sites: [string];

  let editor_site: Site;
  let newSite = "";

  async function createNewSite() {
    try {
      let site = await create_site(newSite);
      openEditor(site.name);
    } catch (e) {
      console.log(e);
    }
  }

  async function openEditor(site_name: string) {
    editor_site = await load_site(site_name);
    navigate("/editor");
    await appWindow.setFullscreen(true);
  }

  function ignore() {}

  async function init() {
    try {
      sites = await list_sites();
    } catch (e) {
      //TODO replace with tauri logger
      console.log(e);
    }
  }

  init();
</script>

<Router url="/">
  <Route path="/editor"><Editor site = {editor_site} /></Route>
  <Route path="/">
    <main class="dark:bg-slate-800 dark:text-slate-400 font-mono h-screen">
      <div class="font-bold text-lx mb-2 p-5">Select Site to edit:</div>
      <div class="grid grid-cols-3 gap-4 p-5">
        {#each sites as site}
          <div
            class="max-w-sm rounded overflow-hidden border border-dashed border-yellow-600 hover:bg-slate-600/50 cursor-pointer"
            on:click={() => {
              openEditor(site);
            }}
            on:keyup={ignore}
          >
            <div class="px-6 py-4 align-middle">
              <p class="text-center">{site}</p>
            </div>
          </div>
        {/each}
      </div>

      <div class="p-5">
        <form class="mt-5 grid grid-cols-1 gap-2" autocomplete="off">
          <div class="text-center">Create new site:</div>
          <input
            class="text-black"
            type="text"
            placeholder="site name"
            bind:value={newSite}
          />
          <button
            class="outline-dashed outline-yellow-600 hover:bg-slate-600/50"
            on:click={() => {
              createNewSite();
            }}
            on:keyup={ignore}>Create</button
          >
        </form>
      </div>
    </main>
  </Route>
</Router>
