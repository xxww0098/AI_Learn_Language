# Hashbrown 0.15.4 - Rust 高性能哈希表完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心特性](#核心特性)
- [基本用法](#基本用法)
- [高级特性](#高级特性)
- [性能优化](#性能优化)
- [与标准库对比](#与标准库对比)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Hashbrown 是 Google SwissTable 哈希表算法的 Rust 实现，提供了比标准库更高性能的哈希表。它也是 Rust 标准库 HashMap 的底层实现。

### 核心特性
- **SwissTable 算法**: Google 开发的高性能哈希表算法
- **内存紧凑**: 更好的缓存友好性和内存效率
- **高性能**: 在大多数场景下比传统哈希表更快
- **API 兼容**: 与标准库 HashMap 高度兼容
- **自定义分配器**: 支持自定义内存分配器

### 版本信息
- **当前版本**: 0.15.4
- **发布时间**: 2025-06-07
- **下载次数**: 719,748,054+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
hashbrown = "0.15.4"
```

### 基本示例

```rust
use hashbrown::{HashMap, HashSet};

fn main() {
    // 创建 HashMap
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    println!("Value: {:?}", map.get("key1"));
    
    // 创建 HashSet
    let mut set = HashSet::new();
    set.insert("item1");
    set.insert("item2");
    
    println!("Contains item1: {}", set.contains("item1"));
    
    // 使用宏创建
    let numbers: HashMap<i32, &str> = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
    ].iter().cloned().collect();
    
    println!("Numbers: {:?}", numbers);
}
```

## 核心特性

### SwissTable 算法原理

```rust
use hashbrown::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn demonstrate_swisstable() {
    let mut map = HashMap::new();
    
    // SwissTable 使用开放寻址法和 SIMD 指令优化
    for i in 0..1000 {
        map.insert(i, format!("value_{}", i));
    }
    
    // 密集的内存布局提供更好的缓存性能
    println!("Map size: {}", map.len());
    println!("Capacity: {}", map.capacity());
    
    // 演示哈希值分布
    let mut hasher = DefaultHasher::new();
    "test_key".hash(&mut hasher);
    println!("Hash value: {}", hasher.finish());
}
```

### 内存布局优化

```rust
use hashbrown::HashMap;
use std::mem;

fn memory_layout_demo() {
    let map: HashMap<i32, String> = HashMap::new();
    
    println!("HashMap size: {} bytes", mem::size_of_val(&map));
    println!("Entry size: {} bytes", mem::size_of::<(i32, String)>());
    
    // hashbrown 的内存布局更紧凑
    let mut map = HashMap::with_capacity(100);
    
    for i in 0..50 {
        map.insert(i, format!("value_{}", i));
    }
    
    println!("Used capacity: {}/{}", map.len(), map.capacity());
    println!("Load factor: {:.2}", map.len() as f64 / map.capacity() as f64);
}
```

## 基本用法

### HashMap 操作

```rust
use hashbrown::HashMap;

fn hashmap_operations() {
    let mut map = HashMap::new();
    
    // 插入
    map.insert("name", "张三");
    map.insert("age", "25");
    map.insert("city", "北京");
    
    // 查询
    if let Some(name) = map.get("name") {
        println!("姓名: {}", name);
    }
    
    // 更新
    map.insert("age", "26");  // 覆盖原值
    
    // 条件插入
    map.entry("country").or_insert("中国");
    map.entry("age").or_insert("30");  // 不会覆盖已存在的值
    
    // 删除
    if let Some(removed) = map.remove("city") {
        println!("删除的城市: {}", removed);
    }
    
    // 遍历
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // 清空
    map.clear();
    println!("清空后大小: {}", map.len());
}
```

### HashSet 操作

```rust
use hashbrown::HashSet;

fn hashset_operations() {
    let mut set1 = HashSet::new();
    set1.insert("apple");
    set1.insert("banana");
    set1.insert("orange");
    
    let mut set2 = HashSet::new();
    set2.insert("banana");
    set2.insert("grape");
    set2.insert("apple");
    
    // 检查成员
    println!("Contains banana: {}", set1.contains("banana"));
    
    // 集合运算
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);
    
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);
    
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("差集: {:?}", difference);
    
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).collect();
    println!("对称差集: {:?}", symmetric_difference);
    
    // 子集检查
    let subset: HashSet<_> = ["apple"].iter().cloned().collect();
    println!("Is subset: {}", subset.is_subset(&set1));
}
```

### 自定义哈希函数

```rust
use hashbrown::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};

