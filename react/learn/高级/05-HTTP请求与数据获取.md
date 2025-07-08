# 05 - HTTP 请求与数据获取

## Fetch API 基础

### 基本 GET 请求
```jsx
import React, { useState, useEffect } from 'react';

function BasicDataFetching() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoading(true);
        const response = await fetch('https://jsonplaceholder.typicode.com/users');
        
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        setUsers(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };
    
    fetchUsers();
  }, []);
  
  if (loading) return <div>加载中...</div>;
  if (error) return <div>错误: {error}</div>;
  
  return (
    <div>
      <h2>用户列表</h2>
      {users.map(user => (
        <div key={user.id} style={{ 
          padding: '10px', 
          border: '1px solid #ddd', 
          marginBottom: '10px' 
        }}>
          <h4>{user.name}</h4>
          <p>邮箱: {user.email}</p>
          <p>公司: {user.company.name}</p>
        </div>
      ))}
    </div>
  );
}
```

### POST 请求和数据提交
```jsx
function DataSubmission() {
  const [formData, setFormData] = useState({
    title: '',
    body: '',
    userId: 1
  });
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState(null);
  
  const handleSubmit = async (e) => {
    e.preventDefault();
    setSubmitting(true);
    
    try {
      const response = await fetch('https://jsonplaceholder.typicode.com/posts', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData)
      });
      
      if (!response.ok) {
        throw new Error('提交失败');
      }
      
      const data = await response.json();
      setResult(data);
      setFormData({ title: '', body: '', userId: 1 });
    } catch (err) {
      setResult({ error: err.message });
    } finally {
      setSubmitting(false);
    }
  };
  
  return (
    <div>
      <h2>发表文章</h2>
      <form onSubmit={handleSubmit}>
        <div>
          <input
            type="text"
            placeholder="标题"
            value={formData.title}
            onChange={(e) => setFormData(prev => ({ ...prev, title: e.target.value }))}
            required
          />
        </div>
        <div>
          <textarea
            placeholder="内容"
            value={formData.body}
            onChange={(e) => setFormData(prev => ({ ...prev, body: e.target.value }))}
            required
          />
        </div>
        <button type="submit" disabled={submitting}>
          {submitting ? '提交中...' : '发表'}
        </button>
      </form>
      
      {result && (
        <div style={{ marginTop: '20px', padding: '10px', backgroundColor: '#f8f9fa' }}>
          {result.error ? (
            <p style={{ color: 'red' }}>错误: {result.error}</p>
          ) : (
            <div>
              <h4>发表成功!</h4>
              <p>文章ID: {result.id}</p>
              <p>标题: {result.title}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
```

## 自定义 Hook 封装

### useApi Hook
```jsx
import { useState, useEffect, useCallback } from 'react';

function useApi(url, options = {}) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const fetchData = useCallback(async (customUrl = url, customOptions = {}) => {
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch(customUrl, { ...options, ...customOptions });
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const result = await response.json();
      setData(result);
      return result;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [url, options]);
  
  useEffect(() => {
    if (url) {
      fetchData();
    }
  }, [fetchData]);
  
  return { data, loading, error, refetch: fetchData };
}

// 使用示例
function ApiExample() {
  const { data: posts, loading, error, refetch } = useApi('https://jsonplaceholder.typicode.com/posts');
  
  return (
    <div>
      <h2>文章列表</h2>
      <button onClick={refetch}>刷新</button>
      
      {loading && <p>加载中...</p>}
      {error && <p>错误: {error}</p>}
      {posts && (
        <div>
          {posts.slice(0, 5).map(post => (
            <div key={post.id}>
              <h4>{post.title}</h4>
              <p>{post.body}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
```

## 实时数据和 WebSocket

