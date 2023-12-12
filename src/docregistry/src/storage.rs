use crate::types::*;
use candid::{Nat, Principal};
use ic_cdk::api::time;
use std::collections::{HashMap, HashSet};

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DocReg {
    pub admin: String,
    pub fee: Nat,
    pub no_of_documents: u64,
    pub id_2_hash_mapping: HashMap<u64, String>,
    pub hash_2_doc_mapping: HashMap<String, Document>,
    pub user_doc_mapping: HashMap<Principal, HashSet<u64>>,
}

impl Default for DocReg {
    fn default() -> Self {
        DocReg {
            admin: String::from(""),
            fee: 0u128.into(),
            no_of_documents: 0,
            id_2_hash_mapping: HashMap::new(),
            hash_2_doc_mapping: HashMap::new(),
            user_doc_mapping: HashMap::new(),
        }
    }
}

impl DocReg {
    pub fn initialize(&mut self, admin: &str, fee: Nat){
        // add admin and fee to pay
        self.admin = String::from(admin);
        self.fee = fee;
    }
    pub fn add_document(&mut self, doc_hash: &str, doc_name: &str) -> Result<u64, String> {
        let next_doc_id = self.no_of_documents;
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
            .insert(String::from(doc_hash), document.clone());

        let user_set = self
            .user_doc_mapping
            .entry(ic_cdk::caller())
            .or_insert_with(HashSet::new);

        user_set.insert(next_doc_id);

        self.no_of_documents += 1;

        Ok(next_doc_id)
    }

    pub fn verify_document(&self, doc_hash: &str) -> Result<Document, String> {
        let document = self
            .hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with hash {} not found", doc_hash))?;

        // Implement document authenticity verification here

        Ok(document.clone())
    }

    pub fn check_document(&self, doc_hash: &str)-> bool {
        let document = self
            .hash_2_doc_mapping
            .get(doc_hash);

        match document {
            Some(_doc) => true,
            None => false
        }
    }

    pub fn view_document(&self, doc_id: u64) -> Result<Document, String> {
        let doc_hash = self
            .id_2_hash_mapping
            .get(&doc_id)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        let document = self
            .hash_2_doc_mapping
            .get(doc_hash)
            .ok_or_else(|| format!("Document with id {} not found", doc_id))?;

        let user_set = self
            .user_doc_mapping
            .get(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in registry"))?;

        if !user_set.contains(&doc_id) {
            return Err(format!(
                "You do not have access to this document with id {}",
                doc_id
            ));
        }

        Ok(document.clone())
    }

    pub fn delete_document(&mut self, doc_id: u64) -> Result<Document, String> {
        let user_set = self
            .user_doc_mapping
            .get_mut(&ic_cdk::caller())
            .ok_or_else(|| format!("You do not have any document in registry"))?;

        if !user_set.contains(&doc_id) {
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

        user_set.remove(&doc_id);
        self.no_of_documents -= 1;

        Ok(document.clone())
    }

    pub fn get_user_docs(&self, user: Principal) -> Option<Vec<u64>> {
        self.user_doc_mapping.get(&user).map(|set| set.iter().cloned().collect())
    }

    pub fn get_no_of_docs(&self) -> u64 {
        self.no_of_documents
    }

    pub fn get_payment_info(&self)-> (String, Nat) {
        (self.admin.clone(), self.fee.clone())
    }
}