// 自定义哈希器
struct SimpleHasher(u64);

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
}

impl Default for SimpleHasher {
    fn default() -> Self {
        SimpleHasher(0)
    }
}

type SimpleBuildHasher = BuildHasherDefault<SimpleHasher>;

fn custom_hasher() {
    let mut map: HashMap<String, i32, SimpleBuildHasher> = 
        HashMap::with_hasher(SimpleBuildHasher::default());
    
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    
    println!("Custom hasher map: {:?}", map);
}
```

## 高级特性

### 原始条目API

```rust
use hashbrown::HashMap;
use hashbrown::hash_map::RawEntryMut;

fn raw_entry_api() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    // 使用原始条目 API 进行底层操作
    let hash = hashbrown::hash_map::make_hash(&map, &"key1");
    
    match map.raw_entry_mut().from_hash(hash, |k| k == &"key1") {
        RawEntryMut::Occupied(entry) => {
            println!("Found key: {:?}, value: {:?}", entry.key(), entry.get());
            *entry.get_mut() = "new_value1";
        }
        RawEntryMut::Vacant(_) => {
            println!("Key not found");
        }
    }
    
    // 使用自定义相等函数
    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"key1") {
        RawEntryMut::Occupied(entry) => {
            println!("Direct access: {:?}", entry.get());
        }
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}
```

### 自定义分配器

```rust
use hashbrown::HashMap;

#[cfg(feature = "allocator-api2")]
fn custom_allocator() {
    use allocator_api2::alloc::Global;
    
    // 使用全局分配器
    let mut map: HashMap<i32, String, _, Global> = 
        HashMap::new_in(Global);
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    println!("Custom allocator map: {:?}", map);
}

#[cfg(not(feature = "allocator-api2"))]
fn custom_allocator() {
    println!("自定义分配器需要 allocator-api2 特性");
}
```

### 高效的批量操作

```rust
use hashbrown::HashMap;

fn batch_operations() {
    let mut map = HashMap::new();
    
    // 批量插入
    let data = vec![
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3"),
    ];
    
    map.extend(data);
    
    // 批量查询
    let keys = ["key1", "key2", "key4"];
    let results: Vec<_> = keys.iter()
        .filter_map(|&key| map.get(key).map(|v| (key, v)))
        .collect();
    
    println!("Batch query results: {:?}", results);
    
    // 批量删除
    let keys_to_remove = ["key1", "key3"];
    for key in &keys_to_remove {
        map.remove(key);
    }
    
    println!("After batch removal: {:?}", map);
    
    // 批量更新
    let updates = [("key2", "updated_value2"), ("key5", "value5")];
    for (key, value) in &updates {
        map.insert(*key, *value);
    }
    
    println!("After batch update: {:?}", map);
}
```

### 内存预分配

```rust
use hashbrown::HashMap;

fn memory_preallocation() {
    // 预分配容量避免重新分配
    let mut map = HashMap::with_capacity(1000);
    
    println!("Initial capacity: {}", map.capacity());
    
    // 插入数据不会触发重新分配
    for i in 0..500 {
        map.insert(i, format!("value_{}", i));
    }
    
    println!("After 500 insertions - capacity: {}, len: {}", 
             map.capacity(), map.len());
    
    // 继续插入
    for i in 500..800 {
        map.insert(i, format!("value_{}", i));
    }
    
    println!("After 800 insertions - capacity: {}, len: {}", 
             map.capacity(), map.len());
    
    // 收缩容量
    map.shrink_to_fit();
    println!("After shrink_to_fit - capacity: {}, len: {}", 
             map.capacity(), map.len());
    
    // 保留指定容量
    map.shrink_to(600);
    println!("After shrink_to(600) - capacity: {}, len: {}", 
             map.capacity(), map.len());
}
```

## 性能优化

### 哈希函数选择

```rust
use hashbrown::HashMap;
use std::hash::BuildHasherDefault;

