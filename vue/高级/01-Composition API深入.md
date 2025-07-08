# Vue 3 Composition API 深入解析

## 1. Composition API 概述

Composition API 是 Vue 3 引入的新特性，提供了一种更灵活的方式来组织组件逻辑。它解决了 Options API 在复杂组件中的一些限制。

### 为什么使用 Composition API？
- **逻辑复用**：更好的代码复用机制
- **类型推断**：更好的 TypeScript 支持
- **逻辑组织**：相关逻辑可以组织在一起
- **性能优化**：更好的 tree-shaking 支持

## 2. setup() 函数

### 基本用法
```vue
<template>
    <div class="user-profile">
        <h2>用户信息</h2>
        <div class="profile-section">
            <div class="avatar">
                <img :src="user.avatar" :alt="user.name">
                <button @click="changeAvatar">更换头像</button>
            </div>
            <div class="info">
                <h3>{{ user.name }}</h3>
                <p>邮箱：{{ user.email }}</p>
                <p>注册时间：{{ formatDate(user.createdAt) }}</p>
                <p>在线状态：{{ user.isOnline ? '在线' : '离线' }}</p>
            </div>
        </div>
        
        <div class="stats-section">
            <div class="stat-item">
                <div class="stat-value">{{ userStats.posts }}</div>
                <div class="stat-label">发布文章</div>
            </div>
            <div class="stat-item">
                <div class="stat-value">{{ userStats.followers }}</div>
                <div class="stat-label">粉丝</div>
            </div>
            <div class="stat-item">
                <div class="stat-value">{{ userStats.following }}</div>
                <div class="stat-label">关注</div>
            </div>
        </div>
        
        <div class="actions-section">
            <button @click="toggleOnlineStatus" 
                    :class="{ active: user.isOnline }">
                {{ user.isOnline ? '设为离线' : '设为在线' }}
            </button>
            <button @click="refreshUserData">刷新数据</button>
            <button @click="exportUserData">导出数据</button>
        </div>
    </div>
</template>

<script>
import { ref, reactive, computed, onMounted, onUnmounted } from 'vue'

export default {
    name: 'UserProfile',
    
    setup() {
        // 响应式数据
        const user = reactive({
            id: 1,
            name: '张三',
            email: 'zhangsan@example.com',
            avatar: 'https://via.placeholder.com/100',
            createdAt: new Date('2023-01-01'),
            isOnline: true
        })
        
        const userStats = reactive({
            posts: 42,
            followers: 1234,
            following: 567
        })
        
        const isLoading = ref(false)
        const error = ref(null)
        
        // 计算属性
        const profileCompleteness = computed(() => {
            let score = 0
            if (user.name) score += 25
            if (user.email) score += 25
            if (user.avatar) score += 25
            if (user.createdAt) score += 25
            return score
        })
        
        // 方法
        const formatDate = (date) => {
            return new Intl.DateTimeFormat('zh-CN', {
                year: 'numeric',
                month: 'long',
                day: 'numeric'
            }).format(date)
        }
        
        const changeAvatar = () => {
            const avatars = [
                'https://via.placeholder.com/100/ff0000',
                'https://via.placeholder.com/100/00ff00',
                'https://via.placeholder.com/100/0000ff',
                'https://via.placeholder.com/100/ffff00'
            ]
            const randomAvatar = avatars[Math.floor(Math.random() * avatars.length)]
            user.avatar = randomAvatar
        }
        
        const toggleOnlineStatus = () => {
            user.isOnline = !user.isOnline
        }
        
        const refreshUserData = async () => {
            isLoading.value = true
            error.value = null
            
            try {
                // 模拟API调用
                await new Promise(resolve => setTimeout(resolve, 1000))
                
                // 更新统计数据
                userStats.posts = Math.floor(Math.random() * 100)
                userStats.followers = Math.floor(Math.random() * 2000)
                userStats.following = Math.floor(Math.random() * 1000)
                
                console.log('用户数据已刷新')
            } catch (err) {
                error.value = '刷新失败，请重试'
                console.error('刷新用户数据失败:', err)
            } finally {
                isLoading.value = false
            }
        }
        
        const exportUserData = () => {
            const data = {
                user: { ...user },
                stats: { ...userStats },
                profileCompleteness: profileCompleteness.value,
                exportedAt: new Date().toISOString()
            }
            
            console.log('导出用户数据:', data)
            // 实际应用中可以下载为 JSON 文件
        }
        
        // 生命周期钩子
        onMounted(() => {
            console.log('UserProfile 组件已挂载')
            refreshUserData()
        })
        
        onUnmounted(() => {
            console.log('UserProfile 组件已卸载')
        })
        
        // 返回要在模板中使用的数据和方法
        return {
            user,
            userStats,
            isLoading,
            error,
            profileCompleteness,
            formatDate,
            changeAvatar,
            toggleOnlineStatus,
            refreshUserData,
            exportUserData
        }
    }
}
</script>

<style scoped>
.user-profile {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
}

.profile-section {
    display: flex;
    gap: 20px;
    margin-bottom: 30px;
    padding: 20px;
    background: #f5f5f5;
    border-radius: 8px;
}

.avatar {
    text-align: center;
}

.avatar img {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    object-fit: cover;
    margin-bottom: 10px;
}

.avatar button {
    padding: 8px 16px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.info h3 {
    margin: 0 0 10px 0;
    color: #333;
}

.info p {
    margin: 5px 0;
    color: #666;
}

.stats-section {
    display: flex;
    gap: 20px;
    margin-bottom: 30px;
}

.stat-item {
    flex: 1;
    text-align: center;
    padding: 20px;
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
}

.stat-value {
    font-size: 24px;
    font-weight: bold;
    color: #42b883;
    margin-bottom: 5px;
}

.stat-label {
    color: #666;
    font-size: 14px;
}

.actions-section {
    display: flex;
    gap: 10px;
}

.actions-section button {
    padding: 12px 24px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: #6c757d;
    color: white;
}

.actions-section button.active {
    background: #28a745;
}

.actions-section button:hover {
    opacity: 0.8;
}
</style>
```

