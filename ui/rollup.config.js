// 导入必要的 rollup 插件
import { spawn } from 'child_process';
import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import terser from '@rollup/plugin-terser';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import css from 'rollup-plugin-css-only';

// 判断是否为生产环境
const production = !process.env.ROLLUP_WATCH;

// 开发服务器启动函数
function serve() {
	let server;

	// 退出时清理服务器进程
	function toExit() {
		if (server) server.kill(0);
	}

	return {
		writeBundle() {
			if (server) return;
			// 启动开发服务器
			server = spawn('npm', ['run', 'start', '--', '--dev'], {
				stdio: ['ignore', 'inherit', 'inherit'],
				shell: true
			});

			// 监听进程退出信号
			process.on('SIGTERM', toExit);
			process.on('exit', toExit);
		}
	};
}

export default {
	// 入口文件配置
	input: 'src/main.js',
	// 输出配置
	output: {
		sourcemap: true,    // 生成 sourcemap
		format: 'iife',     // 输出格式为立即执行函数
		name: 'app',        // 全局变量名
		file: 'public/build/bundle.js'  // 输出文件路径
	},
	// 插件配置
	plugins: [
		svelte({
			compilerOptions: {
				// 非生产环境启用运行时检查
				dev: !production
			}
		}),
		// 提取组件 CSS 到单独文件，提升性能
		css({ output: 'bundle.css' }),

		// 解析第三方依赖
		resolve({
			browser: true,
			dedupe: ['svelte'],
			exportConditions: ['svelte']
		}),
		// 转换 CommonJS 模块为 ES6
		commonjs(),

		// 开发环境特有配置
		// 当 bundle 生成后启动开发服务器
		!production && serve(),

		// 监视 public 目录变化并刷新浏览器
		!production && livereload('public'),

		// 生产环境下压缩代码
		production && terser()
	],
	// 监视配置
	watch: {
		clearScreen: false  // 禁止清屏
	}
};