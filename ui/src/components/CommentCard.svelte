<script>
  import { onDestroy } from 'svelte';
  
  export let currentComment;
  let remainingTime = '';

  function updateRemainingTime() {
    const remaining = Math.max(0, currentComment.expirationTime - Date.now());
    const hours = Math.floor(remaining / (1000 * 60 * 60));
    remainingTime = `${hours}小时`;
  }

  async function handleLike() {
    try {
      const response = await fetch(`/api/comments/${currentComment.id}/like`, {
        method: 'POST'
      });
      const updatedComment = await response.json();
      currentComment = updatedComment;
    } catch (error) {
      console.error('点赞失败:', error);
    }
  }

  $: if (currentComment) {
    updateRemainingTime();
    const timer = setInterval(updateRemainingTime, 60000);
    onDestroy(() => clearInterval(timer));
  }
</script>

<div class="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden">
  <div class="p-6">
    <p class="text-gray-700 text-lg mb-4">{currentComment.content}</p>
    <div class="flex justify-between items-center">
      <button
        on:click={handleLike}
        class="flex items-center space-x-2 text-gray-600 hover:text-red-500 transition"
      >
        <svg
          class="w-6 h-6"
          fill={currentComment.likes > 0 ? "currentColor" : "none"}
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
          />
        </svg>
        <span>{currentComment.likes}</span>
      </button>
      <div class="text-sm text-gray-500">
        剩余时间: {remainingTime}
      </div>
    </div>
  </div>
</div> 