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

        let created_at = time(); // Ensure that this is a valid timestamp.

        let document = Document {
            id: next_doc_id,
            name: String::from(doc_name),
            hash: String::from(doc_hash),
            created_at,
            owner: ic_cdk::caller(),
        };

        self.id_2_hash_mapping.insert(next_doc_id, String::from(doc_hash));

        let user_map = self.user_doc_mapping.entry(ic_cdk::caller()).or_insert_with(Vec::new);
        user_map.push(next_doc_id);

        self.hash_2_doc_mapping.insert(String::from(doc_hash), document);

        Ok(next_doc_id)
    }

    pub fn verify_document(&self, doc_hash: &str) -> Result<&Document, String> {
        self.hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with hash {} not found", doc_hash))
    }

    pub fn view_document(&self, doc_id: u64) -> Result<&Document, String> {
        let user_docs = self
            .user_doc_mapping
            .get(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in the registry"))?;

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

        self.hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))
    }

    pub fn delete_document(&mut self, doc_id: u64) -> Result<Document, String> {
        let user_docs = self
            .user_doc_mapping
            .get_mut(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in the registry"))?;

        if !user_docs.contains(&doc_id) {
            return Err(format!(
                "You do not have access to this document with id {}",
                doc_id
            ));
        }

        let doc_hash = self
            .id_2_hash_mapping
            .remove(&doc_id)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        let document = self
            .hash_2_doc_mapping
            .remove(&doc_hash)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        self.no_of_documents -= 1;

        Ok(document)
    }

    pub fn get_user_docs(&self, user: Principal) -> Option<&Vec<u64>> {
        self.user_doc_mapping.get(&user)
    }

    pub fn get_no_of_docs(&self) -> u64 {
        self.no_of_documents
    }
}
