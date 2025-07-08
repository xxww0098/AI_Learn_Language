# 02 - Context 与状态管理

## Context API 基础

### 创建和使用 Context
```jsx
import React, { createContext, useContext, useState, useReducer } from 'react';

// 创建 Context
const ThemeContext = createContext();
const UserContext = createContext();

// Provider 组件
function ThemeProvider({ children }) {
  const [theme, setTheme] = useState('light');
  
  const toggleTheme = () => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  };
  
  const value = {
    theme,
    toggleTheme
  };
  
  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  );
}

// 自定义 Hook 简化使用
function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}

// 使用 Context 的组件
function Header() {
  const { theme, toggleTheme } = useTheme();
  
  const styles = {
    backgroundColor: theme === 'light' ? '#fff' : '#333',
    color: theme === 'light' ? '#333' : '#fff',
    padding: '20px'
  };
  
  return (
    <header style={styles}>
      <h1>我的应用</h1>
      <button onClick={toggleTheme}>
        切换到 {theme === 'light' ? '暗色' : '亮色'} 主题
      </button>
    </header>
  );
}

function Content() {
  const { theme } = useTheme();
  
  const styles = {
    backgroundColor: theme === 'light' ? '#f5f5f5' : '#444',
    color: theme === 'light' ? '#333' : '#fff',
    padding: '20px',
    minHeight: '200px'
  };
  
  return (
    <div style={styles}>
      <p>这是主要内容区域</p>
      <p>当前主题：{theme}</p>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <Header />
      <Content />
    </ThemeProvider>
  );
}
```

## useReducer 状态管理

