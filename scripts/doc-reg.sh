admin=$(dfx identity get-principal --identity default)
dfx deploy docregistry --argument "(record { admin = $admin; fee = $1 })"