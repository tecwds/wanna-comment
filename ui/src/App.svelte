<script>
	import CommentCard from './components/CommentCard.svelte';
	import CommentInput from './components/CommentInput.svelte';
	
	// 当前显示的评论
	let currentComment = null;

	// 获取随机评论
	async function exploreComment() {
		try {
			const response = await fetch('/api/comments/random');
			currentComment = await response.json();
		} catch (error) {
			console.error('获取评论失败:', error);
		}
	}
</script>

<div class="min-h-screen bg-gray-50 flex flex-col">
	<!-- 标题 -->
	<header class="py-6 bg-white shadow-sm">
		<h1 class="text-2xl font-bold text-center text-gray-800">有点想说</h1>
	</header>

	<!-- 主要内容区域 -->
	<main class="flex-1 container mx-auto px-4 py-8">
		<!-- 探索按钮 -->
		<div class="flex justify-center mb-8">
			<button
				on:click={exploreComment}
				class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-6 rounded-lg
					   shadow-lg transform transition hover:scale-105"
			>
				探索新评论
			</button>
		</div>

		<!-- 评论卡片 -->
		{#if currentComment}
			<CommentCard {currentComment} />
		{/if}
	</main>

	<!-- 底部评论输入 -->
	<div class="sticky bottom-0 w-full bg-white shadow-lg">
		<CommentInput />
	</div>
</div>