# Solana Mini Escrow

A high-performance Solana program designed for secure SPL token exchanges. This project utilizes the Anchor framework to ensure memory safety and clear account validation.

### Workflow
1. **Initialize:** Party A (Maker) locks tokens into a temporary vault account and specifies the amount they want in return from Party B.
2. **Exchange:** Party B (Taker) sends the requested tokens to Party A, and the vault automatically releases the locked tokens to Party B.
3. **Cancel:** Party A can cancel the escrow and retrieve their locked tokens at any time before the exchange occurs.

### Requirements
* Rust / Cargo
* Solana CLI
* Anchor Framework

### Build & Deploy
`anchor build`
`anchor deploy`

### License
MIT
