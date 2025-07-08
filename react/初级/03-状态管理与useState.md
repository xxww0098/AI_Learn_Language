# 03 - 状态管理与 useState

## 什么是状态（State）？

状态是组件内部的数据，当状态改变时，组件会重新渲染。与 Props 不同，状态是可变的。

### 基本用法
```jsx
import React, { useState } from 'react';

function Counter() {
  const [count, setCount] = useState(0);
  
  return (
    <div>
      <p>当前计数：{count}</p>
      <button onClick={() => setCount(count + 1)}>
        增加
      </button>
      <button onClick={() => setCount(count - 1)}>
        减少
      </button>
      <button onClick={() => setCount(0)}>
        重置
      </button>
    </div>
  );
}
```

## 状态更新规则

### 直接更新
```jsx
function SimpleInput() {
  const [text, setText] = useState('');
  
  return (
    <div>
      <input 
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="请输入内容"
      />
      <p>你输入的内容：{text}</p>
    </div>
  );
}
```

### 基于前一个状态更新
```jsx
function AdvancedCounter() {
  const [count, setCount] = useState(0);
  
  const increment = () => {
    setCount(prevCount => prevCount + 1);
  };
  
  const decrementTwice = () => {
    setCount(prevCount => prevCount - 1);
    setCount(prevCount => prevCount - 1);
  };
  
  return (
    <div>
      <p>计数：{count}</p>
      <button onClick={increment}>+1</button>
      <button onClick={decrementTwice}>-2</button>
    </div>
  );
}
```

## 实践案例：待办事项列表

```jsx
function TodoList() {
  const [todos, setTodos] = useState([]);
  const [inputValue, setInputValue] = useState('');
  
  const addTodo = () => {
    if (inputValue.trim()) {
      setTodos([...todos, {
        id: Date.now(),
        text: inputValue,
        completed: false
      }]);
      setInputValue('');
    }
  };
  
  const toggleTodo = (id) => {
    setTodos(todos.map(todo => 
      todo.id === id 
        ? { ...todo, completed: !todo.completed }
        : todo
    ));
  };
  
  const deleteTodo = (id) => {
    setTodos(todos.filter(todo => todo.id !== id));
  };
  
  return (
    <div style={{ padding: '20px' }}>
      <h2>我的待办事项</h2>
      
      <div style={{ marginBottom: '20px' }}>
        <input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder="添加新任务..."
          onKeyPress={(e) => e.key === 'Enter' && addTodo()}
        />
        <button onClick={addTodo}>添加</button>
      </div>
      
      <ul style={{ listStyle: 'none', padding: 0 }}>
        {todos.map(todo => (
          <li key={todo.id} style={{
            padding: '10px',
            borderBottom: '1px solid #eee',
            display: 'flex',
            alignItems: 'center'
          }}>
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => toggleTodo(todo.id)}
            />
            <span style={{
              marginLeft: '10px',
              textDecoration: todo.completed ? 'line-through' : 'none',
              color: todo.completed ? '#888' : '#000'
            }}>
              {todo.text}
            </span>
            <button 
              onClick={() => deleteTodo(todo.id)}
              style={{ marginLeft: 'auto' }}
            >
              删除
            </button>
          </li>
        ))}
      </ul>
      
      <div style={{ marginTop: '20px' }}>
        <p>总计：{todos.length} 项</p>
        <p>已完成：{todos.filter(todo => todo.completed).length} 项</p>
      </div>
    </div>
  );
}
```

## 复杂状态管理

### 对象状态
```jsx
function UserProfile() {
  const [user, setUser] = useState({
    name: '',
    email: '',
    age: 0
  });
  
  const updateUser = (field, value) => {
    setUser(prevUser => ({
      ...prevUser,
      [field]: value
    }));
  };
  
  return (
    <div>
      <input
        placeholder="姓名"
        value={user.name}
        onChange={(e) => updateUser('name', e.target.value)}
      />
      <input
        placeholder="邮箱"
        value={user.email}
        onChange={(e) => updateUser('email', e.target.value)}
      />
      <input
        placeholder="年龄"
        type="number"
        value={user.age}
        onChange={(e) => updateUser('age', parseInt(e.target.value))}
      />
      
      <div>
        <h3>用户信息</h3>
        <p>姓名：{user.name}</p>
        <p>邮箱：{user.email}</p>
        <p>年龄：{user.age}</p>
      </div>
    </div>
  );
}
```

## 实践案例：简单的购物车

```jsx
function ShoppingCart() {
  const [cart, setCart] = useState([]);
  
  const products = [
    { id: 1, name: '苹果', price: 5 },
    { id: 2, name: '香蕉', price: 3 },
    { id: 3, name: '橙子', price: 4 }
  ];
  
  const addToCart = (product) => {
    setCart(prevCart => {
      const existingItem = prevCart.find(item => item.id === product.id);
      if (existingItem) {
        return prevCart.map(item =>
          item.id === product.id
            ? { ...item, quantity: item.quantity + 1 }
            : item
        );
      }
      return [...prevCart, { ...product, quantity: 1 }];
    });
  };
  
  const removeFromCart = (productId) => {
    setCart(prevCart => prevCart.filter(item => item.id !== productId));
  };
  
  const getTotalPrice = () => {
    return cart.reduce((total, item) => total + item.price * item.quantity, 0);
  };
  
  return (
    <div style={{ padding: '20px' }}>
      <h2>商品列表</h2>
      {products.map(product => (
        <div key={product.id} style={{ marginBottom: '10px' }}>
          <span>{product.name} - ¥{product.price}</span>
          <button 
            onClick={() => addToCart(product)}
            style={{ marginLeft: '10px' }}
          >
            加入购物车
          </button>
        </div>
      ))}
      
      <h2>购物车</h2>
      {cart.length === 0 ? (
        <p>购物车是空的</p>
      ) : (
        <div>
          {cart.map(item => (
            <div key={item.id} style={{ marginBottom: '10px' }}>
              <span>{item.name} x {item.quantity} = ¥{item.price * item.quantity}</span>
              <button 
                onClick={() => removeFromCart(item.id)}
                style={{ marginLeft: '10px' }}
              >
                移除
              </button>
            </div>
          ))}
          <h3>总计：¥{getTotalPrice()}</h3>
        </div>
      )}
    </div>
  );
}
```

## 练习任务

1. 创建一个表单组件，管理用户输入
2. 制作一个简单的计算器
3. 实现一个图片轮播组件

## 下一步

学会状态管理后，下一章将学习事件处理和表单操作。