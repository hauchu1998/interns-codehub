# nft-voter: create a NFT powered govereance in spl-governance
`nft-voter` is a plugin designed to empower DAOs by leveraging NFTs and cNFTs within the spl-governance framework. This integration allows DAOs to manage and control their realms with an alternative way, providing both secure and a customizable governance model based on NFT ownership.

[slide](https://docs.google.com/presentation/d/1VWgNDAyS3RxijzVQWLZHpuxXdZ8YBMuZLIOCxHFEePU/edit?usp=sharing)

## Features 🌟
1. *NFT Collection Configuration* 🎨
- Easily register your approved NFT collections to your realm.
- Ensure that only genuine and approved NFT can participate in the governance process.

2. *Record Voter's Weight* ⚖️
- Keep track and monitor the voting power of each NFTs holder.
- This feature allows for a flexible and dynamic assignment of voting weights, enabling different levels of influence based on realm's configuration.

3. *NFT Ownership Verification* 🔒
- Ensure the authenticity of each vote by verifying NFT ownership.
- Prevent fraudulent voting or manipulative tactics by keeping a robust verification process.

4. *Execute Voter Action* ⚙️
- Seamlessly execute actions based on the collective decisions of NFT holders.
- From proposals to specific DAO functions, ensure a smooth execution process guided by the will of your community.

## Installation 🛠️
```cmd
# build
cd /programs/{program}
cargo build-sbf

# test
cd /programs/{program}
cargo test-sbf

# generate idl (json, ts)
anchor build --arch sbf

# deploy contract
cargo program deploy <program file path> --program-id <keypair of program id file path>
```

## Developers & Maintainers 🤖

