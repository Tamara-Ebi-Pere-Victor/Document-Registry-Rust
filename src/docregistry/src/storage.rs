use crate::types::*;
use candid::Principal;
use ic_cdk::api::time;
use std::collections::HashMap;

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DocReg {
    pub no_of_documents: u64,
    pub id_2_hash_mapping: HashMap<u64, String>,
    pub hash_2_doc_mapping: HashMap<String, Document>,
    pub user_doc_mapping: HashMap<Principal, Vec<u64>>,
}

impl Default for DocReg {
    fn default() -> Self {
        DocReg {
            no_of_documents: 0,
            id_2_hash_mapping: HashMap::new(),
            hash_2_doc_mapping: HashMap::new(),
            user_doc_mapping: HashMap::new(),
        }
    }
}

impl DocReg {
    pub fn add_document(&mut self, doc_hash: &str, doc_name: &str) -> Result<u64, String> {
        let next_doc_id = self.no_of_documents;
        self.no_of_documents += 1;
        let document = Document {
            id: next_doc_id,
            name: String::from(doc_name),
            hash: String::from(doc_hash),
            created_at: time(),
            owner: ic_cdk::caller(),
        };

        self.id_2_hash_mapping
            .insert(next_doc_id, String::from(doc_hash));

        self.hash_2_doc_mapping
            .insert(String::from(doc_hash), document);

        let user_map = self.user_doc_mapping.get_mut(&ic_cdk::caller());

        match user_map {
            Some(value) => {
                value.push(next_doc_id);
                Ok(next_doc_id)
            }
            None => {
                let mut value: Vec<u64> = Vec::new();
                value.push(next_doc_id);
                self.user_doc_mapping.insert(ic_cdk::caller(), value);
                Ok(next_doc_id)
            }
        }
    }

    pub fn verify_document(&self, doc_hash: &str) -> Result<Document, String> {
        let document = self
            .hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with hash {} not found", doc_hash))?;

        Ok(document.clone())
    }

    pub fn view_document(&self, doc_id: u64) -> Result<Document, String> {
        let user_docs = self
            .user_doc_mapping
            .get(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in registry"))?;

        if !user_docs.contains(&doc_id) {
            return Err(format!(
                "You do not have access to this document with id {}",
                doc_id
            ));
        }

        let doc_hash = self
            .id_2_hash_mapping
            .get(&doc_id)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        let document = self
            .hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        Ok(document.clone())
    }

    pub fn delete_document(&mut self, doc_id: u64) -> Result<Document, String> {
        let user_docs = self
            .user_doc_mapping
            .get_mut(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in registry"))?;

        if !user_docs.contains(&doc_id) {
            return Err(format!(
                "You do not have access to this document with id {}",
                doc_id
            ));
        }

        let index = user_docs.iter().position(|x| *x == doc_id).unwrap();
        user_docs.remove(index);

        let doc_hash = self
            .id_2_hash_mapping
            .remove(&doc_id)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        let document = self
            .hash_2_doc_mapping
            .remove(&doc_hash)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        self.no_of_documents -= 1;

        Ok(document.clone())
    }

    pub fn get_user_docs(&self, user: Principal) -> Option<Vec<u64>> {
        self.user_doc_mapping.get(&user).cloned()
    }

    pub fn get_no_of_docs(&self) -> u64 {
        self.no_of_documents
    }
}
