# 02 - 组件与 Props

## 什么是组件？

组件是 React 应用的基本构建块。就像搭积木一样，我们用组件来构建整个应用。

### 函数组件
```jsx
function Greeting(props) {
  return <h1>你好，{props.name}！</h1>;
}

// 使用组件
<Greeting name="小李" />
```

### 组件嵌套
```jsx
function UserProfile() {
  return (
    <div>
      <Avatar />
      <UserInfo />
      <UserActions />
    </div>
  );
}
```

## Props 详解

Props 是组件间传递数据的方式，类似于函数的参数。

### 基础用法
```jsx
function ProductCard({ title, price, image, onSale }) {
  return (
    <div className="product-card">
      <img src={image} alt={title} />
      <h3>{title}</h3>
      <p className={onSale ? 'sale-price' : 'regular-price'}>
        ¥{price}
      </p>
      {onSale && <span className="sale-badge">特价</span>}
    </div>
  );
}
```

### 使用组件
```jsx
function App() {
  return (
    <div>
      <ProductCard 
        title="iPhone 15" 
        price={6999} 
        image="/images/iphone15.jpg"
        onSale={true}
      />
      <ProductCard 
        title="MacBook Pro" 
        price={12999} 
        image="/images/macbook.jpg"
        onSale={false}
      />
    </div>
  );
}
```

## 实践案例：博客文章列表

### 单个文章组件
```jsx
function BlogPost({ title, author, date, content, tags }) {
  return (
    <article style={{
      border: '1px solid #e0e0e0',
      borderRadius: '8px',
      padding: '20px',
      marginBottom: '20px'
    }}>
      <header>
        <h2>{title}</h2>
        <div style={{color: '#666', fontSize: '14px'}}>
          作者：{author} | 发布时间：{date}
        </div>
      </header>
      <main>
        <p>{content}</p>
      </main>
      <footer>
        <div>
          {tags.map(tag => (
            <span key={tag} style={{
              backgroundColor: '#f0f0f0',
              padding: '4px 8px',
              borderRadius: '4px',
              marginRight: '8px'
            }}>
              {tag}
            </span>
          ))}
        </div>
      </footer>
    </article>
  );
}
```

### 文章列表组件
```jsx
function BlogList() {
  const posts = [
    {
      id: 1,
      title: "React 入门指南",
      author: "张三",
      date: "2024-01-15",
      content: "React 是一个强大的前端库...",
      tags: ["React", "JavaScript", "前端"]
    },
    {
      id: 2,
      title: "JavaScript ES6 特性",
      author: "李四",
      date: "2024-01-10",
      content: "ES6 带来了许多新特性...",
      tags: ["JavaScript", "ES6"]
    }
  ];
  
  return (
    <div>
      <h1>技术博客</h1>
      {posts.map(post => (
        <BlogPost 
          key={post.id}
          title={post.title}
          author={post.author}
          date={post.date}
          content={post.content}
          tags={post.tags}
        />
      ))}
    </div>
  );
}
```

## Props 的默认值

```jsx
function Button({ text, color, size, onClick }) {
  return (
    <button 
      style={{
        backgroundColor: color,
        fontSize: size === 'large' ? '18px' : '14px',
        padding: size === 'large' ? '12px 24px' : '8px 16px'
      }}
      onClick={onClick}
    >
      {text}
    </button>
  );
}

// 设置默认值
Button.defaultProps = {
  color: '#007bff',
  size: 'medium',
  text: '按钮'
};
```

## 练习任务

1. 创建一个学生信息卡片组件
2. 制作一个商品展示组件，包含图片、名称、价格
3. 设计一个导航菜单组件，支持传入菜单项

## 下一步

掌握了 Props 后，下一章将学习如何使用 State 来管理组件的状态。