### WebSocket 连接管理
```jsx
function useWebSocket(url) {
  const [socket, setSocket] = useState(null);
  const [lastMessage, setLastMessage] = useState(null);
  const [connectionStatus, setConnectionStatus] = useState('Disconnected');
  
  useEffect(() => {
    const ws = new WebSocket(url);
    
    ws.onopen = () => {
      setConnectionStatus('Connected');
      setSocket(ws);
    };
    
    ws.onmessage = (event) => {
      setLastMessage(JSON.parse(event.data));
    };
    
    ws.onclose = () => {
      setConnectionStatus('Disconnected');
      setSocket(null);
    };
    
    ws.onerror = () => {
      setConnectionStatus('Error');
    };
    
    return () => {
      ws.close();
    };
  }, [url]);
  
  const sendMessage = useCallback((message) => {
    if (socket && socket.readyState === WebSocket.OPEN) {
      socket.send(JSON.stringify(message));
    }
  }, [socket]);
  
  return { lastMessage, connectionStatus, sendMessage };
}

// 实时聊天组件
function RealTimeChat() {
  const [messages, setMessages] = useState([]);
  const [inputMessage, setInputMessage] = useState('');
  const { lastMessage, connectionStatus, sendMessage } = useWebSocket('ws://localhost:8080');
  
  useEffect(() => {
    if (lastMessage) {
      setMessages(prev => [...prev, lastMessage]);
    }
  }, [lastMessage]);
  
  const handleSend = () => {
    if (inputMessage.trim()) {
      sendMessage({
        text: inputMessage,
        timestamp: new Date().toISOString(),
        user: 'me'
      });
      setInputMessage('');
    }
  };
  
  return (
    <div>
      <h2>实时聊天</h2>
      <p>连接状态: {connectionStatus}</p>
      
      <div style={{ 
        height: '300px', 
        overflowY: 'auto', 
        border: '1px solid #ddd',
        padding: '10px'
      }}>
        {messages.map((msg, index) => (
          <div key={index}>
            <strong>{msg.user}:</strong> {msg.text}
          </div>
        ))}
      </div>
      
      <div>
        <input
          value={inputMessage}
          onChange={(e) => setInputMessage(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSend()}
        />
        <button onClick={handleSend}>发送</button>
      </div>
    </div>
  );
}
```

## 分页和无限滚动

### 分页数据获取
```jsx
function usePagination(baseUrl, itemsPerPage = 10) {
  const [currentPage, setCurrentPage] = useState(1);
  const [data, setData] = useState([]);
  const [totalPages, setTotalPages] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const fetchPage = useCallback(async (page) => {
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${baseUrl}?_page=${page}&_limit=${itemsPerPage}`);
      
      if (!response.ok) {
        throw new Error('获取数据失败');
      }
      
      const result = await response.json();
      const total = parseInt(response.headers.get('X-Total-Count') || '0');
      
      setData(result);
      setTotalPages(Math.ceil(total / itemsPerPage));
      setCurrentPage(page);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [baseUrl, itemsPerPage]);
  
  useEffect(() => {
    fetchPage(1);
  }, [fetchPage]);
  
  return {
    data,
    currentPage,
    totalPages,
    loading,
    error,
    goToPage: fetchPage,
    nextPage: () => currentPage < totalPages && fetchPage(currentPage + 1),
    prevPage: () => currentPage > 1 && fetchPage(currentPage - 1)
  };
}