// 使用不同的哈希函数
fn hash_function_comparison() {
    // 默认哈希函数 (SipHash)
    let mut map1 = HashMap::new();
    
    // FxHash (更快但密码学不安全)
    #[cfg(feature = "default-hasher")]
    let mut map2: HashMap<String, i32, _> = HashMap::default();
    
    let data: Vec<_> = (0..1000).map(|i| (format!("key_{}", i), i)).collect();
    
    // 测试插入性能
    let start = std::time::Instant::now();
    for (key, value) in &data {
        map1.insert(key.clone(), *value);
    }
    println!("Default hasher insert time: {:?}", start.elapsed());
    
    #[cfg(feature = "default-hasher")]
    {
        let start = std::time::Instant::now();
        for (key, value) in &data {
            map2.insert(key.clone(), *value);
        }
        println!("FxHash insert time: {:?}", start.elapsed());
    }
}
```

### 缓存友好的遍历

```rust
use hashbrown::HashMap;

fn cache_friendly_traversal() {
    let mut map = HashMap::new();
    
    // 插入大量数据
    for i in 0..10000 {
        map.insert(i, i * 2);
    }
    
    // hashbrown 的内存布局使遍历更缓存友好
    let start = std::time::Instant::now();
    let mut sum = 0;
    
    for (key, value) in &map {
        sum += key + value;
    }
    
    println!("Traversal time: {:?}, sum: {}", start.elapsed(), sum);
    
    // 按插入顺序遍历 (使用 IndexMap 如果需要)
    let keys: Vec<_> = map.keys().collect();
    println!("First 10 keys: {:?}", &keys[..10.min(keys.len())]);
}
```

### 内存使用优化

```rust
use hashbrown::HashMap;

fn memory_usage_optimization() {
    // 对于小集合，考虑使用 Vec 或 BTreeMap
    let small_data = [("a", 1), ("b", 2), ("c", 3)];
    
    // HashMap 有固定开销
    let mut hashmap = HashMap::new();
    for (k, v) in &small_data {
        hashmap.insert(*k, *v);
    }
    
    // Vec 对小数据更内存友好
    let vec_data: Vec<_> = small_data.to_vec();
    
    println!("HashMap overhead for {} items", hashmap.len());
    println!("Vector linear search for small data might be faster");
    
    // 对于大集合，HashMap 更高效
    let mut large_map = HashMap::with_capacity(10000);
    for i in 0..10000 {
        large_map.insert(i, i * 2);
    }
    
    println!("Large map size: {}, capacity: {}", 
             large_map.len(), large_map.capacity());
}
```

## 与标准库对比

### 性能基准测试

```rust
use hashbrown::HashMap as HashbrownMap;
use std::collections::HashMap as StdHashMap;
use std::time::Instant;

fn performance_benchmark() {
    let data: Vec<_> = (0..100000).map(|i| (i, format!("value_{}", i))).collect();
    
    // Hashbrown 插入测试
    let start = Instant::now();
    let mut hashbrown_map = HashbrownMap::new();
    for (key, value) in &data {
        hashbrown_map.insert(*key, value.clone());
    }
    let hashbrown_insert_time = start.elapsed();
    
    // 标准库 HashMap 插入测试
    let start = Instant::now();
    let mut std_map = StdHashMap::new();
    for (key, value) in &data {
        std_map.insert(*key, value.clone());
    }
    let std_insert_time = start.elapsed();
    
    println!("Hashbrown insert time: {:?}", hashbrown_insert_time);
    println!("Std HashMap insert time: {:?}", std_insert_time);
    
    // 查询测试
    let keys: Vec<_> = (0..100000).step_by(10).collect();
    
    let start = Instant::now();
    let mut hashbrown_sum = 0;
    for &key in &keys {
        if let Some(value) = hashbrown_map.get(&key) {
            hashbrown_sum += value.len();
        }
    }
    let hashbrown_lookup_time = start.elapsed();
    
    let start = Instant::now();
    let mut std_sum = 0;
    for &key in &keys {
        if let Some(value) = std_map.get(&key) {
            std_sum += value.len();
        }
    }
    let std_lookup_time = start.elapsed();
    
    println!("Hashbrown lookup time: {:?}", hashbrown_lookup_time);
    println!("Std HashMap lookup time: {:?}", std_lookup_time);
    
    assert_eq!(hashbrown_sum, std_sum);
}
```

### API 兼容性

```rust
use hashbrown::HashMap as HashbrownMap;
use std::collections::HashMap as StdHashMap;

