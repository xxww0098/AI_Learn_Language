# Vue 3 初级教程

欢迎来到 Vue 3 初级教程！这里包含了从零基础到熟练使用 Vue 3 的完整学习路径。

## 📚 教程目录

### 01. Vue 3 基础入门
- Vue 3 核心概念介绍
- 安装和环境配置
- 基本语法和指令
- 实战项目：简单计数器

### 02. 数据绑定与事件处理
- 单向数据绑定
- 双向数据绑定（v-model）
- 事件处理机制
- 实战项目：待办事项列表

### 03. 列表渲染与条件渲染
- v-for 列表渲染
- v-if 条件渲染
- 动态列表操作
- 实战项目：购物车系统

### 04. 组件基础
- 组件概念和创建
- Props 数据传递
- 自定义事件
- 实战项目：多计数器应用

### 05. 插槽与组件通信
- 插槽系统详解
- provide/inject 依赖注入
- Event Bus 事件总线
- 实战项目：主题切换系统

### 06. 计算属性与侦听器
- 计算属性基础和高级用法
- getter/setter 双向计算属性
- 侦听器深入使用
- 实战项目：搜索过滤系统

### 07. 生命周期与错误处理
- 组件生命周期钩子
- Composition API 生命周期
- 错误边界和全局错误处理
- 实战项目：用户活动监控

## 🎯 学习目标

完成本教程后，你将能够：

- ✅ 理解 Vue 3 的核心概念和设计理念
- ✅ 熟练使用 Vue 3 的模板语法和指令
- ✅ 掌握组件化开发的方法
- ✅ 理解响应式数据和事件处理
- ✅ 能够构建中小型的 Vue 3 应用

## 📖 学习建议

### 适合人群
- 前端开发初学者
- 有其他框架经验想学习 Vue 3 的开发者
- 从 Vue 2 迁移到 Vue 3 的开发者

### 前置知识
- HTML、CSS 基础
- JavaScript ES6+ 语法
- 基本的编程概念

### 学习方法
1. **按顺序学习**：建议按照文件编号顺序逐步学习
2. **动手实践**：每个章节都有完整的代码示例，建议亲自动手实践
3. **完成项目**：每个章节的实战项目都要完成
4. **总结回顾**：学完每个章节后，总结关键知识点

## 🛠️ 开发环境

### 推荐工具
- **编辑器**：VS Code
- **浏览器**：Chrome/Firefox（支持 Vue Devtools）
- **包管理器**：npm 或 yarn
- **构建工具**：Vite（下一代前端构建工具）

### 为什么选择 Vite？

**Vite** 是由 Vue.js 创作者 Evan You 开发的下一代前端构建工具，在 2025 年已成为 Vue 3 开发的首选工具：

#### 核心优势
- ⚡ **极速启动**：开发服务器启动时间 < 1 秒
- 🔥 **闪电 HMR**：热模块替换 < 100ms
- 🌐 **原生 ES 模块**：浏览器直接处理模块导入
- 🚀 **esbuild 集成**：TypeScript 编译速度提升 20-30x
- 📦 **Rollup 生产构建**：高度优化的静态资源

#### 性能对比（2025 年基准）
| 功能 | Vite | Webpack |
|------|------|---------|
| 启动时间 | < 1 秒 | 5-10 秒 |
| HMR 速度 | < 100ms | 1-3 秒 |
| 构建时间 | 50% 更快 | 基准 |

### 环境配置
```bash
# 确保 Node.js 版本 (推荐 18+ 版本)
node --version

# 创建 Vue 3 + Vite 项目
npm create vue@latest my-vue-app
# 或者使用 yarn
yarn create vue my-vue-app

# 进入项目目录
cd my-vue-app

# 安装依赖
npm install

# 启动 Vite 开发服务器
npm run dev
```

### Vite 配置示例
```javascript
// vite.config.js
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 3000,
    open: true, // 自动打开浏览器
    proxy: {
      '/api': {
        target: 'http://localhost:4000',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    target: 'baseline-widely-available'
  }
})
```

### 开发脚本
```json
{
  "scripts": {
    "dev": "vite",              // 启动开发服务器
    "build": "vite build",      // 构建生产版本
    "preview": "vite preview"   // 预览生产构建
  }
}
```

## 📝 实战项目

每个章节都包含实际的项目案例：

1. **计数器应用** - 学习基本响应式和事件处理
2. **待办事项** - 掌握数据绑定和表单处理
3. **购物车系统** - 理解列表渲染和条件渲染
4. **多计数器应用** - 学习组件通信
5. **主题切换系统** - 掌握高级组件通信模式
6. **搜索过滤系统** - 学习计算属性和侦听器
7. **用户活动监控** - 理解生命周期和错误处理

## 📚 学习资源

### 官方文档
- [Vue 3 官方文档](https://vuejs.org/)
- [Vue 3 中文文档](https://cn.vuejs.org/)

### 开发工具
- [Vue Devtools](https://devtools.vuejs.org/)
- [Vite](https://vitejs.dev/)
- [Vue SFC Playground](https://sfc.vuejs.org/)

### 社区资源
- [Vue.js 官方论坛](https://github.com/vuejs/vue/discussions)
- [Vue.js 中文社区](https://www.vue-js.com/)

## 🔄 进阶学习

完成初级教程后，建议继续学习：

1. **Vue 3 高级教程** - 深入学习 Composition API、性能优化等
2. **Vue Router** - 单页面应用路由管理
3. **Pinia** - 现代化状态管理
4. **测试** - 单元测试和 E2E 测试
5. **部署** - 项目构建和部署

## 💡 学习技巧

1. **渐进式学习**：Vue 3 设计为渐进式框架，可以逐步引入高级特性
2. **实践导向**：通过构建实际项目来巩固知识
3. **阅读源码**：理解底层实现原理
4. **社区参与**：参与开源项目，与其他开发者交流

## 📞 获取帮助

如果在学习过程中遇到问题：

1. 仔细阅读错误信息和文档
2. 查看每个章节的"学习要点"部分
3. 尝试完成"练习建议"中的项目
4. 参考 Vue 3 官方文档和社区资源

祝你学习愉快！🎉