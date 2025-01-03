<script>
  import CommentCard from '../components/CommentCard.svelte';
  import CommentInput from '../components/CommentInput.svelte';

  // 是否显示输入表单
  let showInput = false;

  // 模拟数据
  let comments = [
    {
      id: 1,
      comment: "这是一条测试评论",
      contact_type: "Anonymous",
      contact: ""
    },
    {
      id: 2, 
      comment: "这是另一条测试评论",
      contact_type: "Wechat",
      contact: "test123"
    }
  ];

  let loading = false;
  let error = null;

  // 处理探索按钮点击
  async function handleExplore() {
    loading = true;
    try {
      // 模拟加载延迟
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // 添加一条新评论
      comments = [...comments, {
        id: comments.length + 1,
        comment: "新探索到的评论 " + (comments.length + 1),
        contact_type: "Anonymous",
        contact: ""
      }];
      
      loading = false;
    } catch (e) {
      error = "加载失败";
      loading = false;
    }
  }

  // 处理评论提交成功
  function handleCommentSubmit(event) {
    // 添加新评论
    comments = [...comments, {
      id: comments.length + 1,
      comment: event.detail.commentText,
      contact_type: event.detail.contactType,
      contact: event.detail.contact
    }];
    showInput = false;
  }
</script>

<div class="min-h-screen bg-gray-50 flex flex-col">
  <!-- 标题 -->
  <header class="py-6 bg-white shadow-sm">
    <h1 class="text-2xl font-bold text-center text-gray-800">有点想说</h1>
  </header>

  <!-- 主要内容区域 -->
  <main class="flex-1 container mx-auto px-4 py-8">
    <!-- 错误提示 -->
    {#if error}
      <div class="max-w-md mx-auto mb-4 p-4 bg-red-100 text-red-700 rounded-lg">
        {error}
      </div>
    {/if}

    <!-- 加载提示 -->
    {#if loading}
      <div class="max-w-md mx-auto mb-4 p-4 bg-blue-100 text-blue-700 rounded-lg">
        加载中...
      </div>
    {/if}

    <!-- 评论卡片列表 -->
    <div class="space-y-4">
      {#each comments as comment (comment.id)}
        <CommentCard {comment} />
      {/each}
    </div>
  </main>

  <!-- 底部按钮区域 -->
  <div class="sticky bottom-0 w-full bg-white shadow-lg p-4">
    <div class="max-w-md mx-auto flex justify-between space-x-4">
      {#if !showInput}
        <button
          on:click={handleExplore}
          disabled={loading}
          class="flex-1 bg-blue-500 hover:bg-blue-600 text-white py-3 px-6 rounded-lg transition disabled:opacity-50"
        >
          探索
        </button>
        <button
          on:click={() => showInput = true}
          disabled={loading}
          class="flex-1 bg-green-500 hover:bg-green-600 text-white py-3 px-6 rounded-lg transition disabled:opacity-50"
        >
          说点什么
        </button>
      {:else}
        <div class="w-full">
          <CommentInput on:submit={handleCommentSubmit} on:cancel={() => showInput = false} />
        </div>
      {/if}
    </div>
  </div>
</div>
