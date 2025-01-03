<script>
  import { createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher();
  export let commentText = '';
  export let contact = '';
  export let contactType = 'Anonymous';
  export let duration = 24; // 默认24小时
  export let errorMessage = '';

  const contactOptions = [
    { value: 'Anonymous', label: '匿名' },
    { value: 'Wechat', label: '微信' },
    { value: 'Phone', label: '电话' },
    { value: 'Email', label: '邮箱' }
  ]


  function handleCancel() {
    dispatch('cancel');
  }
</script>

<div class="bg-white rounded-lg w-full max-w-md p-6 space-y-4">
  <h2 class="text-xl font-bold text-gray-800">写点什么</h2>
  
  <!-- 错误提示 -->
  {#if errorMessage}
    <div class="text-red-500 text-sm">{errorMessage}</div>
  {/if}

  <!-- 评论内容 -->
  <div>
    <textarea
      bind:value={commentText}
      placeholder="说点什么吧..."
      class="w-full h-32 border rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
    ></textarea>
  </div>
  
  <!-- 联系方式类型 -->
  <div>
    <select
      bind:value={contactType}
      class="w-full border rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      {#each contactOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </div>
  
  <!-- 联系方式输入 -->
  {#if contactType !== 'Anonymous'}
    <div>
      <input
        type="text"
        bind:value={contact}
        placeholder="输入联系方式"
        class="w-full border rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
  {/if}

  <!-- 存留时间 -->
  <div>
    <label for="duration" class="block text-sm text-gray-600 mb-2">存留时间（小时）</label>
    <input
      type="range"
      bind:value={duration}
      min="1"
      max="72"
      class="w-full"
    />
    <div class="text-center text-gray-600">{duration} 小时</div>
  </div>

  <!-- 按钮组 -->
  <div class="flex space-x-4">
    <button
      on:click={handleCancel}
      class="flex-1 bg-gray-500 hover:bg-gray-600 text-white py-2 rounded-lg transition"
    >
      取消
    </button>
    <button
      on:click={() => dispatch('submit')}
      class="flex-1 bg-blue-500 hover:bg-blue-600 text-white py-2 rounded-lg transition"
    >
      发布
    </button>
  </div>
</div>