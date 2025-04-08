# EternityChain Smart Contract

## Overview

The **EternityChain Smart Contract** is a Solana-based program designed to manage user profiles, lockers, storage pointers, and vaults. It provides a secure and efficient way to store and manage data on-chain, while also enabling token-based interactions. This contract is built using the Anchor framework, which simplifies Solana smart contract development.

---

## Features

1. **Profile Management**: Create and update user profiles with attributes like name, age, hobbies, and messages.
2. **Locker System**: Create and manage lockers to store metadata and link to storage pointers.
3. **Storage Pointer Management**: Add, update, and delete storage pointers for lockers.
4. **Vault System**: Manage vaults for token-based interactions, including buying and withdrawing tokens.

---

## Getting Started

### Prerequisites

- Rust and Cargo installed.
- Solana CLI installed.
- Anchor framework installed.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/eternity-sc.git
   cd eternity-sc
   ```

2. Build the program:
   ```bash
   anchor build
   ```

3. Deploy the program:
   ```bash
   anchor deploy
   ```

---

## On-Chain Functions

### Profile Management

#### `create_profile`
- **Description**: Creates a new user profile.
- **Parameters**:
  - `data`: Profile data (name, age, hobbies, message).
- **Access**: Only callable by the signer.

#### `update_profile`
- **Description**: Updates an existing user profile.
- **Parameters**:
  - `data`: Updated profile data.
- **Access**: Only callable by the profile owner.

---

### Locker Management

#### `create_locker`
- **Description**: Creates a new locker for storing metadata.
- **Parameters**:
  - `locker_id`: Unique identifier for the locker.
  - `name`: Name of the locker.
  - `description`: Description of the locker.
- **Access**: Only callable by the signer.

#### `update_locker`
- **Description**: Updates an existing locker.
- **Parameters**:
  - `locker_id`: Locker identifier.
  - `name`: Updated name of the locker.
  - `description`: Updated description of the locker.
  - `visibility`: Updated visibility status.
- **Access**: Only callable by the locker owner.

---

### Storage Pointer Management

#### `create_sp`
- **Description**: Creates a new storage pointer linked to a locker.
- **Parameters**:
  - `locker_id`: Locker identifier.
  - `sp_id`: Storage pointer identifier.
- **Access**: Only callable by the locker owner.

#### `add_sp`
- **Description**: Adds a new key to an existing storage pointer.
- **Parameters**:
  - `locker_id`: Locker identifier.
  - `sp_id`: Storage pointer identifier.
  - `key`: Key to be added.
- **Access**: Only callable by the locker or storage pointer owner.

#### `update_sp`
- **Description**: Updates a key in an existing storage pointer.
- **Parameters**:
  - `locker_id`: Locker identifier.
  - `sp_id`: Storage pointer identifier.
  - `id`: Index of the key to update.
  - `key`: New key value.
- **Access**: Only callable by the storage pointer owner.

#### `delete_sp`
- **Description**: Deletes a key in an existing storage pointer.
- **Parameters**:
  - `locker_id`: Locker identifier.
  - `sp_id`: Storage pointer identifier.
  - `id`: Index of the key to delete.
- **Access**: Only callable by the storage pointer owner.

---

### Vault Management

#### `create_vault`
- **Description**: Creates a new vault for token management.
- **Access**: Only callable by the signer.

#### `buy_token`
- **Description**: Buys tokens by transferring lamports to the vault.
- **Parameters**:
  - `amount`: Amount of lamports to exchange for tokens.
- **Access**: Only callable by the vault owner.

#### `take_token`
- **Description**: Withdraws lamports from the vault by burning tokens.
- **Parameters**:
  - `amount`: Amount of lamports to withdraw.
- **Access**: Only callable by the vault owner.

---

## Error Codes

- **Profile Errors**:
  - `ProfileNotFound`: Profile not found.
  - `ProfileAlreadyExists`: Profile already exists.

- **Locker Errors**:
  - `LockerNotFound`: Locker not found.
  - `LockerAlreadyExists`: Locker already exists.
  - `LockerLimitExceeded`: Maximum number of lockers exceeded.

- **Storage Pointer Errors**:
  - `StoragePointerGroupNotFound`: Storage pointer group not found.
  - `StoragePointerGroupAlreadyExists`: Storage pointer group already exists.
  - `StoragePointerGroupLimitExceeded`: Maximum number of storage pointer groups exceeded.

- **General Errors**:
  - `DataNotValid`: Provided data is invalid.
  - `UnAuthorized`: Caller is not authorized.
  - `LamportNotEnough`: Not enough lamports or tokens.

---

## Account Definitions

### Profile
- **Fields**:
  - `owner`: Public key of the profile owner.
  - `name`: Name of the profile (max 100 characters).
  - `age`: Age of the profile owner.
  - `hobbie`: List of hobbies (max 5, each max 100 characters).
  - `message`: Custom message (max 300 characters).

### Locker
- **Fields**:
  - `owner`: Public key of the locker owner.
  - `id`: Unique identifier for the locker.
  - `name`: Name of the locker (max 50 characters).
  - `description`: Description of the locker (max 300 characters).
  - `data_count`: Number of data entries in the locker.
  - `size`: Size of the locker.
  - `visibility`: Visibility status of the locker.
  - `storage_pointer`: Optional public key of the linked storage pointer.

### Storage Pointer
- **Fields**:
  - `owner`: Public key of the storage pointer owner.
  - `locker_id`: Identifier of the linked locker.
  - `id`: Unique identifier for the storage pointer.
  - `data`: List of keys (max 1 key, each 32 bytes).
  - `data_count`: Number of keys in the storage pointer.
  - `next_sp`: Optional public key of the next storage pointer.

### Vault
- **Fields**:
  - `owner`: Public key of the vault owner.
  - `token`: Number of tokens in the vault.

---

## Helper Functions

### `transfer_lamports`
- Transfers lamports between accounts.

### `calculate_rent_and_size`
- Calculates the rent and size for account reallocations.

---

## Testing

Run the tests using the Anchor framework:
```bash
anchor test
```

---

## License

This project is licensed under the MIT License.