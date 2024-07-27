# Web3 Builders Q3 Cohort Enrollment

## Enroll with Typescript

Install dependencies:

```sh
yarn
```

Create wallet the Dev wallet:

```sh
yarn keygen
```

Aidrop SOL to the Dev wallet:

```sh
yarn airdrop
```

Transfer SOL from the Dev wallet to the WBA wallet:

```sh
yarn transfer
```

Convert WBA wallet private key exported from Phantom:

```sh
yarn convert
```

Enroll in to the 2024 Q3 Cohort

```sh
yarn enroll
```

## Enroll with Rust

Create wallet the Dev wallet:

```sh
cargo test keygen -- --nocapture
```

Aidrop SOL to the Dev wallet:

```sh
cargo test airdop -- --nocapture
```

Transfer SOL from the Dev wallet to the WBA wallet:

```sh
cargo test transfer_sol -- --nocapture
```

Convert WBA wallet private key exported from Phantom:

```sh
cargo test base58_to_wallet -- --nocapture
```

Enroll in to the 2024 Q3 Cohort

```sh
cargo test enroll_to_wba -- --nocapture
```