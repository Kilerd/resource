<script>
  import Tailwindcss from "./Tailwindcss.svelte";
  import marked from "marked";
  import { each } from "svelte/internal";

  let item_id;
  let isShow = false;
  let promise = getRedditTrending();

  async function getRedditTrending() {
    let ret = await fetch("https://api.resource.rs/trending");
    return ret.json();
  }
  function onClick(id) {
    if (item_id === id) {
      isShow = !isShow;
    } else {
      isShow = true;
    }
    item_id = id;
  }

  function getDateFormat(date) {
    let d = new Date(date)
    return `${d.getFullYear()}-${("0"+(d.getMonth()+1)).slice(-2)}-${("0" + d.getDate()).slice(-2)}`
  }
</script>

<style>
@keyframes loader-rotate {
		0% {
			transform: rotate(0);
		}
		100% {
			transform: rotate(360deg);
		}
	}
	.loader {
		border-right-color: transparent;
		animation: loader-rotate 1s linear infinite;
	}

h1 {
  font-size: 1.75rem;
  display: inline-block;
  margin: 0;
  padding: 0;
}
h1:after {
  position: relative;
  top: -10px;
  background-color: #ffc832;
  display: inline-block;
  height: 6px;
  width: 100%;
  max-width: 90vw;
  line-height: 1;
  border-radius: 2px;
  content: " ";
}
.right {
  min-width: 85px;
}
</style>

<svelte:head>
  <title>Reddit Trending</title>
</svelte:head>
<Tailwindcss />
<main>
  <section class="container">
    <section class="trending">
      <h1 class="head">Reddit Trending</h1>

      {#await promise}
      <div class="flex flex-row justify-center items-center">
      <div class="w-12 h-12 border-2 border-red-600 rounded-full loader"></div>
      </div>
        
        {:then data}
        {#each data.data as item (item.id)}
          
          <section
            class="item my-2 py-3 px-4  bg-gray-100 border {item_id === item.id && isShow? "rounded border-black":"border-gray-100"}">
            <div
              class="title flex flex-row justify-between items-center {item.selftext !== null ? 'cursor-pointer' : ''}"
              on:click={() => onClick(item.id)}>
              <div
                class="left flex flex-row items-center text-lg font-medium
                  text-gray-800">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="w-5 h-5 mx-2">
                  {#if item_id === item.id && isShow}
                    <polyline points="18 15 12 9 6 15" />
                  {:else}
                    <polyline points="6 9 12 15 18 9" />
                  {/if}
                </svg>
                <span
                  class="score text-sm px-2 py-1 bg-gray-400 rounded mr-2
                    font-normal">{item.score}</span>
                {item.title}
              </div>
              <div class="right text-gray-500 text-sm">{getDateFormat(item.create_time)}</div>
            </div>
            {#if item_id === item.id && isShow}
            <div class="flex flex-row justify-end">
            <a href="https://www.reddit.com{item.permalink}" class="btn-primary transition duration-300 ease-in-out focus:outline-none focus:shadow-outline bg-teal-700 hover:bg-teal-900 text-white font-normal py-2 px-4 mr-1 rounded">Reddit</a>
            {#if !item.url.startsWith("https://www.reddit.com")}
            <a  href="{item.url}" class="btn-outline-primary transition duration-300 ease-in-out focus:outline-none focus:shadow-outline border border-teal-700 hover:bg-teal-700 text-teal-700 hover:text-white font-normal py-2 px-4 rounded">原地址</a>
            {/if}
            </div>
              <div class="content yue px-4 pt-4 black">
                {@html marked(item.selftext || '')}
              </div>
            {/if}
          </section>
        {/each}
      {:catch e}
        error
      {/await}
    </section>
  </section>
</main>
