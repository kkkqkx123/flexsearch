use crate::r#type::{SearchResults, IntermediateSearchResults};
use std::collections::HashMap;

pub fn intersect(
    arrays: &[SearchResults],
    resolution: usize,
    limit: usize,
    offset: usize,
    suggest: bool,
    boost: i32,
    resolve: bool,
) -> SearchResults {
    let length = arrays.len();

    if length == 0 {
        return vec![];
    }

    if length == 1 {
        if resolve {
            return arrays[0][offset..].to_vec();
        } else {
            return arrays[0][offset..].to_vec();
        }
    }

    let mut check: HashMap<u64, usize> = HashMap::new();
    let mut result: Vec<Vec<u64>> = Vec::new();

    for y in 0..resolution {
        for x in 0..length {
            if y >= arrays[x].len() {
                continue;
            }

            let id = arrays[x][y];

            let count = check.entry(id).or_insert(0);
            *count += 1;

            let slot_idx = *count;
            if slot_idx > result.len() {
                result.push(Vec::new());
            }

            if resolve {
                result[slot_idx - 1].push(id);

                if limit > 0 && slot_idx == length - 1 {
                    let slot = &result[slot_idx - 1];
                    if slot.len() >= offset + limit {
                        let start = offset;
                        let end = std::cmp::min(start + limit, slot.len());
                        return slot[start..end].to_vec();
                    }
                }
            } else {
                let score = y as i32 + if x > 0 || !suggest { 0 } else { boost };
                let score_idx = score as usize;
                if score_idx >= result[slot_idx - 1].len() {
                    result[slot_idx - 1].resize(score_idx + 1, 0);
                }
                result[slot_idx - 1][score_idx] = id;
            }
        }
    }

    let result_len = result.len();

    if result_len == 0 {
        return vec![];
    }

    if !suggest {
        if result_len < length {
            return vec![];
        }

        let final_result = &result[length - 1];

        if resolve {
            if limit > 0 || offset > 0 {
                let start = offset;
                let end = if limit > 0 {
                    std::cmp::min(start + limit, final_result.len())
                } else {
                    final_result.len()
                };
                return final_result[start..end].to_vec();
            }
            return final_result.clone();
        } else {
            return final_result.clone();
        }
    } else {
        if result_len > 1 {
            return union(&result, limit, offset, resolve, boost);
        } else {
            let final_result = &result[0];
            if limit > 0 || offset > 0 {
                let start = offset;
                let end = if limit > 0 {
                    std::cmp::min(start + limit, final_result.len())
                } else {
                    final_result.len()
                };
                return final_result[start..end].to_vec();
            }
            return final_result.clone();
        }
    }
}

pub fn union(
    arrays: &[Vec<u64>],
    limit: usize,
    offset: usize,
    _resolve: bool,
    _boost: i32,
) -> SearchResults {
    let mut result: SearchResults = Vec::new();
    let mut check: HashMap<u64, bool> = HashMap::new();
    let mut count = 0;
    let mut offset_remaining = offset;

    for i in (0..arrays.len()).rev() {
        let ids = &arrays[i];

        for &id in ids.iter().rev() {
            if check.contains_key(&id) {
                continue;
            }

            check.insert(id, true);

            if offset_remaining > 0 {
                offset_remaining -= 1;
                continue;
            }

            result.push(id);
            count += 1;

            if limit > 0 && count >= limit {
                return result;
            }
        }
    }

    result
}

pub fn intersect_union(
    arrays: &SearchResults,
    mandatory: &[SearchResults],
    _resolve: bool,
) -> SearchResults {
    let mut check: HashMap<u64, bool> = HashMap::new();
    let mut result: SearchResults = Vec::new();

    for mandatory_ids in mandatory {
        for &id in mandatory_ids {
            check.insert(id, true);
        }
    }

    for &id in arrays {
        if check.contains_key(&id) {
            result.push(id);
            check.remove(&id);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_empty() {
        let arrays: Vec<SearchResults> = vec![];
        let result = intersect(&arrays, 9, 10, 0, false, 0, true);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_intersect_single() {
        let arrays: Vec<SearchResults> = vec![
            vec![1, 2, 3],
        ];
        let result = intersect(&arrays, 9, 10, 0, false, 0, true);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_intersect_two_arrays() {
        let arrays: Vec<SearchResults> = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
        ];
        let result = intersect(&arrays, 9, 10, 0, false, 0, true);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_intersect_with_resolution() {
        let arrays: Vec<SearchResults> = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
        ];
        let result = intersect(&arrays, 3, 10, 0, false, 0, true);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_intersect_with_limit() {
        let arrays: Vec<SearchResults> = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
        ];
        let result = intersect(&arrays, 9, 2, 0, false, 0, true);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_intersect_with_offset() {
        let arrays: Vec<SearchResults> = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
        ];
        let result = intersect(&arrays, 9, 10, 2, false, 0, true);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_union() {
        let arrays: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
        ];
        let result = union(&arrays, 10, 0, true, 0);
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_union_with_limit() {
        let arrays: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
        ];
        let result = union(&arrays, 3, 0, true, 0);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_union_with_offset() {
        let arrays: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
        ];
        let result = union(&arrays, 10, 2, true, 0);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_intersect_union() {
        let arrays: SearchResults = vec![1, 2, 3, 4, 5];
        let mandatory: Vec<SearchResults> = vec![vec![2, 3, 4]];
        let result = intersect_union(&arrays, &mandatory, true);
        assert_eq!(result, vec![2, 3, 4]);
    }
}