## 3. 响应式 API

### ref() 和 reactive()
```vue
<template>
    <div class="reactive-demo">
        <h2>响应式 API 演示</h2>
        
        <!-- ref 示例 -->
        <div class="section">
            <h3>ref() 示例</h3>
            <p>计数器: {{ count }}</p>
            <p>消息: {{ message }}</p>
            <p>是否显示: {{ isVisible }}</p>
            <button @click="incrementCount">增加计数</button>
            <button @click="updateMessage">更新消息</button>
            <button @click="toggleVisibility">切换显示</button>
        </div>
        
        <!-- reactive 示例 -->
        <div class="section">
            <h3>reactive() 示例</h3>
            <div class="user-form">
                <input v-model="form.name" placeholder="姓名">
                <input v-model="form.email" placeholder="邮箱">
                <input v-model.number="form.age" placeholder="年龄">
                <select v-model="form.city">
                    <option value="">选择城市</option>
                    <option value="beijing">北京</option>
                    <option value="shanghai">上海</option>
                    <option value="guangzhou">广州</option>
                </select>
                <button @click="submitForm">提交</button>
                <button @click="resetForm">重置</button>
            </div>
            <div class="form-preview">
                <h4>表单数据预览：</h4>
                <pre>{{ JSON.stringify(form, null, 2) }}</pre>
            </div>
        </div>
        
        <!-- 复杂对象示例 -->
        <div class="section">
            <h3>复杂对象管理</h3>
            <div class="todo-manager">
                <div class="add-todo">
                    <input v-model="newTodo" placeholder="新任务..." @keyup.enter="addTodo">
                    <button @click="addTodo">添加</button>
                </div>
                <div class="todo-list">
                    <div v-for="todo in todos" :key="todo.id" class="todo-item">
                        <input type="checkbox" v-model="todo.completed">
                        <span :class="{ completed: todo.completed }">{{ todo.text }}</span>
                        <button @click="removeTodo(todo.id)">删除</button>
                    </div>
                </div>
                <div class="todo-stats">
                    <p>总任务: {{ todos.length }}</p>
                    <p>已完成: {{ completedTodos.length }}</p>
                    <p>待完成: {{ pendingTodos.length }}</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref, reactive, computed } from 'vue'

export default {
    name: 'ReactiveDemo',
    
    setup() {
        // ref 示例
        const count = ref(0)
        const message = ref('Hello Vue 3')
        const isVisible = ref(true)
        
        const incrementCount = () => {
            count.value++
        }
        
        const updateMessage = () => {
            const messages = ['Hello Vue 3', '你好 Vue 3', 'Bonjour Vue 3', 'Hola Vue 3']
            const randomIndex = Math.floor(Math.random() * messages.length)
            message.value = messages[randomIndex]
        }
        
        const toggleVisibility = () => {
            isVisible.value = !isVisible.value
        }
        
        // reactive 示例
        const form = reactive({
            name: '',
            email: '',
            age: null,
            city: ''
        })
        
        const submitForm = () => {
            console.log('提交表单:', form)
        }
        
        const resetForm = () => {
            form.name = ''
            form.email = ''
            form.age = null
            form.city = ''
        }
        
        // 复杂对象示例
        const todos = reactive([
            { id: 1, text: '学习 Vue 3', completed: false },
            { id: 2, text: '掌握 Composition API', completed: true },
            { id: 3, text: '构建应用', completed: false }
        ])
        
        const newTodo = ref('')
        let nextId = 4
        
        const addTodo = () => {
            if (newTodo.value.trim()) {
                todos.push({
                    id: nextId++,
                    text: newTodo.value.trim(),
                    completed: false
                })
                newTodo.value = ''
            }
        }
        
        const removeTodo = (id) => {
            const index = todos.findIndex(todo => todo.id === id)
            if (index > -1) {
                todos.splice(index, 1)
            }
        }
        
        // 计算属性
        const completedTodos = computed(() => {
            return todos.filter(todo => todo.completed)
        })
        
        const pendingTodos = computed(() => {
            return todos.filter(todo => !todo.completed)
        })
        
        return {
            // ref 数据
            count,
            message,
            isVisible,
            incrementCount,
            updateMessage,
            toggleVisibility,
            
            // reactive 数据
            form,
            submitForm,
            resetForm,
            
            // 复杂对象
            todos,
            newTodo,
            addTodo,
            removeTodo,
            completedTodos,
            pendingTodos
        }
    }
}
</script>

<style scoped>
.reactive-demo {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.section {
    margin-bottom: 40px;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 8px;
}

.section h3 {
    margin-top: 0;
    color: #333;
}

.user-form {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
    flex-wrap: wrap;
}

.user-form input, .user-form select {
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.user-form button {
    padding: 8px 16px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.form-preview {
    background: #f5f5f5;
    padding: 15px;
    border-radius: 4px;
}

.form-preview pre {
    margin: 0;
    font-size: 14px;
}

.todo-manager {
    max-width: 500px;
}

.add-todo {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}

.add-todo input {
    flex: 1;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.add-todo button {
    padding: 8px 16px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.todo-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-bottom: 1px solid #eee;
}

.todo-item span {
    flex: 1;
}

.todo-item span.completed {
    text-decoration: line-through;
    color: #999;
}

.todo-item button {
    padding: 4px 8px;
    background: #e74c3c;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
}

.todo-stats {
    margin-top: 20px;
    padding: 15px;
    background: #f5f5f5;
    border-radius: 4px;
}

.todo-stats p {
    margin: 5px 0;
    color: #666;
}
</style>
```

