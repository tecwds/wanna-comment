<script>
  export let comment;
</script>

<div class="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden">
  <div class="p-6">
    <p class="text-gray-700 text-lg mb-4">{comment.comment}</p>
    <div class="flex flex-col space-y-2">
      {#if comment.contact_type !== 'Anonymous' && comment.contact}
        <div class="flex items-center text-sm text-gray-500">
          <span class="mr-2">{
                            comment.contact_type === 'Wechat' ? '微信' : 
                            comment.contact_type === 'Phone' ? '电话' :
                            comment.contact_type === 'Email' ? '邮箱' : 
                            comment.contact_type === 'Anonymous' ? '匿名' : '未知' }:</span>
          <span>{comment.contact}</span>
        </div>
      {/if}
    </div>
  </div>
</div>