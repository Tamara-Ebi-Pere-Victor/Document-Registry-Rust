# DOCUMENT-REGISTRY-RUST

This is a icp canister application that helps users verify documents that have been issued by an organization.

The application uses the encryption method sha256 and to produce a distinct key that is identifiable to that single issued document.

## Canister Parameters

- Users have the ability to add documents to the contract and they can also delete the documents they added.
- Other users can verify if a document already exists on the registry.

## Use Cases

1. This Dapp can be used by document issuing organizations, like schools, business, e.t.c.
2. It can be used to ensure validity of a perticular document, and help reduce the effect of forgery in the professional world.

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

# This deploys the canister to the replica and generates your candid interface
npm run gen-deploy
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
