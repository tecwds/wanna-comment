import request from '../utils/request';

/**
 * 评论相关的API服务
 */
export const commentAPI = {
    /**
     * 探索指定数量的随机评论
     * @param {number} count 需要获取的评论数量
     * @returns {Promise<Array>} 评论数组
     */
    explore(count) {
        return request.get(`/comments/${count}/explore`);
    },

    /**
     * 创建新评论
     * @param {Object} data 评论数据
     * @param {string} data.comment 评论内容
     * @param {string|null} data.contact 联系方式
     * @param {string} data.contact_type 联系方式类型
     * @param {number} data.duration 持续时间（秒）
     * @returns {Promise<Object>} 创建的评论
     */
    create(data) {
        return request.post('/comments', data);
    }
}; 