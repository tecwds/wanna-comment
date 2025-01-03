import { writable } from 'svelte/store';
import { commentService } from '../api/comment';

function createCommentStore() {
    const { subscribe, set, update } = writable({
        comments: [],
        loading: false,
        error: null
    });

    return {
        subscribe,
        // 探索新评论
        async explore() {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const count = Math.floor(Math.random() * 3) + 1;
                const newComments = await commentService.explore(count);
                
                update(state => {
                    // 过滤掉已存在的评论
                    const seenIds = new Set(state.comments.map(c => c.id));
                    const uniqueComments = newComments.filter(comment => !seenIds.has(comment.id));
                    
                    return {
                        ...state,
                        comments: [...state.comments, ...uniqueComments],
                        loading: false
                    };
                });
            } catch (error) {
                console.error('获取评论失败:', error);
                update(state => ({
                    ...state,
                    loading: false,
                    error: '获取评论失败'
                }));
            }
        },

        // 添加新评论
        async addComment(commentData) {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const newComment = await commentService.create(commentData);
                update(state => ({
                    ...state,
                    comments: [...state.comments, newComment],
                    loading: false
                }));
                return newComment;
            } catch (error) {
                console.error('发布评论失败:', error);
                update(state => ({
                    ...state,
                    loading: false,
                    error: '发布评论失败'
                }));
                throw error;
            }
        },

        // 清空评论列表
        clear() {
            update(state => ({ ...state, comments: [], error: null }));
        }
    };
}

export const commentStore = createCommentStore(); 