use crate::r#type::{SearchOptions, SearchResults, IntermediateSearchResults};
use crate::index::Index;
use crate::error::Result;

pub struct Resolver {
    pub index: Option<Index>,
    pub result: IntermediateSearchResults,
    pub boostval: i32,
    pub resolved: bool,
}

impl Resolver {
    pub fn new(result: IntermediateSearchResults, index: Option<Index>) -> Self {
        Resolver {
            index,
            result,
            boostval: 0,
            resolved: false,
        }
    }

    pub fn from_options(options: &SearchOptions, index: &Index) -> Result<Self> {
        let boost = options.boost.unwrap_or(0);
        let result = if let Some(_query) = &options.query {
            let search_result = crate::search::search(index, options)?;
            vec![search_result.results]
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
                    if remaining_limit == 0 {
                        break;
                    }
                } else {
                    final_result.push(ids[..remaining_limit].to_vec());
                    break;
                }
            }

            self.result = final_result;
        }
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        if !self.result.is_empty() {
            let mut final_result: IntermediateSearchResults = Vec::new();
            let mut remaining_offset = offset;

            for ids in &self.result {
                if ids.is_empty() {
                    continue;
                }

                if ids.len() <= remaining_offset {
                    remaining_offset -= ids.len();
                } else {
                    final_result.push(ids[remaining_offset..].to_vec());
                    remaining_offset = 0;
                }
            }

            self.result = final_result;
        }
        self
    }

    pub fn boost(&mut self, boost: i32) -> &mut Self {
        self.boostval += boost;
        self
    }

    pub fn resolve(&mut self, limit: Option<usize>, offset: Option<usize>, _enrich: bool) -> Result<SearchResults> {
        if self.result.is_empty() {
            return Ok(vec![]);
        }

        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let final_result = crate::search::resolve_default(&self.result, limit, offset);

        self.resolved = true;
        self.index = None;
        self.result = vec![];

        Ok(final_result)
    }

    pub fn execute(&mut self) -> Result<SearchResults> {
        if self.resolved {
            return Ok(vec![]);
        }

        let result = crate::search::resolve_default(&self.result, 100, 0);

        self.resolved = true;
        self.index = None;
        self.result = vec![];

        Ok(result)
    }

    pub fn and(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() && !other.is_empty() {
            let resolution = 9;
            let result = crate::intersect::intersect(&other, resolution, 100, 0, false, self.boostval, true);
            self.result = vec![result];
        } else {
            self.result = vec![];
        }
        self
    }

    pub fn or(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() && !other.is_empty() {
            let result = crate::intersect::union(&other, 100, 0, true, self.boostval);
            self.result = vec![result];
        } else if self.result.is_empty() {
            self.result = other;
        }
        self
    }

    pub fn xor(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() && !other.is_empty() {
            let result = crate::intersect::union(&other, 100, 0, true, self.boostval);
            self.result = vec![result];
        } else if self.result.is_empty() {
            self.result = other;
        }
        self
    }

    pub fn not(&mut self, other: IntermediateSearchResults) -> &mut Self {
        if !self.result.is_empty() {
            let current = crate::search::resolve_default(&self.result, 100, 0);
            let other_flat = crate::search::resolve_default(&other, 100, 0);
            
            let mut result: SearchResults = Vec::new();
            let mut check = std::collections::HashMap::new();

            for &id in &other_flat {
                check.insert(id, true);
            }

            for &id in &current {
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
    use crate::r#type::IndexOptions;

    #[test]
    fn test_resolver_new() {
        let result: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let resolver = Resolver::new(result, None);
        assert_eq!(resolver.result.len(), 1);
        assert_eq!(resolver.result[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_resolver_limit() {
        let result: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let mut resolver = Resolver::new(result, None);
        resolver.limit(4);
        assert_eq!(resolver.result.len(), 2);
        assert_eq!(resolver.result[0], vec![1, 2, 3]);
        assert_eq!(resolver.result[1], vec![4]);
    }

    #[test]
    fn test_resolver_offset() {
        let result: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let mut resolver = Resolver::new(result, None);
        resolver.offset(2);
        assert_eq!(resolver.result.len(), 2);
        assert_eq!(resolver.result[0], vec![3]);
        assert_eq!(resolver.result[1], vec![4, 5]);
    }

    #[test]
    fn test_resolver_boost() {
        let result: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let mut resolver = Resolver::new(result, None);
        resolver.boost(10);
        assert_eq!(resolver.boostval, 10);
    }

    #[test]
    fn test_resolver_resolve() {
        let result: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
        ];
        let mut resolver = Resolver::new(result, None);
        let final_result = resolver.resolve(Some(10), Some(0), false).unwrap();
        assert_eq!(final_result, vec![1, 2, 3, 4, 5]);
        assert!(resolver.resolved);
    }

    #[test]
    fn test_resolver_execute() {
        let result: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
        ];
        let mut resolver = Resolver::new(result, None);
        let final_result = resolver.execute().unwrap();
        assert_eq!(final_result, vec![1, 2, 3, 4, 5]);
        assert!(resolver.resolved);
    }

    #[test]
    fn test_resolver_and() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3, 4]];
        let result2: IntermediateSearchResults = vec![vec![2, 3, 4, 5]];
        let mut resolver = Resolver::new(result1, None);
        resolver.and(result2);
        let final_result = resolver.execute().unwrap();
        assert_eq!(final_result, vec![2, 3, 4]);
    }

    #[test]
    fn test_resolver_or() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3]];
        let result2: IntermediateSearchResults = vec![vec![3, 4, 5]];
        let mut resolver = Resolver::new(result1, None);
        resolver.or(result2);
        let final_result = resolver.execute().unwrap();
        assert_eq!(final_result.len(), 5);
    }

    #[test]
    fn test_resolver_not() {
        let result1: IntermediateSearchResults = vec![vec![1, 2, 3, 4, 5]];
        let result2: IntermediateSearchResults = vec![vec![2, 4]];
        let mut resolver = Resolver::new(result1, None);
        resolver.not(result2);
        let final_result = resolver.execute().unwrap();
        assert_eq!(final_result, vec![1, 3, 5]);
    }

    #[test]
    fn test_resolver_from_options() {
        let index = Index::new(IndexOptions::default()).unwrap();
        let options = SearchOptions {
            query: Some("test".to_string()),
            limit: Some(10),
            offset: Some(0),
            ..Default::default()
        };
        let resolver = Resolver::from_options(&options, &index).unwrap();
        assert_eq!(resolver.boostval, 0);
    }

    #[test]
    fn test_resolver_default() {
        let resolver = Resolver::default();
        assert!(resolver.result.is_empty());
        assert!(resolver.index.is_none());
        assert_eq!(resolver.boostval, 0);
        assert!(!resolver.resolved);
    }
}
