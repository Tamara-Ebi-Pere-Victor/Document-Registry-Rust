use candid::Principal;

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: u64,
    pub name: String,
    pub hash: String,
    pub created_at: u64,
    pub owner: Principal,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct AddDocumentPayload {
    pub doc_name: String,
    pub doc_hash: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct VerifyDocumentPayload {
    pub doc_hash: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct ViewDocumentPayload {
    pub doc_id: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct DeleteDocumentPayload {
    pub doc_id: u64,
}
