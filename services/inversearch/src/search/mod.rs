use crate::r#type::{SearchOptions, SearchResults, IntermediateSearchResults};
use crate::index::Index;
use crate::error::Result;
use std::collections::HashMap;

pub struct SearchResult {
    pub results: SearchResults,
    pub enriched: bool,
}

pub fn search(index: &Index, options: &SearchOptions) -> Result<SearchResult> {
    let query = options.query.as_deref().unwrap_or("");
    let limit = options.limit.unwrap_or(100);
    let offset = options.offset.unwrap_or(0);
    let context = options.context.unwrap_or(index.depth > 0);
    let suggest = options.suggest.unwrap_or(false);
    let resolve = options.resolve.unwrap_or(true);
    let resolution = options.resolution.unwrap_or(if context { index.resolution_ctx } else { index.resolution });

    let query_terms = index.encoder.encode(query)?;
    let length = query_terms.len();

    if length == 0 {
        return Ok(SearchResult {
            results: vec![],
            enriched: false,
        });
    }

    if length == 1 {
        return single_term_query(index, &query_terms[0], "", limit, offset, resolve);
    }

    if length == 2 && context && !suggest {
        return single_term_query(index, &query_terms[1], &query_terms[0], limit, offset, resolve);
    }

    let mut result: Vec<IntermediateSearchResults> = Vec::new();
    let mut dupes: HashMap<String, bool> = HashMap::new();
    let mut index_ptr = 0;
    let mut keyword = "";

    if context {
        keyword = &query_terms[0];
        index_ptr = 1;
    }

    for idx in index_ptr..length {
        let term = &query_terms[idx];

        if term.is_empty() || dupes.contains_key(term) {
            continue;
        }

        dupes.insert(term.to_string(), true);

        let arr = get_array(index, term, keyword);

        if let Some(arr) = arr {
            let processed = add_result(arr, &result, suggest, resolution);
            if let Some(processed) = processed {
                result = processed;
                break;
            }
        }

        if !keyword.is_empty() {
            if !suggest || result.is_empty() {
                keyword = term;
            }
        }
    }

    if suggest && !keyword.is_empty() && result.is_empty() && index_ptr == length - 1 {
        let mut dupes: HashMap<String, bool> = HashMap::new();
        let mut result: Vec<IntermediateSearchResults> = Vec::new();
        let keyword = "";

        for idx in 0..length {
            let term = &query_terms[idx];

            if term.is_empty() || dupes.contains_key(term) {
                continue;
            }

            dupes.insert(term.to_string(), true);

            let arr = get_array(index, term, keyword);

            if let Some(arr) = arr {
                let processed = add_result(arr, &result, suggest, resolution);
                if let Some(processed) = processed {
                    result = processed;
                    break;
                }
            }
        }
    }

    let final_results = return_result(result, resolution, limit, offset, suggest, resolve);

    Ok(SearchResult {
        results: final_results,
        enriched: false,
    })
}

fn single_term_query(
    index: &Index,
    term: &str,
    keyword: &str,
    limit: usize,
    offset: usize,
    resolve: bool,
) -> Result<SearchResult> {
    let arr = get_array(index, term, keyword);

    if let Some(arr) = arr {
        if !arr.is_empty() {
            let results = if resolve {
                resolve_default(&arr, limit, offset)
            } else {
                resolve_default(&arr, limit, offset)
            };
            return Ok(SearchResult {
                results,
                enriched: false,
            });
        }
    }

    Ok(SearchResult {
        results: vec![],
        enriched: false,
    })
}

fn get_array(index: &Index, term: &str, keyword: &str) -> Option<IntermediateSearchResults> {
    let mut term = term;
    let mut keyword = keyword;

    if !keyword.is_empty() {
        let swap = index.bidirectional && term > keyword;
        if swap {
            std::mem::swap(&mut term, &mut keyword);
        }
    }

    if !keyword.is_empty() {
        if let Some(ctx_map) = index.ctx.index.get(&index.keystore_hash_str(keyword)) {
            if let Some(term_map) = ctx_map.get(term) {
                let mut result = Vec::<Vec<DocId>>::new();
                for (_, doc_ids) in term_map {
                    result.push(doc_ids.clone());
                }
                if !result.is_empty() {
                    return Some(result);
                }
            }
        }
    } else {
        if let Some(term_map) = index.map.index.get(&index.keystore_hash_str(term)) {
            if let Some(doc_ids_map) = term_map.get(term) {
                let mut result = Vec::<Vec<DocId>>::new();
                for (_, doc_ids) in doc_ids_map {
                    result.push(doc_ids.clone());
                }
                if !result.is_empty() {
                    return Some(result);
                }
            }
        }
    }

    None
}

