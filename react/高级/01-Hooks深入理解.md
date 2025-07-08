# 01 - Hooks 深入理解

## Hook 规则与原理

### Hook 使用规则
1. 只能在函数组件或自定义 Hook 中调用
2. 只能在组件的顶层调用，不能在循环、条件或嵌套函数中调用
3. 必须保证每次渲染时 Hook 的调用顺序一致

### useState 深入
```jsx
import React, { useState, useCallback } from 'react';

function StateDeepDive() {
  const [count, setCount] = useState(0);
  const [user, setUser] = useState({ name: '', email: '' });
  
  // 函数式更新
  const incrementCount = useCallback(() => {
    setCount(prevCount => prevCount + 1);
  }, []);
  
  // 惰性初始化
  const [expensiveValue, setExpensiveValue] = useState(() => {
    console.log('只在初始化时执行');
    return computeExpensiveValue();
  });
  
  // 对象状态的正确更新方式
  const updateUser = useCallback((field, value) => {
    setUser(prevUser => ({
      ...prevUser,
      [field]: value
    }));
  }, []);
  
  return (
    <div>
      <p>计数：{count}</p>
      <button onClick={incrementCount}>增加</button>
      
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
    </div>
  );
}

function computeExpensiveValue() {
  // 模拟耗时计算
  return Date.now();
}
```

## useEffect 深入

### 副作用的完整生命周期
```jsx
import React, { useState, useEffect, useRef } from 'react';

function EffectDeepDive() {
  const [count, setCount] = useState(0);
  const [user, setUser] = useState(null);
  const intervalRef = useRef(null);
  
  // 1. 没有依赖数组 - 每次重新渲染都执行
  useEffect(() => {
    console.log('组件重新渲染了');
  });
  
  // 2. 空依赖数组 - 只在挂载时执行
  useEffect(() => {
    console.log('组件挂载了');
    
    // 清理函数
    return () => {
      console.log('组件即将卸载');
    };
  }, []);
  
  // 3. 有依赖的 effect
  useEffect(() => {
    console.log('count 改变了:', count);
    
    // 设置定时器
    intervalRef.current = setInterval(() => {
      console.log('定时器执行，当前count:', count);
    }, 1000);
    
    // 清理函数
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [count]);
  
  // 4. 异步数据获取
  useEffect(() => {
    let cancelled = false;
    
    async function fetchUser() {
      try {
        const response = await fetch('/api/user');
        const userData = await response.json();
        
        if (!cancelled) {
          setUser(userData);
        }
      } catch (error) {
        if (!cancelled) {
          console.error('获取用户失败:', error);
        }
      }
    }
    
    fetchUser();
    
    return () => {
      cancelled = true;
    };
  }, []);
  
  return (
    <div>
      <p>计数：{count}</p>
      <button onClick={() => setCount(count + 1)}>增加</button>
      {user && <p>用户：{user.name}</p>}
    </div>
  );
}
```

## useCallback 和 useMemo

### 性能优化的正确使用
```jsx
import React, { useState, useCallback, useMemo, memo } from 'react';

// 子组件使用 memo 优化
const ExpensiveChildComponent = memo(({ data, onProcess }) => {
  console.log('ExpensiveChildComponent 重新渲染');
  
  return (
    <div>
      <p>数据：{data}</p>
      <button onClick={onProcess}>处理数据</button>
    </div>
  );
});

function OptimizationExample() {
  const [count, setCount] = useState(0);
  const [items, setItems] = useState([1, 2, 3, 4, 5]);
  const [filter, setFilter] = useState('');
  
  // 使用 useCallback 缓存函数
  const handleProcess = useCallback(() => {
    console.log('处理数据');
  }, []);
  
  // 使用 useMemo 缓存计算结果
  const filteredItems = useMemo(() => {
    console.log('计算过滤后的数据');
    return items.filter(item => 
      item.toString().includes(filter)
    );
  }, [items, filter]);
  
  // 使用 useMemo 缓存复杂计算
  const expensiveValue = useMemo(() => {
    console.log('执行耗时计算');
    return items.reduce((sum, item) => sum + item * item, 0);
  }, [items]);
  
  return (
    <div>
      <p>计数：{count}</p>
      <button onClick={() => setCount(count + 1)}>增加计数</button>
      
      <input
        placeholder="过滤条件"
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
      />
      
      <p>过滤后的数据：{filteredItems.join(', ')}</p>
      <p>计算结果：{expensiveValue}</p>
      
      <ExpensiveChildComponent 
        data={expensiveValue} 
        onProcess={handleProcess}
      />
    </div>
  );
}
```

## useRef 高级用法

### 访问 DOM 和存储可变值
```jsx
import React, { useRef, useEffect, useState, useCallback } from 'react';

function RefAdvancedExample() {
  const [count, setCount] = useState(0);
  const inputRef = useRef(null);
  const countRef = useRef(0);
  const timerRef = useRef(null);
  
  // 聚焦输入框
  const focusInput = useCallback(() => {
    inputRef.current?.focus();
  }, []);
  
  // 获取前一个值
  const usePrevious = (value) => {
    const ref = useRef();
    useEffect(() => {
      ref.current = value;
    });
    return ref.current;
  };
  
  const prevCount = usePrevious(count);
  
  // 防抖功能
  const useDebounce = (callback, delay) => {
    const timeoutRef = useRef(null);
    
    return useCallback((...args) => {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = setTimeout(() => {
        callback(...args);
      }, delay);
    }, [callback, delay]);
  };
  
  const debouncedSearch = useDebounce((value) => {
    console.log('搜索:', value);
  }, 300);
  
  // 计时器控制
  const startTimer = () => {
    if (timerRef.current) return;
    
    timerRef.current = setInterval(() => {
      countRef.current += 1;
      setCount(countRef.current);
    }, 1000);
  };
  
  const stopTimer = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
  };
  
  useEffect(() => {
    countRef.current = count;
  }, [count]);
  
  useEffect(() => {
    return () => {
      stopTimer();
    };
  }, []);
  
  return (
    <div>
      <input ref={inputRef} placeholder="输入内容" />
      <button onClick={focusInput}>聚焦输入框</button>
      
      <p>当前计数：{count}</p>
      <p>前一个计数：{prevCount}</p>
      
      <button onClick={startTimer}>开始计时</button>
      <button onClick={stopTimer}>停止计时</button>
      
      <input
        placeholder="防抖搜索"
        onChange={(e) => debouncedSearch(e.target.value)}
      />
    </div>
  );
}
```

