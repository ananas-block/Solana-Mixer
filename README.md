### Description

A first attempt to implement tornado cash protocol on Solana. Tornado Cash is a noncustodial mixer which takes in tokens and a commitment, a hash. These commitments are added to a merkle tree. Using a zeroknowledge proof a relayer can withdraw funds to a new address therefore achieving certain anonymity.

In the current stage the program is basically an insecure vault, since the merkle tree implementation is not working and I did not get to the zero proof verifier yet. The program stores commitments and a denominated amount of SOL per commitment. Every commitment is only taken in once. Funds can be withdrawn by supplying the same commitment.

Assuming an installation of rust, cargo, the solana cli and a running test-validator the following commands demonstrate functionality of the program.

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.5.0 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

```
solana-test-validator

solana airdrop 100 ALA2cnz41Wa2v2EYUdkYHsg7VnKsbH1j7secM5aiP8k
```

###Deploy:
```
./run.sh
```  

###Client:
```
cd Client-Js
npm install
npm run-script run 'init storage account'
npm run-script run 'Deposit SOL'
npm run-script run 'Withdraw SOL'
```
The deposit and withdraw commands will only work is this order, since same commitment is used every time and will not be taken twice.
