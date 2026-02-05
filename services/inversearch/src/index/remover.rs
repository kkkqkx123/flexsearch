use crate::index::{Index, DocId};
use crate::error::{Result, InversearchError};
use std::collections::HashMap;

pub fn remove_document(index: &mut Index, id: DocId, skip_deletion: bool) -> Result<()> {
    if index.fastupdate {
        if let Some(refs) = get_fastupdate_refs(index, id) {
            remove_fastupdate(index, refs, id)?;
        }
    } else {
        remove_from_index(index, id)?;
    }

    if !skip_deletion {
        match &mut index.reg {
            crate::index::Register::Set(reg) => {
                reg.delete(&id);
            }
            crate::index::Register::Map(reg) => {
                reg.delete(&id);
            }
        }
    }

    Ok(())
}

fn get_fastupdate_refs(index: &Index, id: DocId) -> Option<Vec<(bool, String, Option<String>, usize)>> {
    match &index.reg {
        crate::index::Register::Map(reg) => {
            let id_hash = index.keystore_hash_str(&id.to_string());
            if let Some(id_map) = reg.index.get(&id_hash) {
                if let Some(refs) = id_map.get(&id) {
                    return Some(refs.clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn remove_fastupdate(
    index: &mut Index,
    refs: Vec<(bool, String, Option<String>, usize)>,
    id: DocId,
) -> Result<()> {
    for (is_ctx, term, keyword, score) in refs {
        if is_ctx {
            if let Some(kw) = keyword {
                let kw_hash = index.keystore_hash_str(&kw);
                if let Some(outer_map) = index.ctx.index.get_mut(&kw_hash) {
                    if let Some(term_map) = outer_map.get_mut(&term) {
                        if let Some(doc_ids_map) = term_map.get_mut(&score) {
                            for (_, doc_ids) in doc_ids_map.iter_mut() {
                                if let Some(pos) = doc_ids.iter().position(|x| x == &id) {
                                    if doc_ids.len() > 1 {
                                        doc_ids.remove(pos);
                                    } else {
                                        doc_ids.clear();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let term_hash = index.keystore_hash_str(&term);
            if let Some(outer_map) = index.map.index.get_mut(&term_hash) {
                if let Some(term_map) = outer_map.get_mut(&term) {
                    if let Some(doc_ids_map) = term_map.get_mut(&score.to_string()) {
                        for (_, doc_ids) in doc_ids_map.iter_mut() {
                            if let Some(pos) = doc_ids.iter().position(|x| x == &id) {
                                if doc_ids.len() > 1 {
                                    doc_ids.remove(pos);
                                } else {
                                    doc_ids.clear();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn remove_from_index(index: &mut Index, id: DocId) -> Result<()> {
    for (_, outer_map) in index.map.index.iter_mut() {
        for (_, term_map) in outer_map.iter_mut() {
            for (_, doc_ids_map) in term_map.iter_mut() {
                for (_, doc_ids) in doc_ids_map.iter_mut() {
                    if let Some(pos) = doc_ids.iter().position(|x| x == &id) {
                        if doc_ids.len() > 1 {
                            doc_ids.remove(pos);
                        } else {
                            doc_ids.clear();
                        }
                    }
                }
            }
        }
    }

    for (_, outer_map) in index.ctx.index.iter_mut() {
        for (_, term_map) in outer_map.iter_mut() {
            for (_, doc_ids_map) in term_map.iter_mut() {
                for (_, doc_ids) in doc_ids_map.iter_mut() {
                    if let Some(pos) = doc_ids.iter().position(|x| x == &id) {
                        if doc_ids.len() > 1 {
                            doc_ids.remove(pos);
                        } else {
                            doc_ids.clear();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
