// 02_集合类型.rs
// Rust标准库集合类型详解

/*
Rust标准库提供了丰富的集合类型，主要包括：
1. Vec<T> - 动态数组，可变长度的序列
2. HashMap<K, V> - 哈希映射，键值对存储
3. HashSet<T> - 哈希集合，唯一元素集合
4. BTreeMap<K, V> - 有序映射，基于B树
5. BTreeSet<T> - 有序集合，基于B树
6. LinkedList<T> - 双向链表
7. VecDeque<T> - 双端队列
8. BinaryHeap<T> - 二叉堆，优先队列

这些集合类型为不同的使用场景提供了最优的性能特征。
*/

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    println!("=== Rust标准库集合类型 ===");
    
    // 1. Vec<T> - 动态数组
    println!("\n1. Vec<T> - 动态数组：");
    vec_examples();
    
    // 2. HashMap<K, V> - 哈希映射
    println!("\n2. HashMap<K, V> - 哈希映射：");
    hashmap_examples();
    
    // 3. HashSet<T> - 哈希集合
    println!("\n3. HashSet<T> - 哈希集合：");
    hashset_examples();
    
    // 4. BTreeMap<K, V> - 有序映射
    println!("\n4. BTreeMap<K, V> - 有序映射：");
    btreemap_examples();
    
    // 5. BTreeSet<T> - 有序集合
    println!("\n5. BTreeSet<T> - 有序集合：");
    btreeset_examples();
    
    // 6. LinkedList<T> - 双向链表
    println!("\n6. LinkedList<T> - 双向链表：");
    linkedlist_examples();
    
    // 7. VecDeque<T> - 双端队列
    println!("\n7. VecDeque<T> - 双端队列：");
    vecdeque_examples();
    
    // 8. BinaryHeap<T> - 二叉堆
    println!("\n8. BinaryHeap<T> - 二叉堆：");
    binaryheap_examples();
    
    // 9. 性能对比示例
    println!("\n9. 性能对比：");
    performance_comparison();
    
    println!("\n=== 集合类型学习完成 ===");
}

// Vec<T> 示例
fn vec_examples() {
    // 创建Vec
    let mut fruits = vec!["苹果", "香蕉", "橙子"];
    println!("初始水果: {:?}", fruits);
    
    // 添加元素
    fruits.push("葡萄");
    fruits.extend(["草莓", "芒果"]);
    println!("添加后: {:?}", fruits);
    
    // 访问元素
    if let Some(first) = fruits.get(0) {
        println!("第一个水果: {}", first);
    }
    
    // 删除元素
    let removed = fruits.remove(1); // 删除索引1的元素
    println!("删除的元素: {}", removed);
    println!("删除后: {:?}", fruits);
    
    // 迭代器操作
    let uppercase_fruits: Vec<String> = fruits
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("大写水果: {:?}", uppercase_fruits);
    
    // 筛选操作
    let long_names: Vec<&str> = fruits
        .iter()
        .filter(|name| name.len() > 2)
        .copied()
        .collect();
    println!("长名称水果: {:?}", long_names);
}

// HashMap<K, V> 示例
fn hashmap_examples() {
    // 创建HashMap
    let mut student_scores = HashMap::new();
    student_scores.insert("张三", 85);
    student_scores.insert("李四", 92);
    student_scores.insert("王五", 78);
    
    println!("学生成绩: {:?}", student_scores);
    
    // 访问值
    match student_scores.get("张三") {
        Some(score) => println!("张三的成绩: {}", score),
        None => println!("找不到张三的成绩"),
    }
    
    // 更新值
    student_scores.insert("张三", 88); // 更新张三的成绩
    *student_scores.entry("赵六").or_insert(0) = 95; // 插入新学生
    
    println!("更新后的成绩: {:?}", student_scores);
    
    // 统计字符出现次数
    let text = "hello world";
    let mut char_count = HashMap::new();
    for ch in text.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    println!("字符统计: {:?}", char_count);
    
    // 遍历HashMap
    for (student, score) in &student_scores {
        println!("{}: {}", student, score);
    }
}

// HashSet<T> 示例
fn hashset_examples() {
    // 创建HashSet
    let mut programming_languages = HashSet::new();
    programming_languages.insert("Rust");
    programming_languages.insert("Python");
    programming_languages.insert("JavaScript");
    programming_languages.insert("Go");
    
    println!("编程语言: {:?}", programming_languages);
    
    // 检查是否包含
    if programming_languages.contains("Rust") {
        println!("包含Rust语言");
    }
    
    // 集合运算
    let mut web_languages = HashSet::new();
    web_languages.insert("JavaScript");
    web_languages.insert("TypeScript");
    web_languages.insert("Python");
    
    println!("Web语言: {:?}", web_languages);
    
    // 交集
    let intersection: HashSet<_> = programming_languages
        .intersection(&web_languages)
        .collect();
    println!("交集: {:?}", intersection);
    
    // 并集
    let union: HashSet<_> = programming_languages
        .union(&web_languages)
        .collect();
    println!("并集: {:?}", union);
    
    // 差集
    let difference: HashSet<_> = programming_languages
        .difference(&web_languages)
        .collect();
    println!("差集: {:?}", difference);
    
    // 去重操作
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique_numbers: HashSet<_> = numbers.into_iter().collect();
    println!("去重后的数字: {:?}", unique_numbers);
}

