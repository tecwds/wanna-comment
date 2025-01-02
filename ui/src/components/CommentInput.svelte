<script>
  let commentText = '';

  async function submitComment() {
    if (!commentText.trim()) return;

    try {
      const response = await fetch('/api/comments', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ content: commentText }),
      });

      if (response.ok) {
        commentText = '';
        alert('评论发布成功！');
      }
    } catch (error) {
      console.error('发布评论失败:', error);
    }
  }
</script>

<div class="max-w-md mx-auto p-4">
  <div class="flex space-x-4">
    <input
      type="text"
      bind:value={commentText}
      placeholder="写下你的评论..."
      class="flex-1 border rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
    <button
      on:click={submitComment}
      class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-2 rounded-lg transition"
    >
      发送
    </button>
  </div>
</div> 