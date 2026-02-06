//! 解析器模块
//! 
//! 提供搜索结果的解析和处理功能

use crate::r#type::{IntermediateSearchResults, SearchResults, SearchOptions};
use crate::error::Result;
use crate::search::resolve_default;

/// 解析器结构体
#[derive(Clone)]
pub struct Resolver {
    pub index: Option<crate::Index>,
    pub result: IntermediateSearchResults,
    pub boostval: i32,
    pub resolved: bool,
}

impl std::fmt::Debug for Resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resolver")
            .field("index", &self.index.as_ref().map(|_| "Index"))
            .field("result", &self.result)
            .field("boostval", &self.boostval)
            .field("resolved", &self.resolved)
            .finish()
    }
}

impl Resolver {
    /// 创建新的解析器
    pub fn new(result: IntermediateSearchResults, index: Option<crate::Index>) -> Self {
        Resolver {
            index,
            result,
            boostval: 0,
            resolved: false,
        }
    }

    /// 从搜索选项创建解析器
    pub fn from_options(options: &SearchOptions, index: &crate::Index) -> Result<Self> {
        let boost = options.boost.unwrap_or(0);
        let result = if let Some(_query) = &options.query {
            // 简化实现：直接返回空结果
            vec![]
        } else {
            vec![]
        };

        Ok(Resolver {
            index: Some(index.clone()),
            result,
            boostval: boost,
            resolved: false,
        })
    }

    /// 设置限制
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        if !self.result.is_empty() {
            let mut final_result: IntermediateSearchResults = Vec::new();
            let mut remaining_limit = limit;

            for ids in &self.result {
                if ids.is_empty() {
                    continue;
                }

                if ids.len() <= remaining_limit {
                    final_result.push(ids.clone());
                    remaining_limit -= ids.len();
                } else {
                    final_result.push(ids[..remaining_limit].to_vec());
                    break;
                }
            }

            self.result = final_result;
        }
        self
    }

    /// 获取解析后的结果
    pub fn get(&mut self) -> SearchResults {
        if !self.resolved {
            self.resolved = true;
            
            if self.result.is_empty() {
                return Vec::new();
            }
            
            // 展平结果
            let mut flattened = Vec::new();
            for array in &self.result {
                flattened.extend_from_slice(array);
            }
            
            flattened
        } else {
            Vec::new()
        }
    }

    /// 交集操作
    pub fn and(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() && !other.is_empty() {
            // 使用兼容的交集函数
            let current = self.result.clone();
            let arrays = vec![current, other];
            
            let simple_arrays: Vec<Vec<u64>> = arrays.into_iter().flatten().collect();
            let intersection_result = crate::intersect::core::intersect_simple(&simple_arrays);
            
            self.result = vec![intersection_result];
        } else if !self.result.is_empty() {
            // 保持不变
        } else if !other.is_empty() {
            self.result = other;
        }
        self
    }

    /// 并集操作
    pub fn or(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() && !other.is_empty() {
            // 合并结果
            let mut combined = self.result.clone();
            combined.extend(other);
            
            // 去重
            let mut seen = std::collections::HashMap::new();
            let mut unique_result = Vec::new();
            
            for array in combined {
                let mut unique_array = Vec::new();
                for &id in &array {
                    if !seen.contains_key(&id) {
                        seen.insert(id, true);
                        unique_array.push(id);
                    }
                }
                if !unique_array.is_empty() {
                    unique_result.push(unique_array);
                }
            }
            
            self.result = unique_result;
        } else if self.result.is_empty() {
            self.result = other;
        }
        self
    }

    /// 差集操作
    pub fn not(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() {
            let current_flat = resolve_default(&self.result, 100, 0);
            let other_flat = resolve_default(&other, 100, 0);
            
            let mut result: SearchResults = Vec::new();
            let mut check = std::collections::HashMap::new();

            for &id in &other_flat {
                check.insert(id, true);
            }

            for &id in &current_flat {
                if !check.contains_key(&id) {
                    result.push(id);
                }
            }

            self.result = vec![result];
        }
        self
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            index: None,
            result: vec![],
            boostval: 0,
            resolved: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_new() {
        let result: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let resolver = Resolver::new(result, None);
        assert_eq!(resolver.result.len(), 1);
        assert_eq!(resolver.result[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_resolver_limit() {
        let result: IntermediateSearchResults = vec![vec![1, 2, 3, 4, 5]];
        let mut resolver = Resolver::new(result, None);
        resolver.limit(3);
        
        let result = resolver.get();
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_resolver_and() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let result2: IntermediateSearchResults = vec![vec![2, 3, 4]];
        
        let mut resolver = Resolver::new(result1, None);
        resolver.and(result2);
        
        let result = resolver.get();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_resolver_or() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let result2: IntermediateSearchResults = vec![vec![4, 5, 6]];
        
        let mut resolver = Resolver::new(result1, None);
        resolver.or(result2);
        
        let result = resolver.get();
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_resolver_not() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3, 4, 5]];
        let result2: IntermediateSearchResults = vec![vec![3, 4]];
        
        let mut resolver = Resolver::new(result1, None);
        resolver.not(result2);
        
        let result = resolver.get();
        assert_eq!(result, vec![1, 2, 5]);
    }
}