// BTreeMap<K, V> 示例
fn btreemap_examples() {
    // 创建BTreeMap (自动排序)
    let mut grade_book = BTreeMap::new();
    grade_book.insert("数学", 88);
    grade_book.insert("英语", 92);
    grade_book.insert("物理", 85);
    grade_book.insert("化学", 90);
    
    println!("成绩册 (按科目名排序): {:?}", grade_book);
    
    // 范围查询
    let range_query: BTreeMap<_, _> = grade_book
        .range("数学".."英语")
        .collect();
    println!("范围查询 (数学到英语): {:?}", range_query);
    
    // 获取第一个和最后一个元素
    if let Some((first_key, first_value)) = grade_book.first_key_value() {
        println!("第一个科目: {} - {}", first_key, first_value);
    }
    
    if let Some((last_key, last_value)) = grade_book.last_key_value() {
        println!("最后一个科目: {} - {}", last_key, last_value);
    }
}

// BTreeSet<T> 示例
fn btreeset_examples() {
    // 创建BTreeSet (自动排序)
    let mut scores = BTreeSet::new();
    scores.insert(85);
    scores.insert(92);
    scores.insert(78);
    scores.insert(90);
    scores.insert(88);
    
    println!("有序分数: {:?}", scores);
    
    // 范围操作
    let high_scores: BTreeSet<_> = scores.range(85..).collect();
    println!("高分 (>=85): {:?}", high_scores);
    
    // 分割操作
    let (lower, upper) = scores.split_off(&87);
    println!("低分组 (<87): {:?}", lower);
    println!("高分组 (>=87): {:?}", upper);
}

// LinkedList<T> 示例
fn linkedlist_examples() {
    // 创建LinkedList
    let mut task_list = LinkedList::new();
    task_list.push_back("写代码");
    task_list.push_back("测试");
    task_list.push_back("部署");
    
    println!("任务列表: {:?}", task_list);
    
    // 在前面添加任务
    task_list.push_front("需求分析");
    task_list.push_front("设计");
    
    println!("完整任务列表: {:?}", task_list);
    
    // 从两端移除
    if let Some(first_task) = task_list.pop_front() {
        println!("完成任务: {}", first_task);
    }
    
    if let Some(last_task) = task_list.pop_back() {
        println!("推迟任务: {}", last_task);
    }
    
    println!("剩余任务: {:?}", task_list);
}

// VecDeque<T> 示例
fn vecdeque_examples() {
    // 创建VecDeque
    let mut buffer = VecDeque::new();
    
    // 在两端添加元素
    buffer.push_back("第二个");
    buffer.push_back("第三个");
    buffer.push_front("第一个");
    
    println!("缓冲区: {:?}", buffer);
    
    // 访问元素
    if let Some(front) = buffer.front() {
        println!("前端元素: {}", front);
    }
    
    if let Some(back) = buffer.back() {
        println!("后端元素: {}", back);
    }
    
    // 旋转操作
    buffer.rotate_left(1);
    println!("左旋转后: {:?}", buffer);
    
    buffer.rotate_right(2);
    println!("右旋转后: {:?}", buffer);
}

// BinaryHeap<T> 示例
fn binaryheap_examples() {
    // 创建最大堆
    let mut max_heap = BinaryHeap::new();
    max_heap.push(5);
    max_heap.push(2);
    max_heap.push(8);
    max_heap.push(1);
    max_heap.push(9);
    
    println!("最大堆: {:?}", max_heap);
    
    // 弹出最大元素
    while let Some(max) = max_heap.pop() {
        println!("弹出最大值: {}", max);
    }
    
    // 创建最小堆 (使用Reverse包装)
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(8));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(9));
    
    println!("最小堆: {:?}", min_heap);
    
    // 弹出最小元素
    while let Some(Reverse(min)) = min_heap.pop() {
        println!("弹出最小值: {}", min);
    }
    
    // 优先任务队列示例
    #[derive(Debug, Eq, PartialEq)]
    struct Task {
        priority: u32,
        name: String,
    }
    
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }
    
    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    
    let mut task_queue = BinaryHeap::new();
    task_queue.push(Task { priority: 1, name: "低优先级任务".to_string() });
    task_queue.push(Task { priority: 5, name: "高优先级任务".to_string() });
    task_queue.push(Task { priority: 3, name: "中优先级任务".to_string() });
    
    println!("任务队列: {:?}", task_queue);
    
    while let Some(task) = task_queue.pop() {
        println!("执行任务: {} (优先级: {})", task.name, task.priority);
    }
}

// 性能对比示例
fn performance_comparison() {
    println!("不同集合类型的特点:");
    println!("Vec<T>:      顺序访问O(1)，随机访问O(1)，插入/删除O(n)");
    println!("HashMap<K,V>: 查找O(1)，插入O(1)，删除O(1)，无序");
    println!("BTreeMap<K,V>: 查找O(log n)，插入O(log n)，删除O(log n)，有序");
    println!("HashSet<T>:  查找O(1)，插入O(1)，删除O(1)，无序");
    println!("BTreeSet<T>: 查找O(log n)，插入O(log n)，删除O(log n)，有序");
    println!("LinkedList<T>: 顺序访问O(n)，头尾插入/删除O(1)");
    println!("VecDeque<T>: 头尾访问O(1)，头尾插入/删除O(1)");
    println!("BinaryHeap<T>: 查找最大值O(1)，插入O(log n)，删除最大值O(log n)");
    
    // 选择合适的集合类型
    println!("\n选择建议:");
    println!("- 需要随机访问和顺序遍历：Vec<T>");
    println!("- 需要快速查找键值对：HashMap<K,V>");
    println!("- 需要有序的键值对：BTreeMap<K,V>");
    println!("- 需要去重和集合运算：HashSet<T>");
    println!("- 需要有序的唯一元素：BTreeSet<T>");
    println!("- 需要频繁在两端插入/删除：VecDeque<T>");
    println!("- 需要优先队列：BinaryHeap<T>");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
        assert_eq!(map.get("nonexistent"), None);
    }
    
    #[test]
    fn test_hashset_operations() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1); // 重复元素
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
    }
}