### 复杂状态管理
```jsx
import React, { useReducer, useContext, createContext } from 'react';

// 定义状态类型
const ActionTypes = {
  ADD_TODO: 'ADD_TODO',
  TOGGLE_TODO: 'TOGGLE_TODO',
  DELETE_TODO: 'DELETE_TODO',
  SET_FILTER: 'SET_FILTER',
  CLEAR_COMPLETED: 'CLEAR_COMPLETED'
};

// 初始状态
const initialState = {
  todos: [],
  filter: 'all' // all, active, completed
};

// Reducer 函数
function todoReducer(state, action) {
  switch (action.type) {
    case ActionTypes.ADD_TODO:
      return {
        ...state,
        todos: [
          ...state.todos,
          {
            id: Date.now(),
            text: action.payload,
            completed: false,
            createdAt: new Date()
          }
        ]
      };
      
    case ActionTypes.TOGGLE_TODO:
      return {
        ...state,
        todos: state.todos.map(todo =>
          todo.id === action.payload
            ? { ...todo, completed: !todo.completed }
            : todo
        )
      };
      
    case ActionTypes.DELETE_TODO:
      return {
        ...state,
        todos: state.todos.filter(todo => todo.id !== action.payload)
      };
      
    case ActionTypes.SET_FILTER:
      return {
        ...state,
        filter: action.payload
      };
      
    case ActionTypes.CLEAR_COMPLETED:
      return {
        ...state,
        todos: state.todos.filter(todo => !todo.completed)
      };
      
    default:
      return state;
  }
}

// Context 创建
const TodoContext = createContext();

// Provider 组件
function TodoProvider({ children }) {
  const [state, dispatch] = useReducer(todoReducer, initialState);
  
  // Action creators
  const actions = {
    addTodo: (text) => dispatch({ type: ActionTypes.ADD_TODO, payload: text }),
    toggleTodo: (id) => dispatch({ type: ActionTypes.TOGGLE_TODO, payload: id }),
    deleteTodo: (id) => dispatch({ type: ActionTypes.DELETE_TODO, payload: id }),
    setFilter: (filter) => dispatch({ type: ActionTypes.SET_FILTER, payload: filter }),
    clearCompleted: () => dispatch({ type: ActionTypes.CLEAR_COMPLETED })
  };
  
  // 过滤后的 todos
  const filteredTodos = state.todos.filter(todo => {
    switch (state.filter) {
      case 'active':
        return !todo.completed;
      case 'completed':
        return todo.completed;
      default:
        return true;
    }
  });
  
  const value = {
    todos: filteredTodos,
    filter: state.filter,
    stats: {
      total: state.todos.length,
      active: state.todos.filter(todo => !todo.completed).length,
      completed: state.todos.filter(todo => todo.completed).length
    },
    actions
  };
  
  return (
    <TodoContext.Provider value={value}>
      {children}
    </TodoContext.Provider>
  );
}

// 自定义 Hook
function useTodos() {
  const context = useContext(TodoContext);
  if (!context) {
    throw new Error('useTodos must be used within a TodoProvider');
  }
  return context;
}

// 组件
function TodoInput() {
  const [text, setText] = useState('');
  const { actions } = useTodos();
  
  const handleSubmit = (e) => {
    e.preventDefault();
    if (text.trim()) {
      actions.addTodo(text.trim());
      setText('');
    }
  };
  
  return (
    <form onSubmit={handleSubmit}>
      <input
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="添加新任务..."
      />
      <button type="submit">添加</button>
    </form>
  );
}

function TodoList() {
  const { todos, actions } = useTodos();
  
  return (
    <ul>
      {todos.map(todo => (
        <li key={todo.id} style={{
          textDecoration: todo.completed ? 'line-through' : 'none'
        }}>
          <input
            type="checkbox"
            checked={todo.completed}
            onChange={() => actions.toggleTodo(todo.id)}
          />
          <span>{todo.text}</span>
          <button onClick={() => actions.deleteTodo(todo.id)}>
            删除
          </button>
        </li>
      ))}
    </ul>
  );
}

function TodoFilter() {
  const { filter, actions } = useTodos();
  
  const filters = [
    { key: 'all', label: '全部' },
    { key: 'active', label: '进行中' },
    { key: 'completed', label: '已完成' }
  ];
  
  return (
    <div>
      {filters.map(f => (
        <button
          key={f.key}
          onClick={() => actions.setFilter(f.key)}
          style={{
            backgroundColor: filter === f.key ? '#007bff' : '#f8f9fa',
            color: filter === f.key ? 'white' : 'black'
          }}
        >
          {f.label}
        </button>
      ))}
    </div>
  );
}

function TodoStats() {
  const { stats, actions } = useTodos();
  
  return (
    <div>
      <p>总计：{stats.total} | 进行中：{stats.active} | 已完成：{stats.completed}</p>
      {stats.completed > 0 && (
        <button onClick={actions.clearCompleted}>
          清除已完成
        </button>
      )}
    </div>
  );
}

function TodoApp() {
  return (
    <TodoProvider>
      <div style={{ padding: '20px' }}>
        <h1>Todo 应用</h1>
        <TodoInput />
        <TodoFilter />
        <TodoList />
        <TodoStats />
      </div>
    </TodoProvider>
  );
}
```

## 全局状态管理模式

