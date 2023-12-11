# DOCUMENT-REGISTRY-RUST

This is a icp canister application that helps users verify documents that have been issued by an organization.

The application uses the encryption method sha256 and to produce a distinct key that is identifiable to that single issued document.

## Canister Parameters

- Users have the ability to add documents to the contract and they can also delete the documents they added.
- Other users can verify if a document already exists on the registry.

## Use Cases

1. This Dapp can be used by document issuing organizations, like schools, business, e.t.c.
2. It can be used to ensure validity of a perticular document, and help reduce the effect of forgery in the professional world.

## UPDATE Due to feedback

I added the following features due to feedbacks from the dacade platform

1. A ICRC1-Ledger to enable the doc registry canister accept payments of dummy tokens.
  
- when users try to add a document they pay a fee
- when users try to verify a document they pay half the fee.

    To set up this ledger follow the next instructions

  - Start by creating the following identities on your dfx instance, these identities make the creation of the ledger seamless. For more information about the identities check the [ICRC1 tutorial](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/icrc1-ledger-setup)

      ```bash
      # The minter identity
      dfx identity new minter

      # The archive controller
      dfx identity new archive_controller
      ```

  - Then proceed to deploy the ICRC1 Ledger, a script has been supplied for that. This sets up the ledger.

    ```bash
    npm run deploy-ledger
    ```
  
  - Now you can run the faucet script which mints new tokens to our coffee canister. `<amount>` is a placeholder for any amount of tokens in e8s you want to mint

    ```bash
    # npm run faucet <amount>
    npm run faucet 100_000_000_000
    ```

2. An init function which takes the text format of the admin principal (where the fee will be sent to) and the fee to be paid. This init function is called as we deploy the doc registry. A helper script has been provided for that.

    ```bash
    # npm run deploy-docreg <fee>
    npm run deploy-docreg 10_000
    ```

    `<fee>` is the place holder for the fee you want users to pay to add documents. Also recall that users pay half this fee to verify any document.

## More Information

To learn more before you start working with docregistry, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.icp0.io)

## Running the project locally

To test project locally, use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# deploy the icp ledger
npm run deploy-ledger 

# deploy the doc reg canister
# npm run deploy-docreg <fee>
npm run deploy-docreg 10_000

# run the faucet
# npm run faucet <amount>
npm run faucet 100_000_000_000
```

Once the job completes, your application will be available at

``` txt
http://localhost:4943?canisterId={candid_ui_id}&id={canister_id}

```

## Testing the application

1. First thing to do is to generate the hash of the document you wish to upload. Run any of the following commands on your terminal and copy the document hash.

```bash
  # For Linux users
  # sha256sum <path to document>
  sha256sum ./examples/AttendanceCertificate.pdf

  # For window users using powershell
  # Get-FileHash <path to document> -Algorithm SHA256 | Select-Object Hash
  Get-FileHash ./examples/CertificateOfAdoption.pdf -Algorithm SHA256 | Select-Object Hash

  # For Mac OS users
  # shasum -a 256 <path to document>
  shasum -a 256 ./examples/DegreeCertificate.pdf

```

2. Next go to the candid UI link generated and test out the application.

3. Trying adding documents and use the `get_admin_balance` function to check if the admin balance is truly increased.