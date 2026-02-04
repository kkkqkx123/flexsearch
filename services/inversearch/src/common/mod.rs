pub fn create_object() -> serde_json::Map<String, serde_json::Value> {
    serde_json::Map::new()
}

pub fn is_array(val: &serde_json::Value) -> bool {
    val.is_array()
}

pub fn is_string(val: &serde_json::Value) -> bool {
    val.is_string()
}

pub fn is_object(val: &serde_json::Value) -> bool {
    val.is_object()
}

pub fn is_function(_val: &serde_json::Value) -> bool {
    false
}

pub fn concat<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|arr| arr.iter().cloned()).collect()
}

pub fn sort_by_length_down<T>(a: &[T], b: &[T]) -> std::cmp::Ordering {
    b.len().cmp(&a.len())
}

pub fn sort_by_length_up<T>(a: &[T], b: &[T]) -> std::cmp::Ordering {
    a.len().cmp(&b.len())
}

pub fn parse_simple(obj: &serde_json::Value, tree: &[String]) -> Option<serde_json::Value> {
    let mut current = obj;
    for key in tree {
        match current.get(key) {
            Some(val) => current = val,
            None => return None,
        }
    }
    Some(current.clone())
}

pub fn get_max_len<T>(arr: &[Vec<T>]) -> usize {
    arr.iter().map(|v| v.len()).max().unwrap_or(0)
}

pub fn to_array<T: Clone + Eq + std::hash::Hash>(val: &std::collections::HashSet<T>, stringify: bool) -> Vec<T> {
    if stringify {
        unimplemented!("Stringify not implemented for generic type");
    }
    val.iter().cloned().collect()
}

pub fn merge_option<T>(value: Option<T>, default_value: T, merge_value: Option<T>) -> T
where
    T: Clone,
{
    if let Some(merge) = merge_value {
        if let Some(val) = value {
            if merge_value.is_some() {
                val
            } else {
                merge
            }
        } else {
            merge
        }
    } else {
        value.unwrap_or(default_value)
    }
}

pub fn inherit<T>(target_value: Option<T>, default_value: T) -> T {
    target_value.unwrap_or(default_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_object() {
        let obj = create_object();
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn test_is_array() {
        let arr = serde_json::json!([1, 2, 3]);
        assert!(is_array(&arr));

        let obj = serde_json::json!({"key": "value"});
        assert!(!is_array(&obj));
    }

    #[test]
    fn test_is_string() {
        let s = serde_json::json!("hello");
        assert!(is_string(&s));

        let arr = serde_json::json!([1, 2, 3]);
        assert!(!is_string(&arr));
    }

    #[test]
    fn test_is_object() {
        let obj = serde_json::json!({"key": "value"});
        assert!(is_object(&obj));

        let arr = serde_json::json!([1, 2, 3]);
        assert!(!is_object(&obj));
    }

    #[test]
    fn test_concat() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 5, 6];
        let result = concat(&[arr1, arr2]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sort_by_length_down() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 5];
        let arr3 = vec![6];
        assert_eq!(sort_by_length_down(&arr1, &arr2), std::cmp::Ordering::Less);
        assert_eq!(sort_by_length_down(&arr2, &arr3), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_sort_by_length_up() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 5];
        let arr3 = vec![6];
        assert_eq!(sort_by_length_up(&arr1, &arr2), std::cmp::Ordering::Greater);
        assert_eq!(sort_by_length_up(&arr2, &arr3), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_parse_simple() {
        let obj = serde_json::json!({
            "a": {
                "b": {
                    "c": "value"
                }
            }
        });
        let tree = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = parse_simple(&obj, &tree);
        assert_eq!(result, Some(serde_json::json!("value")));
    }

    #[test]
    fn test_get_max_len() {
        let arr = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
        assert_eq!(get_max_len(&arr), 3);
    }

    #[test]
    fn test_merge_option() {
        assert_eq!(merge_option(Some(1), 2, None), 1);
        assert_eq!(merge_option(None, 2, None), 2);
        assert_eq!(merge_option(None, 2, Some(3)), 3);
    }

    #[test]
    fn test_inherit() {
        assert_eq!(inherit(Some(1), 2), 1);
        assert_eq!(inherit(None, 2), 2);
    }
}