## 4. 生命周期钩子

```vue
<template>
    <div class="lifecycle-demo">
        <h2>生命周期钩子演示</h2>
        
        <div class="controls">
            <button @click="toggleTimer">{{ isTimerRunning ? '停止' : '开始' }} 计时器</button>
            <button @click="resetTimer">重置计时器</button>
            <button @click="addLogEntry">添加日志</button>
            <button @click="clearLogs">清空日志</button>
        </div>
        
        <div class="timer-display">
            <div class="timer">{{ formatTime(elapsedTime) }}</div>
            <div class="status">状态: {{ isTimerRunning ? '运行中' : '已停止' }}</div>
        </div>
        
        <div class="logs-container">
            <h3>生命周期日志</h3>
            <div class="log-list">
                <div v-for="log in logs" :key="log.id" class="log-item">
                    <span class="timestamp">{{ formatLogTime(log.timestamp) }}</span>
                    <span class="hook-name">{{ log.hookName }}</span>
                    <span class="description">{{ log.description }}</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { 
    ref, 
    reactive, 
    computed, 
    onBeforeMount, 
    onMounted, 
    onBeforeUpdate, 
    onUpdated, 
    onBeforeUnmount, 
    onUnmounted,
    onErrorCaptured,
    watch,
    nextTick
} from 'vue'

export default {
    name: 'LifecycleDemo',
    
    setup() {
        // 状态管理
        const elapsedTime = ref(0)
        const isTimerRunning = ref(false)
        const logs = reactive([])
        let timerId = null
        let logId = 1
        
        // 添加日志的辅助函数
        const addLog = (hookName, description) => {
            logs.push({
                id: logId++,
                hookName,
                description,
                timestamp: new Date()
            })
        }
        
        // 计时器相关方法
        const startTimer = () => {
            if (!isTimerRunning.value) {
                isTimerRunning.value = true
                timerId = setInterval(() => {
                    elapsedTime.value++
                }, 1000)
                addLog('Timer', '计时器开始运行')
            }
        }
        
        const stopTimer = () => {
            if (isTimerRunning.value) {
                isTimerRunning.value = false
                clearInterval(timerId)
                timerId = null
                addLog('Timer', '计时器停止运行')
            }
        }
        
        const toggleTimer = () => {
            if (isTimerRunning.value) {
                stopTimer()
            } else {
                startTimer()
            }
        }
        
        const resetTimer = () => {
            stopTimer()
            elapsedTime.value = 0
            addLog('Timer', '计时器重置')
        }
        
        const addLogEntry = () => {
            addLog('Manual', '用户手动添加的日志条目')
        }
        
        const clearLogs = () => {
            logs.splice(0, logs.length)
        }
        
        // 格式化时间
        const formatTime = (seconds) => {
            const hours = Math.floor(seconds / 3600)
            const minutes = Math.floor((seconds % 3600) / 60)
            const secs = seconds % 60
            return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
        }
        
        const formatLogTime = (date) => {
            return date.toLocaleTimeString()
        }
        
        // 生命周期钩子
        onBeforeMount(() => {
            console.log('onBeforeMount: 组件挂载前')
            addLog('onBeforeMount', '组件即将挂载到 DOM')
        })
        
        onMounted(() => {
            console.log('onMounted: 组件已挂载')
            addLog('onMounted', '组件已成功挂载到 DOM')
            
            // 在挂载后自动开始计时器
            nextTick(() => {
                startTimer()
            })
        })
        
        onBeforeUpdate(() => {
            console.log('onBeforeUpdate: 组件更新前')
            addLog('onBeforeUpdate', '组件即将更新')
        })
        
        onUpdated(() => {
            console.log('onUpdated: 组件已更新')
            addLog('onUpdated', '组件已完成更新')
        })
        
        onBeforeUnmount(() => {
            console.log('onBeforeUnmount: 组件卸载前')
            addLog('onBeforeUnmount', '组件即将卸载')
            stopTimer() // 清理计时器
        })
        
        onUnmounted(() => {
            console.log('onUnmounted: 组件已卸载')
            // 注意：这个钩子在组件卸载后执行，此时模板已不可用
        })
        
        onErrorCaptured((error, instance, errorInfo) => {
            console.error('onErrorCaptured:', error, instance, errorInfo)
            addLog('onErrorCaptured', `捕获错误: ${error.message}`)
            return false // 阻止错误继续传播
        })
        
        // 监听器
        watch(elapsedTime, (newTime, oldTime) => {
            if (newTime > 0 && newTime % 10 === 0) {
                addLog('Watch', `计时器达到 ${newTime} 秒`)
            }
        })
        
        watch(isTimerRunning, (newStatus, oldStatus) => {
            addLog('Watch', `计时器状态变化: ${oldStatus} → ${newStatus}`)
        })
        
        // 监听日志数组变化
        watch(logs, (newLogs) => {
            if (newLogs.length > 100) {
                // 保持日志数量在合理范围内
                newLogs.splice(0, newLogs.length - 100)
            }
        }, { deep: true })
        
        return {
            elapsedTime,
            isTimerRunning,
            logs,
            toggleTimer,
            resetTimer,
            addLogEntry,
            clearLogs,
            formatTime,
            formatLogTime
        }
    }
}
</script>

<style scoped>
.lifecycle-demo {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.controls {
    display: flex;
    gap: 10px;
    margin-bottom: 30px;
}

.controls button {
    padding: 10px 20px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.controls button:hover {
    background: #369870;
}

.timer-display {
    text-align: center;
    margin-bottom: 30px;
    padding: 30px;
    background: #f5f5f5;
    border-radius: 8px;
}

.timer {
    font-size: 48px;
    font-weight: bold;
    color: #333;
    margin-bottom: 10px;
    font-family: 'Courier New', monospace;
}

.status {
    font-size: 18px;
    color: #666;
}

.logs-container {
    background: #f9f9f9;
    padding: 20px;
    border-radius: 8px;
}

.logs-container h3 {
    margin-top: 0;
    color: #333;
}

.log-list {
    max-height: 400px;
    overflow-y: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: white;
}

.log-item {
    display: flex;
    padding: 10px;
    border-bottom: 1px solid #eee;
    font-family: 'Courier New', monospace;
    font-size: 14px;
}

.log-item:last-child {
    border-bottom: none;
}

.timestamp {
    color: #666;
    width: 100px;
    flex-shrink: 0;
}

.hook-name {
    color: #42b883;
    font-weight: bold;
    width: 150px;
    flex-shrink: 0;
}

.description {
    color: #333;
    flex: 1;
}
</style>
```