// 分页组件示例
function PaginatedPosts() {
  const {
    data: posts,
    currentPage,
    totalPages,
    loading,
    error,
    goToPage,
    nextPage,
    prevPage
  } = usePagination('https://jsonplaceholder.typicode.com/posts', 5);
  
  if (loading) return <div>加载中...</div>;
  if (error) return <div>错误: {error}</div>;
  
  return (
    <div>
      <h2>分页文章列表</h2>
      
      {posts.map(post => (
        <div key={post.id} style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd' }}>
          <h4>{post.title}</h4>
          <p>{post.body}</p>
        </div>
      ))}
      
      <div style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
        <button onClick={prevPage} disabled={currentPage === 1}>
          上一页
        </button>
        
        {Array.from({ length: totalPages }, (_, i) => i + 1).map(page => (
          <button
            key={page}
            onClick={() => goToPage(page)}
            style={{
              backgroundColor: page === currentPage ? '#007bff' : '#f8f9fa',
              color: page === currentPage ? 'white' : 'black'
            }}
          >
            {page}
          </button>
        ))}
        
        <button onClick={nextPage} disabled={currentPage === totalPages}>
          下一页
        </button>
      </div>
      
      <p>第 {currentPage} 页，共 {totalPages} 页</p>
    </div>
  );
}
```

### 无限滚动
```jsx
function useInfiniteScroll(fetchMore) {
  const [isFetching, setIsFetching] = useState(false);
  
  useEffect(() => {
    const handleScroll = () => {
      if (window.innerHeight + document.documentElement.scrollTop !== document.documentElement.offsetHeight || isFetching) {
        return;
      }
      setIsFetching(true);
    };
    
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, [isFetching]);
  
  useEffect(() => {
    if (!isFetching) return;
    
    fetchMore().then(() => {
      setIsFetching(false);
    });
  }, [isFetching, fetchMore]);
  
  return [isFetching, setIsFetching];
}

function InfiniteScrollPosts() {
  const [posts, setPosts] = useState([]);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  
  const fetchMorePosts = useCallback(async () => {
    try {
      const response = await fetch(`https://jsonplaceholder.typicode.com/posts?_page=${page}&_limit=10`);
      const newPosts = await response.json();
      
      if (newPosts.length === 0) {
        setHasMore(false);
        return;
      }
      
      setPosts(prev => [...prev, ...newPosts]);
      setPage(prev => prev + 1);
    } catch (error) {
      console.error('获取数据失败:', error);
    }
  }, [page]);
  
  const [isFetching] = useInfiniteScroll(fetchMorePosts);
  
  useEffect(() => {
    fetchMorePosts();
  }, []); // 初始加载
  
  return (
    <div>
      <h2>无限滚动文章列表</h2>
      {posts.map(post => (
        <div key={post.id} style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd' }}>
          <h4>{post.title}</h4>
          <p>{post.body}</p>
        </div>
      ))}
      
      {isFetching && hasMore && <div>加载更多...</div>}
      {!hasMore && <div>没有更多内容了</div>}
    </div>
  );
}
```

## 缓存和离线支持

### 简单缓存实现
```jsx
function useCache() {
  const cache = useRef(new Map());
  
  const get = useCallback((key) => {
    const item = cache.current.get(key);
    if (!item) return null;
    
    // 检查是否过期
    if (Date.now() > item.expiry) {
      cache.current.delete(key);
      return null;
    }
    
    return item.data;
  }, []);
  
  const set = useCallback((key, data, ttl = 5 * 60 * 1000) => { // 默认5分钟过期
    cache.current.set(key, {
      data,
      expiry: Date.now() + ttl
    });
  }, []);
  
  const clear = useCallback(() => {
    cache.current.clear();
  }, []);
  
  return { get, set, clear };
}

function useCachedApi(url) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const { get, set } = useCache();
  
  const fetchData = useCallback(async (forceRefresh = false) => {
    // 先检查缓存
    if (!forceRefresh) {
      const cachedData = get(url);
      if (cachedData) {
        setData(cachedData);
        return;
      }
    }
    
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch(url);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const result = await response.json();
      
      // 保存到缓存
      set(url, result);
      setData(result);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [url, get, set]);
  
  useEffect(() => {
    fetchData();
  }, [fetchData]);
  
  return { data, loading, error, refetch: () => fetchData(true) };
}
```

## 练习任务

1. 创建一个天气应用，支持城市搜索和天气预报显示
2. 实现一个图片搜索应用，支持关键词搜索和无限滚动
3. 制作一个新闻阅读器，支持分类浏览和收藏功能

## 下一步

掌握数据获取后，下一章将学习错误边界与错误处理。