fn add_result(
    arr: IntermediateSearchResults,
    result: &Vec<IntermediateSearchResults>,
    suggest: bool,
    resolution: usize,
) -> Option<Vec<IntermediateSearchResults>> {
    if arr.is_empty() {
        if !suggest {
            return Some(vec![]);
        }
        return None;
    }

    let mut word_arr: IntermediateSearchResults = Vec::new();

    if arr.len() <= resolution {
        let mut new_result = result.clone();
        new_result.push(arr);
        return None;
    }

    for x in 0..resolution {
        if x < arr.len() {
            word_arr.push(arr[x].clone());
        }
    }

    if word_arr.is_empty() {
        if !suggest {
            return Some(vec![]);
        }
        return None;
    }

    let mut new_result = result.clone();
    new_result.push(word_arr);
    None
}

fn return_result(
    result: Vec<IntermediateSearchResults>,
    resolution: usize,
    limit: usize,
    offset: usize,
    suggest: bool,
    resolve: bool,
) -> SearchResults {
    let length = result.len();

    if length == 0 {
        return vec![];
    }

    if length == 1 {
        return resolve_default(&result[0], limit, offset);
    }

    let flattened: Vec<SearchResults> = result.into_iter().flatten().collect();
    let final_result = crate::intersect::intersect(&flattened, resolution, limit, offset, suggest, 0, resolve);

    final_result
}

pub fn resolve_default(arr: &IntermediateSearchResults, limit: usize, offset: usize) -> SearchResults {
    let mut results: SearchResults = Vec::new();
    let mut remaining_limit = limit;
    let mut remaining_offset = offset;

    for ids in arr {
        if ids.is_empty() {
            continue;
        }

        if remaining_offset > 0 {
            if ids.len() <= remaining_offset {
                remaining_offset -= ids.len();
                continue;
            } else {
                let start = remaining_offset;
                let end = std::cmp::min(start + remaining_limit, ids.len());
                results.extend_from_slice(&ids[start..end]);
                remaining_limit -= end - start;
                remaining_offset = 0;
            }
        } else {
            let end = std::cmp::min(remaining_limit, ids.len());
            results.extend_from_slice(&ids[0..end]);
            remaining_limit -= end;
        }

        if remaining_limit == 0 {
            break;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#type::IndexOptions;

    #[test]
    fn test_search_empty() {
        let index = Index::new(IndexOptions::default()).unwrap();
        let options = SearchOptions::default();
        let result = search(&index, &options).unwrap();
        assert_eq!(result.results.len(), 0);
    }

    #[test]
    fn test_search_single_term() {
        let mut index = Index::new(IndexOptions::default()).unwrap();
        index.add(1, "hello world", false).unwrap();
        
        let options = SearchOptions {
            query: Some("hello".to_string()),
            limit: Some(10),
            ..Default::default()
        };
        let result = search(&index, &options).unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0], 1);
    }

    #[test]
    fn test_search_multiple_terms() {
        let mut index = Index::new(IndexOptions::default()).unwrap();
        index.add(1, "hello world", false).unwrap();
        index.add(2, "hello there", false).unwrap();
        
        let options = SearchOptions {
            query: Some("hello world".to_string()),
            limit: Some(10),
            ..Default::default()
        };
        let result = search(&index, &options).unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0], 1);
    }

    #[test]
    fn test_resolve_default() {
        let arr: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let result = resolve_default(&arr, 10, 0);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_resolve_default_with_limit() {
        let arr: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let result = resolve_default(&arr, 4, 0);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_resolve_default_with_offset() {
        let arr: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let result = resolve_default(&arr, 10, 2);
        assert_eq!(result, vec![3, 4, 5, 6]);
    }

    #[test]
    fn test_resolve_default_with_limit_and_offset() {
        let arr: IntermediateSearchResults = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];
        let result = resolve_default(&arr, 3, 2);
        assert_eq!(result, vec![3, 4, 5]);
    }
}
