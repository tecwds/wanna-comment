const BASE_URL = 'http://127.0.0.1:8000/api';

/**
 * 请求配置的默认值
 */
const defaultConfig = {
    // 基础URL
    baseURL: BASE_URL,
    // 请求头
    headers: {
        'Content-Type': 'application/json'
    },
    // 请求超时时间
    timeout: 10000
};

/**
 * 处理响应
 * @param {Response} response Fetch API 的响应对象
 */
async function handleResponse(response) {
    if (!response.ok) {
        const error = await response.text();
        throw new Error(error || `HTTP error! status: ${response.status}`);
    }
    return response.json();
}

/**
 * 创建完整的请求URL
 * @param {string} url 请求路径
 * @param {Object} params URL参数
 */
function createURL(url, params) {
    const finalUrl = new URL(url, defaultConfig.baseURL);
    if (params) {
        Object.keys(params).forEach(key => 
            finalUrl.searchParams.append(key, params[key])
        );
    }
    return finalUrl;
}

/**
 * HTTP请求工具类
 */
class Request {
    /**
     * GET请求
     * @param {string} url 请求路径
     * @param {Object} params URL参数
     * @param {Object} config 请求配置
     */
    async get(url, params = null, config = {}) {
        const finalUrl = createURL(url, params);
        const response = await fetch(finalUrl, {
            ...defaultConfig,
            ...config,
            method: 'GET'
        });
        return handleResponse(response);
    }

    /**
     * POST请求
     * @param {string} url 请求路径
     * @param {Object} data 请求体数据
     * @param {Object} config 请求配置
     */
    async post(url, data = null, config = {}) {
        const finalUrl = createURL(url);
        const response = await fetch(finalUrl, {
            ...defaultConfig,
            ...config,
            method: 'POST',
            body: data ? JSON.stringify(data) : null
        });
        return handleResponse(response);
    }

    /**
     * PUT请求
     * @param {string} url 请求路径
     * @param {Object} data 请求体数据
     * @param {Object} config 请求配置
     */
    async put(url, data = null, config = {}) {
        const finalUrl = createURL(url);
        const response = await fetch(finalUrl, {
            ...defaultConfig,
            ...config,
            method: 'PUT',
            body: data ? JSON.stringify(data) : null
        });
        return handleResponse(response);
    }

    /**
     * DELETE请求
     * @param {string} url 请求路径
     * @param {Object} config 请求配置
     */
    async delete(url, config = {}) {
        const finalUrl = createURL(url);
        const response = await fetch(finalUrl, {
            ...defaultConfig,
            ...config,
            method: 'DELETE'
        });
        return handleResponse(response);
    }
}

export default new Request(); 