fn api_compatibility() {
    // 大部分 API 完全兼容
    let mut hb_map = HashbrownMap::new();
    let mut std_map = StdHashMap::new();
    
    // 相同的插入语法
    hb_map.insert("key", "value");
    std_map.insert("key", "value");
    
    // 相同的查询语法
    assert_eq!(hb_map.get("key"), std_map.get("key"));
    
    // 相同的迭代语法
    let hb_pairs: Vec<_> = hb_map.iter().collect();
    let std_pairs: Vec<_> = std_map.iter().collect();
    
    println!("API compatibility: {}", hb_pairs == std_pairs);
    
    // hashbrown 特有的功能
    println!("Hashbrown capacity: {}", hb_map.capacity());
    
    // 可以直接替换标准库 HashMap
    type MyHashMap<K, V> = HashbrownMap<K, V>;
    let mut my_map: MyHashMap<i32, String> = MyHashMap::new();
    my_map.insert(1, "one".to_string());
}
```

## 实战案例

### 缓存系统

```rust
use hashbrown::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

struct LRUCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    max_size: usize,
    default_ttl: Duration,
}

impl<K, V> LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new(max_size: usize, default_ttl: Duration) -> Self {
        LRUCache {
            data: HashMap::with_capacity(max_size),
            max_size,
            default_ttl,
        }
    }
    
    fn get(&mut self, key: &K) -> Option<V> {
        self.cleanup_expired();
        
        if let Some(entry) = self.data.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value.clone());
            } else {
                self.data.remove(key);
            }
        }
        None
    }
    
    fn put(&mut self, key: K, value: V) {
        self.cleanup_expired();
        
        if self.data.len() >= self.max_size {
            // 简单的清理策略：删除过期项或随机项
            let keys_to_remove: Vec<_> = self.data.keys().take(self.max_size / 4).cloned().collect();
            for k in keys_to_remove {
                self.data.remove(&k);
            }
        }
        
        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + self.default_ttl,
        };
        
        self.data.insert(key, entry);
    }
    
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, entry| entry.expires_at > now);
    }
    
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn cache_system_example() {
    let mut cache = LRUCache::new(100, Duration::from_secs(60));
    
    // 添加缓存项
    cache.put("user_123", "John Doe");
    cache.put("user_456", "Jane Smith");
    
    // 查询缓存
    if let Some(user) = cache.get(&"user_123") {
        println!("Found user: {}", user);
    }
    
    println!("Cache size: {}", cache.size());
}
```

### 索引构建

```rust
use hashbrown::{HashMap, HashSet};

struct Document {
    id: u32,
    title: String,
    content: String,
}

struct SearchIndex {
    // 词汇到文档ID的倒排索引
    word_to_docs: HashMap<String, HashSet<u32>>,
    // 文档ID到文档的正排索引
    doc_store: HashMap<u32, Document>,
}

impl SearchIndex {
    fn new() -> Self {
        SearchIndex {
            word_to_docs: HashMap::new(),
            doc_store: HashMap::new(),
        }
    }
    
    fn add_document(&mut self, doc: Document) {
        let doc_id = doc.id;
        
        // 分词（简单实现）
        let words = self.tokenize(&format!("{} {}", doc.title, doc.content));
        
        // 构建倒排索引
        for word in words {
            self.word_to_docs.entry(word)
                .or_insert_with(HashSet::new)
                .insert(doc_id);
        }
        
        // 存储文档
        self.doc_store.insert(doc_id, doc);
    }
    
