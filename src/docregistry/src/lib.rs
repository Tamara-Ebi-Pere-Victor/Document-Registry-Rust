#[macro_use]
extern crate serde;

mod storage;
mod types;

use crate::storage::DocReg;
use crate::types::*;
use std::cell::RefCell;

thread_local! {
    static STORAGE: RefCell<DocReg>= RefCell::default();
}

#[ic_cdk::query]
fn get_no_of_docs() -> u64 {
    STORAGE.with(|storage| storage.borrow().get_no_of_docs())
}

#[ic_cdk::query]
fn get_user_docs() -> Option<Vec<u64>> {
    STORAGE.with(|storage| storage.borrow().get_user_docs(ic_cdk::caller()))
}

#[ic_cdk::update]
fn add_document(payload: AddDocumentPayload) -> Result<u64, String> {
    STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .add_document(&payload.doc_hash, &payload.doc_name)
    })
}

#[ic_cdk::query]
fn verify_document(payload: VerifyDocumentPayload) -> Result<Document, String> {
    STORAGE.with(|storage| storage.borrow().verify_document(&payload.doc_hash))
}

#[ic_cdk::query]
fn view_document(payload: ViewDocumentPayload) -> Result<Document, String> {
    STORAGE.with(|storage| storage.borrow().view_document(payload.doc_id))
}

#[ic_cdk::update]
fn delete_document(payload: ViewDocumentPayload) -> Result<Document, String> {
    STORAGE.with(|storage| storage.borrow_mut().delete_document(payload.doc_id))
}

// need this to generate candid
ic_cdk::export_candid!();