### 创建全局状态管理器
```jsx
import React, { createContext, useContext, useReducer, useCallback, useMemo } from 'react';

// 状态类型定义
const StateActionTypes = {
  // 用户相关
  SET_USER: 'SET_USER',
  LOGOUT: 'LOGOUT',
  UPDATE_USER_PROFILE: 'UPDATE_USER_PROFILE',
  
  // 通知相关
  ADD_NOTIFICATION: 'ADD_NOTIFICATION',
  REMOVE_NOTIFICATION: 'REMOVE_NOTIFICATION',
  
  // 加载状态
  SET_LOADING: 'SET_LOADING',
  
  // 错误处理
  SET_ERROR: 'SET_ERROR',
  CLEAR_ERROR: 'CLEAR_ERROR'
};

// 初始状态
const initialGlobalState = {
  user: null,
  notifications: [],
  loading: {},
  errors: {}
};

// 全局 Reducer
function globalReducer(state, action) {
  switch (action.type) {
    case StateActionTypes.SET_USER:
      return {
        ...state,
        user: action.payload
      };
      
    case StateActionTypes.LOGOUT:
      return {
        ...state,
        user: null
      };
      
    case StateActionTypes.UPDATE_USER_PROFILE:
      return {
        ...state,
        user: {
          ...state.user,
          ...action.payload
        }
      };
      
    case StateActionTypes.ADD_NOTIFICATION:
      return {
        ...state,
        notifications: [
          ...state.notifications,
          {
            id: Date.now(),
            ...action.payload,
            timestamp: new Date()
          }
        ]
      };
      
    case StateActionTypes.REMOVE_NOTIFICATION:
      return {
        ...state,
        notifications: state.notifications.filter(
          notification => notification.id !== action.payload
        )
      };
      
    case StateActionTypes.SET_LOADING:
      return {
        ...state,
        loading: {
          ...state.loading,
          [action.payload.key]: action.payload.value
        }
      };
      
    case StateActionTypes.SET_ERROR:
      return {
        ...state,
        errors: {
          ...state.errors,
          [action.payload.key]: action.payload.error
        }
      };
      
    case StateActionTypes.CLEAR_ERROR:
      return {
        ...state,
        errors: {
          ...state.errors,
          [action.payload]: null
        }
      };
      
    default:
      return state;
  }
}

// Context 创建
const GlobalStateContext = createContext();

// 全局状态 Provider
function GlobalStateProvider({ children }) {
  const [state, dispatch] = useReducer(globalReducer, initialGlobalState);
  
  // Action creators
  const actions = useMemo(() => ({
    // 用户操作
    setUser: (user) => dispatch({ type: StateActionTypes.SET_USER, payload: user }),
    logout: () => dispatch({ type: StateActionTypes.LOGOUT }),
    updateUserProfile: (updates) => dispatch({ 
      type: StateActionTypes.UPDATE_USER_PROFILE, 
      payload: updates 
    }),
    
    // 通知操作
    addNotification: (notification) => dispatch({ 
      type: StateActionTypes.ADD_NOTIFICATION, 
      payload: notification 
    }),
    removeNotification: (id) => dispatch({ 
      type: StateActionTypes.REMOVE_NOTIFICATION, 
      payload: id 
    }),
    
    // 加载状态操作
    setLoading: (key, value) => dispatch({ 
      type: StateActionTypes.SET_LOADING, 
      payload: { key, value } 
    }),
    
    // 错误处理操作
    setError: (key, error) => dispatch({ 
      type: StateActionTypes.SET_ERROR, 
      payload: { key, error } 
    }),
    clearError: (key) => dispatch({ 
      type: StateActionTypes.CLEAR_ERROR, 
      payload: key 
    })
  }), []);
  
  // 高级操作（组合多个 action）
  const complexActions = useMemo(() => ({
    // 异步登录
    login: useCallback(async (credentials) => {
      actions.setLoading('login', true);
      actions.clearError('login');
      
      try {
        const response = await fetch('/api/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(credentials)
        });
        
        if (!response.ok) {
          throw new Error('登录失败');
        }
        
        const user = await response.json();
        actions.setUser(user);
        actions.addNotification({
          type: 'success',
          message: '登录成功'
        });
      } catch (error) {
        actions.setError('login', error.message);
      } finally {
        actions.setLoading('login', false);
      }
    }, [actions]),
    
    // 异步更新用户资料
    updateProfile: useCallback(async (updates) => {
      actions.setLoading('updateProfile', true);
      actions.clearError('updateProfile');
      
      try {
        const response = await fetch('/api/profile', {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(updates)
        });
        
        if (!response.ok) {
          throw new Error('更新失败');
        }
        
        const updatedUser = await response.json();
        actions.updateUserProfile(updatedUser);
        actions.addNotification({
          type: 'success',
          message: '资料更新成功'
        });
      } catch (error) {
        actions.setError('updateProfile', error.message);
      } finally {
        actions.setLoading('updateProfile', false);
      }
    }, [actions])
  }), [actions]);
  
  const value = {
    state,
    actions: { ...actions, ...complexActions }
  };
  
  return (
    <GlobalStateContext.Provider value={value}>
      {children}
    </GlobalStateContext.Provider>
  );
}

// 自定义 Hooks
function useGlobalState() {
  const context = useContext(GlobalStateContext);
  if (!context) {
    throw new Error('useGlobalState must be used within a GlobalStateProvider');
  }
  return context;
}

function useAuth() {
  const { state, actions } = useGlobalState();
  return {
    user: state.user,
    isAuthenticated: !!state.user,
    login: actions.login,
    logout: actions.logout,
    updateProfile: actions.updateProfile,
    loading: state.loading.login || false,
    error: state.errors.login
  };
}

function useNotifications() {
  const { state, actions } = useGlobalState();
  return {
    notifications: state.notifications,
    addNotification: actions.addNotification,
    removeNotification: actions.removeNotification
  };
}

// 使用示例组件
function LoginForm() {
  const [credentials, setCredentials] = useState({ username: '', password: '' });
  const { login, loading, error } = useAuth();
  
  const handleSubmit = (e) => {
    e.preventDefault();
    login(credentials);
  };
  
  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        placeholder="用户名"
        value={credentials.username}
        onChange={(e) => setCredentials(prev => ({ ...prev, username: e.target.value }))}
      />
      <input
        type="password"
        placeholder="密码"
        value={credentials.password}
        onChange={(e) => setCredentials(prev => ({ ...prev, password: e.target.value }))}
      />
      <button type="submit" disabled={loading}>
        {loading ? '登录中...' : '登录'}
      </button>
      {error && <p style={{ color: 'red' }}>{error}</p>}
    </form>
  );
}

function NotificationList() {
  const { notifications, removeNotification } = useNotifications();
  
  return (
    <div>
      {notifications.map(notification => (
        <div key={notification.id} style={{
          padding: '10px',
          margin: '5px',
          backgroundColor: notification.type === 'success' ? '#d4edda' : '#f8d7da',
          border: '1px solid',
          borderColor: notification.type === 'success' ? '#c3e6cb' : '#f5c6cb'
        }}>
          <span>{notification.message}</span>
          <button onClick={() => removeNotification(notification.id)}>
            ×
          </button>
        </div>
      ))}
    </div>
  );
}

function UserProfile() {
  const { user, updateProfile, isAuthenticated } = useAuth();
  
  if (!isAuthenticated) {
    return <LoginForm />;
  }
  
  return (
    <div>
      <h2>用户资料</h2>
      <p>用户名：{user.username}</p>
      <p>邮箱：{user.email}</p>
      <button onClick={() => updateProfile({ lastLogin: new Date() })}>
        更新最后登录时间
      </button>
    </div>
  );
}

function App() {
  return (
    <GlobalStateProvider>
      <div style={{ padding: '20px' }}>
        <h1>全局状态管理示例</h1>
        <NotificationList />
        <UserProfile />
      </div>
    </GlobalStateProvider>
  );
}
```