    fn search(&self, query: &str) -> Vec<&Document> {
        let query_words = self.tokenize(query);
        
        if query_words.is_empty() {
            return Vec::new();
        }
        
        // 查找包含所有查询词的文档
        let mut result_docs: Option<HashSet<u32>> = None;
        
        for word in query_words {
            if let Some(doc_ids) = self.word_to_docs.get(&word) {
                match result_docs {
                    None => result_docs = Some(doc_ids.clone()),
                    Some(ref mut docs) => {
                        docs.retain(|id| doc_ids.contains(id));
                    }
                }
            } else {
                return Vec::new(); // 如果任何词都没找到，返回空结果
            }
        }
        
        // 返回文档
        result_docs.unwrap_or_default()
            .iter()
            .filter_map(|id| self.doc_store.get(id))
            .collect()
    }
    
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
    
    fn get_stats(&self) -> (usize, usize) {
        (self.doc_store.len(), self.word_to_docs.len())
    }
}

fn search_index_example() {
    let mut index = SearchIndex::new();
    
    // 添加文档
    index.add_document(Document {
        id: 1,
        title: "Rust编程语言".to_string(),
        content: "Rust是一种系统编程语言".to_string(),
    });
    
    index.add_document(Document {
        id: 2,
        title: "Python教程".to_string(),
        content: "Python是一种高级编程语言".to_string(),
    });
    
    index.add_document(Document {
        id: 3,
        title: "编程语言对比".to_string(),
        content: "Rust和Python都是现代编程语言".to_string(),
    });
    
    // 搜索
    let results = index.search("编程语言");
    println!("搜索'编程语言'的结果:");
    for doc in results {
        println!("  {}: {}", doc.id, doc.title);
    }
    
    let (doc_count, word_count) = index.get_stats();
    println!("索引统计: {} 文档, {} 词汇", doc_count, word_count);
}
```

### 图算法

```rust
use hashbrown::{HashMap, HashSet};

struct Graph {
    // 邻接表表示
    adjacency: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, from: u32, to: u32) {
        self.adjacency.entry(from).or_insert_with(HashSet::new).insert(to);
        self.adjacency.entry(to).or_insert_with(HashSet::new); // 确保节点存在
    }
    
    fn add_undirected_edge(&mut self, a: u32, b: u32) {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }
    
    fn get_neighbors(&self, node: u32) -> Option<&HashSet<u32>> {
        self.adjacency.get(&node)
    }
    
    fn bfs(&self, start: u32, target: u32) -> Option<Vec<u32>> {
        if start == target {
            return Some(vec![start]);
        }
        
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut parent = HashMap::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.get_neighbors(current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        parent.insert(neighbor, current);
                        queue.push_back(neighbor);
                        
                        if neighbor == target {
                            // 重构路径
                            let mut path = vec![target];
                            let mut current = target;
                            
                            while let Some(&p) = parent.get(&current) {
                                path.push(p);
                                current = p;
                            }
                            
                            path.reverse();
                            return Some(path);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn connected_components(&self) -> Vec<HashSet<u32>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        
        for &node in self.adjacency.keys() {
            if !visited.contains(&node) {
                let mut component = HashSet::new();
                self.dfs(node, &mut visited, &mut component);
                components.push(component);
            }
        }
        
        components
    }
    
    fn dfs(&self, node: u32, visited: &mut HashSet<u32>, component: &mut HashSet<u32>) {
        visited.insert(node);
        component.insert(node);
        
        if let Some(neighbors) = self.get_neighbors(node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs(neighbor, visited, component);
                }
            }
        }
    }
    
    fn node_count(&self) -> usize {
        self.adjacency.len()
    }
    
    fn edge_count(&self) -> usize {
        self.adjacency.values().map(|set| set.len()).sum::<usize>() / 2
    }
}

fn graph_algorithm_example() {
    let mut graph = Graph::new();
    
    // 构建一个简单的图
    graph.add_undirected_edge(1, 2);
    graph.add_undirected_edge(2, 3);
    graph.add_undirected_edge(3, 4);
    graph.add_undirected_edge(1, 4);
    graph.add_undirected_edge(5, 6);
    
    println!("图统计: {} 节点, {} 边", graph.node_count(), graph.edge_count());
    
    // 寻找路径
    if let Some(path) = graph.bfs(1, 4) {
        println!("从1到4的路径: {:?}", path);
    }
    
    // 查找连通分量
    let components = graph.connected_components();
    println!("连通分量数: {}", components.len());
    for (i, component) in components.iter().enumerate() {
        println!("分量 {}: {:?}", i + 1, component);
    }
}
```

## 最佳实践

### 1. 容量预分配

```rust
use hashbrown::HashMap;

