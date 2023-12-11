#[macro_use]
extern crate serde;

mod storage;
mod types;

use crate::storage::DocReg;
use crate::types::*;
use candid::{Nat, Principal};
use std::cell::RefCell;
use ic_cdk::api::call::CallResult;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};

thread_local! {
    static STORAGE: RefCell<DocReg>= RefCell::default();
}

#[ic_cdk::init]
fn init(payload: InitPayload){
    // check if text is valid
    let _principal = Principal::from_text(payload.admin.clone()).expect("invalid text address");
    // initialize
    STORAGE.with(|storage| storage.borrow_mut().initialize(&payload.admin, payload.fee))
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
async fn add_document(payload: AddDocumentPayload) -> Result<u64, String> {
    // get payment details
    let (admin, fee) = STORAGE.with(|storage| {
        storage
            .borrow().get_payment_info()
    });

    // create transfer payload
    let transfer_payload = TransferPayload {
        owner: Principal::from_text(admin).expect("invalid text"),
        amount: fee,
    };

    // send funds to admin
    let result = _transfer(transfer_payload)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e));


    // add documents if fee paid successfully
    match result {
        Ok(_value) => STORAGE.with(|storage| { storage .borrow_mut().add_document(&payload.doc_hash, &payload.doc_name)}),
        Err(error) => Err(error),
    }
}

#[ic_cdk::query]
async fn verify_document(payload: VerifyDocumentPayload) -> Result<Document, String> {
    // get payment details
    let (admin, fee) = STORAGE.with(|storage| {
        storage
            .borrow().get_payment_info()
    });

    // create transfer payload
    let transfer_payload = TransferPayload {
        owner: Principal::from_text(admin).expect("invalid text"),
        amount: fee/2,
    };

    // send funds to admin
    let result = _transfer(transfer_payload)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e));

    // verify documents if fee paid successfully
    match result {
        Ok(_value) => STORAGE.with(|storage| storage.borrow().verify_document(&payload.doc_hash)),
        Err(error) => Err(error),
    }
}

#[ic_cdk::query]
fn view_document(payload: ViewDocumentPayload) -> Result<Document, String> {
    STORAGE.with(|storage| storage.borrow().view_document(payload.doc_id))
}

#[ic_cdk::update]
fn delete_document(payload: ViewDocumentPayload) -> Result<Document, String> {
    STORAGE.with(|storage| storage.borrow_mut().delete_document(payload.doc_id))
}

#[ic_cdk::update]
async fn get_admin_balance() -> Result<Nat, String> {
    // get admin details
    let (admin, _fee) = STORAGE.with(|storage| {
        storage
            .borrow().get_payment_info()
    });

    let admin = Principal::from_text(admin).expect("invalid text address");

    let balance = _get_balance(admin)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));
    balance
}

async fn _transfer(transfer_args: TransferPayload) -> CallResult<Result<Nat, TransferError>> {
    let ledger_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();
    // The request object of the `icrc1_name` endpoint is empty.
    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: transfer_args.owner,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: transfer_args.amount,
    };
    let (result,): (Result<Nat, TransferError>,) =
        ic_cdk::call(ledger_id, "icrc1_transfer", (args,)).await?;

    Ok(result)
}

// helper method to get balance
async fn _get_balance(account: Principal) -> CallResult<Nat> {
    let ledger_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();
    let args = Account {
        owner: account,
        subaccount: None,
    };
    // The request object of the `icrc1_name` endpoint is empty.
    let (result,): (Nat,) = ic_cdk::call(ledger_id, "icrc1_balance_of", (args,)).await?;
    Ok(result)
}

// a helper to help get canister principal
#[ic_cdk::query]
fn get_principal() -> Principal {
    ic_cdk::api::id()
}

// need this to generate candid
ic_cdk::export_candid!();