## 性能优化

### Context 性能优化
```jsx
import React, { createContext, useContext, useMemo, memo } from 'react';

// 分离不同的 Context 避免不必要的重新渲染
const UserContext = createContext();
const ThemeContext = createContext();

function OptimizedProvider({ children }) {
  const [user, setUser] = useState(null);
  const [theme, setTheme] = useState('light');
  
  // 使用 useMemo 缓存 value
  const userValue = useMemo(() => ({
    user,
    setUser
  }), [user]);
  
  const themeValue = useMemo(() => ({
    theme,
    setTheme
  }), [theme]);
  
  return (
    <UserContext.Provider value={userValue}>
      <ThemeContext.Provider value={themeValue}>
        {children}
      </ThemeContext.Provider>
    </UserContext.Provider>
  );
}

// 使用 memo 优化子组件
const UserComponent = memo(() => {
  const { user } = useContext(UserContext);
  console.log('UserComponent 重新渲染');
  
  return <div>用户：{user?.name}</div>;
});

const ThemeComponent = memo(() => {
  const { theme } = useContext(ThemeContext);
  console.log('ThemeComponent 重新渲染');
  
  return <div>主题：{theme}</div>;
});
```

## 练习任务

1. 实现一个购物车的全局状态管理
2. 创建一个多语言切换的 Context
3. 实现一个权限管理系统的状态管理

## 下一步

掌握状态管理后，下一章将学习性能优化技巧。