## 自定义 Hook

### 封装逻辑复用
```jsx
import { useState, useEffect, useCallback } from 'react';

// 自定义 Hook：本地存储
function useLocalStorage(key, initialValue) {
  const [storedValue, setStoredValue] = useState(() => {
    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      console.error('Error reading localStorage:', error);
      return initialValue;
    }
  });
  
  const setValue = useCallback((value) => {
    try {
      const valueToStore = value instanceof Function ? value(storedValue) : value;
      setStoredValue(valueToStore);
      window.localStorage.setItem(key, JSON.stringify(valueToStore));
    } catch (error) {
      console.error('Error setting localStorage:', error);
    }
  }, [key, storedValue]);
  
  return [storedValue, setValue];
}

// 自定义 Hook：数据获取
function useApi(url) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const result = await response.json();
      setData(result);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [url]);
  
  useEffect(() => {
    fetchData();
  }, [fetchData]);
  
  return { data, loading, error, refetch: fetchData };
}

// 自定义 Hook：表单处理
function useForm(initialValues, validationRules) {
  const [values, setValues] = useState(initialValues);
  const [errors, setErrors] = useState({});
  const [touched, setTouched] = useState({});
  
  const handleChange = useCallback((name, value) => {
    setValues(prev => ({ ...prev, [name]: value }));
    
    if (touched[name]) {
      validateField(name, value);
    }
  }, [touched]);
  
  const handleBlur = useCallback((name) => {
    setTouched(prev => ({ ...prev, [name]: true }));
    validateField(name, values[name]);
  }, [values]);
  
  const validateField = useCallback((name, value) => {
    const rule = validationRules[name];
    if (rule) {
      const error = rule(value);
      setErrors(prev => ({ ...prev, [name]: error }));
      return !error;
    }
    return true;
  }, [validationRules]);
  
  const validateForm = useCallback(() => {
    const newErrors = {};
    let isValid = true;
    
    Object.keys(validationRules).forEach(name => {
      const error = validationRules[name](values[name]);
      if (error) {
        newErrors[name] = error;
        isValid = false;
      }
    });
    
    setErrors(newErrors);
    return isValid;
  }, [values, validationRules]);
  
  const reset = useCallback(() => {
    setValues(initialValues);
    setErrors({});
    setTouched({});
  }, [initialValues]);
  
  return {
    values,
    errors,
    touched,
    handleChange,
    handleBlur,
    validateForm,
    reset
  };
}

// 使用自定义 Hook 的示例
function CustomHookExample() {
  const [name, setName] = useLocalStorage('userName', '');
  const { data, loading, error } = useApi('/api/users');
  
  const validationRules = {
    email: (value) => {
      if (!value) return '邮箱不能为空';
      if (!/\S+@\S+\.\S+/.test(value)) return '邮箱格式不正确';
      return null;
    },
    password: (value) => {
      if (!value) return '密码不能为空';
      if (value.length < 6) return '密码至少6位';
      return null;
    }
  };
  
  const {
    values,
    errors,
    touched,
    handleChange,
    handleBlur,
    validateForm,
    reset
  } = useForm(
    { email: '', password: '' },
    validationRules
  );
  
  const handleSubmit = (e) => {
    e.preventDefault();
    if (validateForm()) {
      console.log('表单提交:', values);
    }
  };
  
  return (
    <div>
      <h3>自定义 Hook 示例</h3>
      
      <div>
        <input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="姓名（本地存储）"
        />
      </div>
      
      <div>
        <h4>API 数据：</h4>
        {loading && <p>加载中...</p>}
        {error && <p>错误：{error}</p>}
        {data && <p>数据：{JSON.stringify(data)}</p>}
      </div>
      
      <form onSubmit={handleSubmit}>
        <div>
          <input
            type="email"
            placeholder="邮箱"
            value={values.email}
            onChange={(e) => handleChange('email', e.target.value)}
            onBlur={() => handleBlur('email')}
          />
          {touched.email && errors.email && (
            <span style={{ color: 'red' }}>{errors.email}</span>
          )}
        </div>
        
        <div>
          <input
            type="password"
            placeholder="密码"
            value={values.password}
            onChange={(e) => handleChange('password', e.target.value)}
            onBlur={() => handleBlur('password')}
          />
          {touched.password && errors.password && (
            <span style={{ color: 'red' }}>{errors.password}</span>
          )}
        </div>
        
        <button type="submit">提交</button>
        <button type="button" onClick={reset}>重置</button>
      </form>
    </div>
  );
}
```

## 练习任务

1. 创建一个 useCounter 自定义 Hook
2. 实现一个 useToggle 自定义 Hook
3. 封装一个 useAsync 自定义 Hook 处理异步操作

## 下一步

掌握 Hooks 后，下一章将学习 Context API 和状态管理。