## 5. 自定义 Hooks

```vue
<template>
    <div class="custom-hooks-demo">
        <h2>自定义 Hooks 演示</h2>
        
        <!-- 计数器 Hook -->
        <div class="section">
            <h3>计数器 Hook</h3>
            <div class="counter-controls">
                <button @click="decrement">-</button>
                <span class="count">{{ count }}</span>
                <button @click="increment">+</button>
                <button @click="reset">重置</button>
            </div>
            <p>步长: {{ step }}</p>
            <input v-model.number="step" type="number" min="1" max="10">
        </div>
        
        <!-- 本地存储 Hook -->
        <div class="section">
            <h3>本地存储 Hook</h3>
            <div class="storage-demo">
                <input v-model="storedValue" placeholder="输入内容，会自动保存">
                <button @click="clearStorage">清空存储</button>
                <p>存储的值: {{ storedValue }}</p>
            </div>
        </div>
        
        <!-- 网络请求 Hook -->
        <div class="section">
            <h3>网络请求 Hook</h3>
            <div class="api-demo">
                <button @click="fetchUser" :disabled="loading">
                    {{ loading ? '加载中...' : '获取用户信息' }}
                </button>
                <div v-if="error" class="error">{{ error }}</div>
                <div v-if="userData" class="user-data">
                    <h4>用户信息:</h4>
                    <pre>{{ JSON.stringify(userData, null, 2) }}</pre>
                </div>
            </div>
        </div>
        
        <!-- 鼠标位置 Hook -->
        <div class="section">
            <h3>鼠标位置 Hook</h3>
            <div class="mouse-tracker">
                <p>鼠标位置: X: {{ mouseX }}, Y: {{ mouseY }}</p>
                <div class="mouse-indicator" 
                     :style="{ left: mouseX + 'px', top: mouseY + 'px' }"></div>
            </div>
        </div>
        
        <!-- 窗口大小 Hook -->
        <div class="section">
            <h3>窗口大小 Hook</h3>
            <div class="window-size">
                <p>窗口大小: {{ windowWidth }} x {{ windowHeight }}</p>
                <p>屏幕类型: {{ screenType }}</p>
            </div>
        </div>
    </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

// 计数器 Hook
function useCounter(initialValue = 0, step = 1) {
    const count = ref(initialValue)
    const stepRef = ref(step)
    
    const increment = () => {
        count.value += stepRef.value
    }
    
    const decrement = () => {
        count.value -= stepRef.value
    }
    
    const reset = () => {
        count.value = initialValue
    }
    
    return {
        count,
        step: stepRef,
        increment,
        decrement,
        reset
    }
}

// 本地存储 Hook
function useLocalStorage(key, defaultValue = '') {
    const storedValue = ref(defaultValue)
    
    // 初始化时从本地存储读取
    onMounted(() => {
        const item = localStorage.getItem(key)
        if (item) {
            storedValue.value = JSON.parse(item)
        }
    })
    
    // 监听值变化，自动保存到本地存储
    watch(storedValue, (newValue) => {
        localStorage.setItem(key, JSON.stringify(newValue))
    }, { deep: true })
    
    const clearStorage = () => {
        localStorage.removeItem(key)
        storedValue.value = defaultValue
    }
    
    return {
        storedValue,
        clearStorage
    }
}

// 网络请求 Hook
function useApi() {
    const loading = ref(false)
    const error = ref(null)
    const data = ref(null)
    
    const fetchData = async (url, options = {}) => {
        loading.value = true
        error.value = null
        
        try {
            // 模拟网络请求
            await new Promise(resolve => setTimeout(resolve, 1000))
            
            // 模拟数据
            const mockData = {
                id: Math.floor(Math.random() * 1000),
                name: '用户' + Math.floor(Math.random() * 100),
                email: `user${Math.floor(Math.random() * 100)}@example.com`,
                createdAt: new Date().toISOString()
            }
            
            data.value = mockData
        } catch (err) {
            error.value = err.message
        } finally {
            loading.value = false
        }
    }
    
    return {
        loading,
        error,
        data,
        fetchData
    }
}

// 鼠标位置 Hook
function useMouse() {
    const mouseX = ref(0)
    const mouseY = ref(0)
    
    const updateMouse = (e) => {
        mouseX.value = e.clientX
        mouseY.value = e.clientY
    }
    
    onMounted(() => {
        window.addEventListener('mousemove', updateMouse)
    })
    
    onUnmounted(() => {
        window.removeEventListener('mousemove', updateMouse)
    })
    
    return {
        mouseX,
        mouseY
    }
}

// 窗口大小 Hook
function useWindowSize() {
    const windowWidth = ref(window.innerWidth)
    const windowHeight = ref(window.innerHeight)
    
    const screenType = computed(() => {
        if (windowWidth.value < 768) return 'mobile'
        if (windowWidth.value < 1024) return 'tablet'
        return 'desktop'
    })
    
    const updateSize = () => {
        windowWidth.value = window.innerWidth
        windowHeight.value = window.innerHeight
    }
    
    onMounted(() => {
        window.addEventListener('resize', updateSize)
    })
    
    onUnmounted(() => {
        window.removeEventListener('resize', updateSize)
    })
    
    return {
        windowWidth,
        windowHeight,
        screenType
    }
}

export default {
    name: 'CustomHooksDemo',
    
    setup() {
        // 使用计数器 Hook
        const { count, step, increment, decrement, reset } = useCounter(0, 1)
        
        // 使用本地存储 Hook
        const { storedValue, clearStorage } = useLocalStorage('demo-input', '')
        
        // 使用网络请求 Hook
        const { loading, error, data: userData, fetchData } = useApi()
        
        const fetchUser = () => {
            fetchData('/api/user')
        }
        
        // 使用鼠标位置 Hook
        const { mouseX, mouseY } = useMouse()
        
        // 使用窗口大小 Hook
        const { windowWidth, windowHeight, screenType } = useWindowSize()
        
        return {
            // 计数器
            count,
            step,
            increment,
            decrement,
            reset,
            
            // 本地存储
            storedValue,
            clearStorage,
            
            // 网络请求
            loading,
            error,
            userData,
            fetchUser,
            
            // 鼠标位置
            mouseX,
            mouseY,
            
            // 窗口大小
            windowWidth,
            windowHeight,
            screenType
        }
    }
}
</script>

<style scoped>
.custom-hooks-demo {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.section {
    margin-bottom: 30px;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 8px;
}

.section h3 {
    margin-top: 0;
    color: #333;
}

.counter-controls {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 15px;
}

.counter-controls button {
    padding: 8px 16px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.count {
    font-size: 24px;
    font-weight: bold;
    color: #333;
    margin: 0 10px;
}

.storage-demo input {
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    margin-right: 10px;
    width: 200px;
}

.storage-demo button {
    padding: 8px 16px;
    background: #e74c3c;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.api-demo button {
    padding: 10px 20px;
    background: #42b883;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 15px;
}

.api-demo button:disabled {
    background: #ccc;
    cursor: not-allowed;
}

.error {
    color: #e74c3c;
    margin-bottom: 15px;
}

.user-data {
    background: #f5f5f5;
    padding: 15px;
    border-radius: 4px;
}

.user-data pre {
    margin: 0;
    font-size: 14px;
}

.mouse-tracker {
    position: relative;
    height: 200px;
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow: hidden;
}

.mouse-indicator {
    position: absolute;
    width: 10px;
    height: 10px;
    background: #42b883;
    border-radius: 50%;
    pointer-events: none;
    transform: translate(-50%, -50%);
}

.window-size p {
    margin: 5px 0;
    color: #666;
}
</style>
```

## 6. 学习要点

### Composition API 优势
- **逻辑复用**：通过自定义 Hooks 实现
- **更好的组织**：相关逻辑可以组织在一起
- **类型支持**：更好的 TypeScript 支持
- **性能优化**：更好的 tree-shaking

### 响应式原理
- `ref()` 用于基本类型数据
- `reactive()` 用于对象和数组
- 计算属性和监听器的使用

### 生命周期
- 组合式 API 中的生命周期钩子
- 清理副作用的重要性
- 错误处理和监听

### 自定义 Hooks
- 逻辑复用的最佳实践
- 封装复杂的状态逻辑
- 提高代码的可维护性

## 7. 练习建议

1. 创建一个数据分页的 Hook
2. 实现一个表单验证的 Hook
3. 创建一个主题切换的 Hook
4. 实现一个实时数据订阅的 Hook

下一节将学习 Vue 3 的高级特性，包括 Teleport、Suspense 等。