fn capacity_preallocation() {
    // 好的做法：预分配已知大小的容量
    let expected_size = 1000;
    let mut map = HashMap::with_capacity(expected_size);
    
    for i in 0..expected_size {
        map.insert(i, format!("value_{}", i));
    }
    
    println!("No reallocations occurred: capacity {} >= len {}", 
             map.capacity(), map.len());
    
    // 避免的做法：让HashMap自动增长
    let mut bad_map = HashMap::new();
    for i in 0..expected_size {
        bad_map.insert(i, format!("value_{}", i));
        // 这会导致多次重新分配
    }
}
```

### 2. 选择合适的哈希函数

```rust
use hashbrown::HashMap;

fn choose_hasher() {
    // 对于安全敏感的应用，使用默认哈希函数
    let secure_map: HashMap<String, String> = HashMap::new();
    
    // 对于性能关键且不需要安全性的应用，考虑其他哈希函数
    #[cfg(feature = "default-hasher")]
    let fast_map: HashMap<String, String> = HashMap::default();
    
    // 对于特定的键类型，可能有更优化的哈希函数
    let int_map: HashMap<u64, String> = HashMap::new();
    
    println!("选择合适的哈希函数很重要");
}
```

### 3. 内存使用优化

```rust
use hashbrown::{HashMap, HashSet};

fn memory_optimization() {
    // 对于小集合，考虑其他数据结构
    let small_set = ["a", "b", "c"];
    
    // Vec + linear search 可能更高效
    let vec_search = |target: &str| small_set.iter().any(|&x| x == target);
    
    // HashMap 有固定开销
    let mut hash_set = HashSet::new();
    for &item in &small_set {
        hash_set.insert(item);
    }
    
    println!("Vec search for 'b': {}", vec_search("b"));
    println!("HashSet search for 'b': {}", hash_set.contains("b"));
    
    // 对于大集合，HashMap 更高效
    let mut large_set = HashSet::with_capacity(10000);
    for i in 0..10000 {
        large_set.insert(i);
    }
    
    println!("Large set contains 5000: {}", large_set.contains(&5000));
}
```

### 4. 批量操作优化

```rust
use hashbrown::HashMap;

fn batch_operations_optimization() {
    let mut map = HashMap::with_capacity(1000);
    
    // 好的做法：批量插入
    let data: Vec<_> = (0..1000).map(|i| (i, format!("value_{}", i))).collect();
    map.extend(data);
    
    // 好的做法：批量检查
    let keys_to_check = [1, 10, 100, 1000];
    let existing_keys: Vec<_> = keys_to_check.iter()
        .filter(|&&key| map.contains_key(&key))
        .collect();
    
    println!("Existing keys: {:?}", existing_keys);
    
    // 好的做法：预分配 collect 容器
    let values: HashMap<_, _> = map.iter()
        .filter(|(&k, _)| k % 2 == 0)
        .map(|(&k, v)| (k, v.clone()))
        .collect();
    
    println!("Even keys count: {}", values.len());
}
```

## 总结

Hashbrown 是一个高性能的哈希表实现，提供了比标准库更好的性能特征。通过本教程，您应该能够：

1. 理解 SwissTable 算法的优势
2. 正确使用 hashbrown 的各种 API
3. 选择合适的哈希函数和配置
4. 实现高效的缓存、索引和图算法
5. 优化内存使用和性能

关键要点：
- Hashbrown 在大多数场景下比标准库 HashMap 更快
- 预分配容量可以避免重新分配的开销
- 选择合适的哈希函数对性能很重要
- 对于小集合，线性搜索可能更高效
- 批量操作通常比单个操作更高效

Hashbrown 已经成为 Rust 标准库的一部分，掌握它的使用将帮助您构建更高效的 Rust 应用程序。