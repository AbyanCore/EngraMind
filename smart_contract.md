# Eternity Smart Contract Documentation

This document provides an overview of the smart contract functionality implemented in the `eternity_sc` program. The program is built using the Anchor framework for Solana.

## Table of Contents
1. [Program Overview](#program-overview)
2. [Instructions](#instructions)
    - [Profile Management](#profile-management)
    - [Locker Management](#locker-management)
    - [Storage Pointer Management](#storage-pointer-management)
    - [Vault Management](#vault-management)
3. [Error Codes](#error-codes)
4. [Helper Functions](#helper-functions)
5. [Account Definitions](#account-definitions)

---

## Program Overview

The `eternity_sc` program provides functionality for managing user profiles, lockers, storage pointers, and vaults. It includes mechanisms for data validation, ownership checks, and lamport transfers.

---

## Instructions

### Profile Management

- **Create Profile**
  - **Function**: `create_profile`
  - **Description**: Creates a new user profile.
  - **Parameters**: `name`, `age`, `hobbie`, `message`
  - **Validation**: Ensures data length constraints are met.

- **Update Profile**
  - **Function**: `update_profile`
  - **Description**: Updates an existing user profile.
  - **Parameters**: `name`, `age`, `hobbie`, `message`
  - **Ownership Check**: Ensures only the profile owner can update.

---

### Locker Management

- **Create Locker**
  - **Function**: `create_locker`
  - **Description**: Creates a new locker.
  - **Parameters**: `locker_id`, `name`, `description`
  - **Validation**: Ensures data length constraints are met.

- **Update Locker**
  - **Function**: `update_locker`
  - **Description**: Updates an existing locker.
  - **Parameters**: `locker_id`, `name`, `description`, `visibility`
  - **Ownership Check**: Ensures only the locker owner can update.

---

### Storage Pointer Management

- **Create Storage Pointer**
  - **Function**: `create_sp`
  - **Description**: Creates a new storage pointer.
  - **Parameters**: `locker_id`, `sp_id`
  - **Ownership Check**: Ensures only the locker owner can create.

- **Add Storage Pointer**
  - **Function**: `add_sp`
  - **Description**: Adds a key to the storage pointer.
  - **Parameters**: `locker_id`, `sp_id`, `key`
  - **Validation**: Checks storage pointer group limits.

- **Update Storage Pointer**
  - **Function**: `update_sp`
  - **Description**: Updates a key in the storage pointer.
  - **Parameters**: `locker_id`, `sp_id`, `id`, `key`

- **Delete Storage Pointer**
  - **Function**: `delete_sp`
  - **Description**: Deletes a key from the storage pointer.
  - **Parameters**: `locker_id`, `sp_id`, `id`

---

### Vault Management

- **Create Vault**
  - **Function**: `create_vault`
  - **Description**: Creates a vault for managing tokens.

- **Buy Token**
  - **Function**: `buy_token`
  - **Description**: Buys tokens using lamports.
  - **Parameters**: `amount`
  - **Ownership Check**: Ensures only the vault owner can buy.

- **Take Token**
  - **Function**: `take_token`
  - **Description**: Withdraws tokens from the vault.
  - **Parameters**: `amount`
  - **Ownership Check**: Ensures only the vault owner can withdraw.

---

## Error Codes

- `ProfileNotFound`: Profile not found.
- `ProfileAlreadyExists`: Profile already exists.
- `LockerNotFound`: Locker not found.
- `LockerAlreadyExists`: Locker already exists.
- `LockerLimitExceeded`: Maximum number of lockers exceeded.
- `StoragePointerGroupNotFound`: Storage pointer group not found.
- `StoragePointerGroupAlreadyExists`: Storage pointer group already exists.
- `StoragePointerGroupLimitExceeded`: Maximum number of storage pointer groups exceeded.
- `DataNotValid`: Input data is invalid.
- `UnAuthorized`: Unauthorized access.
- `LamportNotEnough`: Insufficient lamports.

---

## Helper Functions

- **`transfer_lamports`**: Handles lamport transfers between accounts.
- **`calculate_rent_and_size`**: Calculates rent and size for account reallocations.

---

## Account Definitions

### Profile
- **Fields**: `owner`, `name`, `age`, `hobbie`, `message`
- **Validation**: Ensures field length constraints.

### Locker
- **Fields**: `owner`, `id`, `name`, `description`, `data_count`, `size`, `visibility`, `storage_pointer`
- **Validation**: Ensures field length constraints.

### Storage Pointer
- **Fields**: `owner`, `locker_id`, `id`, `data`, `data_count`, `next_sp`

### Vault
- **Fields**: `owner`, `token`

### Vault Lamport
- **Fields**: Empty struct for lamport storage.
