//! 搜索模块
//! 
//! 提供搜索功能

use crate::r#type::{IntermediateSearchResults, SearchResults, SearchOptions};
use crate::intersect::compat::intersect_compatible;
use crate::error::Result;

/// 搜索结果结构体
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub results: SearchResults,
    pub total: usize,
    pub query: String,
}

/// 搜索配置
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub resolution: usize,
    pub limit: usize,
    pub offset: usize,
    pub suggest: bool,
    pub boost: i32,
    pub resolve: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            resolution: 9,
            limit: 10,
            offset: 0,
            suggest: false,
            boost: 0,
            resolve: true,
        }
    }
}

/// 执行搜索
pub fn search(_index: &crate::Index, options: &SearchOptions) -> Result<SearchResult> {
    // 简化实现：返回空结果
    Ok(SearchResult {
        results: Vec::new(),
        total: 0,
        query: options.query.clone().unwrap_or_default(),
    })
}

/// 执行搜索（简化版本）
pub fn perform_search(
    results: Vec<IntermediateSearchResults>,
    config: &SearchConfig,
) -> IntermediateSearchResults {
    if results.is_empty() {
        return Vec::new();
    }
    
    // 将所有结果展平为单层结构
    let flattened: IntermediateSearchResults = results
        .into_iter()
        .flatten()
        .collect();
    
    // 使用兼容的交集函数
    intersect_compatible(
        &flattened,
        config.resolution,
        config.limit,
        config.offset,
        config.suggest,
        config.boost,
        config.resolve,
    )
}

/// 默认解析函数（兼容函数）
pub fn resolve_default(
    results: &IntermediateSearchResults,
    limit: usize,
    offset: usize,
) -> Vec<u64> {
    if results.is_empty() {
        return Vec::new();
    }
    
    // 展平结果
    let mut flattened = Vec::new();
    for array in results {
        flattened.extend_from_slice(array);
    }
    
    // 应用限制和偏移
    if offset > 0 {
        if offset >= flattened.len() {
            return Vec::new();
        }
        flattened.drain(0..offset);
    }
    
    if limit > 0 && limit < flattened.len() {
        flattened.truncate(limit);
    }
    
    flattened
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_default() {
        let results = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        
        let resolved = resolve_default(&results, 10, 0);
        assert_eq!(resolved, vec![1, 2, 3, 4, 5, 6]);
        
        let resolved_limited = resolve_default(&results, 3, 0);
        assert_eq!(resolved_limited, vec![1, 2, 3]);
        
        let resolved_offset = resolve_default(&results, 10, 2);
        assert_eq!(resolved_offset, vec![3, 4, 5, 6]);
    }
}