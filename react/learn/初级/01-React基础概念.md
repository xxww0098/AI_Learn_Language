# 01 - React 基础概念

## 什么是 React？

React 是一个用于构建用户界面的 JavaScript 库，由 Facebook 开发。它让我们能够创建交互式的 Web 应用程序。

### 核心特点
- **组件化**：将界面拆分成独立的、可重用的组件
- **声明式**：描述界面应该长什么样，而不是如何操作 DOM
- **Virtual DOM**：提高性能的虚拟 DOM 机制

## 第一个 React 应用

### 安装与创建项目
```bash
npx create-react-app my-first-app
cd my-first-app
npm start
```

### 基本组件结构
```jsx
// App.js
import React from 'react';

function App() {
  return (
    <div className="App">
      <h1>欢迎学习 React！</h1>
      <p>这是我的第一个 React 应用</p>
    </div>
  );
}

export default App;
```

## JSX 语法

JSX 是 JavaScript 的语法扩展，让我们可以在 JS 中写类似 HTML 的代码。

### 基本规则
```jsx
function Welcome() {
  const name = "小明";
  const isLoggedIn = true;
  
  return (
    <div>
      <h1>你好，{name}！</h1>
      {isLoggedIn && <p>欢迎回来</p>}
      <button onClick={() => alert('点击了按钮')}>
        点击我
      </button>
    </div>
  );
}
```

### 实践案例：个人名片组件
```jsx
function PersonCard() {
  const person = {
    name: "张三",
    age: 25,
    job: "前端开发者",
    avatar: "https://via.placeholder.com/100"
  };
  
  return (
    <div style={{
      border: '1px solid #ccc',
      padding: '20px',
      borderRadius: '8px',
      maxWidth: '300px'
    }}>
      <img 
        src={person.avatar} 
        alt={person.name}
        style={{borderRadius: '50%'}}
      />
      <h2>{person.name}</h2>
      <p>年龄：{person.age}</p>
      <p>职业：{person.job}</p>
    </div>
  );
}
```

## 练习任务

1. 创建一个问候组件，显示当前时间
2. 制作一个简单的计数器按钮
3. 设计一个产品卡片组件，显示产品信息

## 下一步

学习完基础概念后，下一章将介绍组件的 Props 和状态管理。