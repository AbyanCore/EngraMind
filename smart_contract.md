# Eternity Smart Contract Documentation

This document provides a comprehensive overview of the `eternity_sc` program, including instructions, data structures, error codes, and helper functions. The program is built using the Anchor framework for Solana.

## Table of Contents
1. [Program Overview](#program-overview)
2. [Instructions](#instructions)
    - [Personality Management](#personality-management)
    - [Relic Management](#relic-management)
    - [Fragment Management](#fragment-management)
    - [Vault Management](#vault-management)
3. [Error Codes](#error-codes)
4. [Helper Functions](#helper-functions)
5. [Account Definitions](#account-definitions)
6. [Events](#events)

---

## Program Overview

The `eternity_sc` program provides functionality for managing personalities, relics, fragments, and vaults. It includes mechanisms for data validation, ownership checks, and lamport transfers.

---

## Instructions

### Personality Management

- **Create Personality**
  - **Function**: `create_personality`
  - **Description**: Creates a new personality profile.
  - **Parameters**:
    - `name` (String): The name of the personality. Maximum length: 100 characters.
    - `age` (u16): The age of the personality.
    - `hobbie` (Vec<String>): A list of hobbies. Maximum 5 hobbies, each up to 100 characters.
    - `message` (String): A custom message. Maximum length: 300 characters.
  - **Validation**:
    - `name` must not exceed 100 characters.
    - `hobbie` must not exceed 5 items, and each item must not exceed 100 characters.
    - `message` must not exceed 300 characters.

- **Update Personality**
  - **Function**: `update_personality`
  - **Description**: Updates an existing personality profile.
  - **Parameters**:
    - `name` (String): The name of the personality. Maximum length: 100 characters.
    - `age` (u16): The age of the personality.
    - `hobbie` (Vec<String>): A list of hobbies. Maximum 5 hobbies, each up to 100 characters.
    - `message` (String): A custom message. Maximum length: 300 characters.
  - **Ownership Check**: Ensures only the profile owner can update.
  - **Validation**:
    - `name` must not exceed 100 characters.
    - `hobbie` must not exceed 5 items, and each item must not exceed 100 characters.
    - `message` must not exceed 300 characters.

- **Set Personality Message**
  - **Function**: `m_set_personality_message`
  - **Description**: Updates the message field of a personality profile.
  - **Parameters**:
    - `message` (String): A custom message. Maximum length: 300 characters.
  - **Ownership Check**: Ensures only the profile owner can update.
  - **Validation**:
    - `message` must not exceed 300 characters.

- **Set Personality Hobbie**
  - **Function**: `m_set_personality_hobbie`
  - **Description**: Updates the hobbie field of a personality profile.
  - **Parameters**:
    - `hobbie` (Vec<String>): A list of hobbies. Maximum 5 hobbies, each up to 100 characters.
  - **Ownership Check**: Ensures only the profile owner can update.
  - **Validation**:
    - `hobbie` must not exceed 5 items, and each item must not exceed 100 characters.

---

### Relic Management

- **Create Relic**
  - **Function**: `create_relic`
  - **Description**: Creates a new relic.
  - **Parameters**:
    - `_relic_id` (u32): A unique identifier for the relic.
    - `name` (String): The name of the relic. Maximum length: 50 characters.
    - `description` (String): A description of the relic. Maximum length: 300 characters.
  - **Validation**:
    - `name` must not exceed 50 characters.
    - `description` must not exceed 300 characters.

- **Update Relic**
  - **Function**: `update_relic`
  - **Description**: Updates an existing relic.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `name` (String): The updated name of the relic. Maximum length: 50 characters.
    - `description` (String): The updated description of the relic. Maximum length: 300 characters.
    - `visibility` (bool): The visibility status of the relic.
  - **Ownership Check**: Ensures only the relic owner or authority can update.
  - **Validation**:
    - `name` must not exceed 50 characters.
    - `description` must not exceed 300 characters.

- **Set Relic Description**
  - **Function**: `m_set_relic_description`
  - **Description**: Updates the description field of a relic.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `description` (String): The updated description. Maximum length: 300 characters.
  - **Ownership Check**: Ensures only the relic owner or authority can update.
  - **Validation**:
    - `description` must not exceed 300 characters.

---

### Fragment Management

- **Create Fragments**
  - **Function**: `create_fragments`
  - **Description**: Creates a new fragment and links it to a relic.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `_fragment_id` (u32): The unique identifier for the fragment.

- **Add Fragment**
  - **Function**: `m_add_fragment`
  - **Description**: Adds a key to a fragment.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `_fragment_id` (u32): The unique identifier for the fragment.
    - `key` ([u8; 32]): A 32-byte key to add to the fragment.
  - **Validation**:
    - The fragment must not exceed 500 keys.

- **Update Fragment**
  - **Function**: `m_update_fragment`
  - **Description**: Updates a key in a fragment.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `_fragment_id` (u32): The unique identifier for the fragment.
    - `id` (u16): The index of the key to update.
    - `key` ([u8; 32]): The updated 32-byte key.
  - **Validation**:
    - The `id` must be within the range of existing keys.

- **Delete Fragment**
  - **Function**: `m_delete_fragment`
  - **Description**: Deletes a key from a fragment.
  - **Parameters**:
    - `_relic_id` (u32): The unique identifier for the relic.
    - `_fragment_id` (u32): The unique identifier for the fragment.
    - `id` (u16): The index of the key to delete.
  - **Validation**:
    - The `id` must be within the range of existing keys.

---

### Vault Management

- **Create Vault**
  - **Function**: `create_vault`
  - **Description**: Creates a vault for managing tokens.

- **Buy Token**
  - **Function**: `m_buy_token`
  - **Description**: Buys tokens using lamports.
  - **Parameters**:
    - `amount` (u64): The number of lamports to spend.
  - **Ownership Check**: Ensures only the vault owner can buy.
  - **Validation**:
    - The user must have sufficient lamports.

- **Take Token**
  - **Function**: `m_take_token`
  - **Description**: Withdraws tokens from the vault.
  - **Parameters**:
    - `amount` (u64): The number of lamports to withdraw.
  - **Ownership Check**: Ensures only the vault owner can withdraw.
  - **Validation**:
    - The vault must have sufficient tokens and lamports.

---

## Error Codes

- **Personality Errors**
  - `ProfileInputDataNotValid`: Invalid profile data.

- **Relic Errors**
  - `RelicInputDataNotValid`: Invalid relic data.

- **Fragment Errors**
  - `FragmentDataLimitExceeded`: Maximum number of fragments exceeded.
  - `FragmentDataNotFound`: Fragment data not found.

- **Other Errors**
  - `UnAuthorized`: Unauthorized access.
  - `LamportNotEnough`: Insufficient lamports.

---

## Helper Functions

- **`transfer_lamports`**
  - **Description**: Handles lamport transfers between accounts.
  - **Parameters**: `from`, `to`, `amount`, `system_program`, `from_pda`

- **`calculate_rent_and_size`**
  - **Description**: Calculates rent and size for account reallocations.
  - **Parameters**: `current_data_len`, `new_data_len`

---

## Account Definitions

### Personality

| Name   | Type         | Description                     | Validation                        |
|--------|--------------|---------------------------------|-----------------------------------|
| owner  | Pubkey       | The owner of the personality.   | -                                 |
| name   | String       | The name of the personality.    | Maximum length: 100 characters.  |
| age    | u16          | The age of the personality.     | -                                 |
| hobbie | Vec<String>  | List of hobbies.                | Maximum 5 items, each up to 100 characters. |
| message| String       | A custom message.               | Maximum length: 300 characters.  |

### Relic

| Name            | Type         | Description                     | Validation                        |
|-----------------|--------------|---------------------------------|-----------------------------------|
| owner           | Pubkey       | The owner of the relic.         | -                                 |
| authority       | Pubkey       | The authority of the relic.     | -                                 |
| name            | String       | The name of the relic.          | Maximum length: 50 characters.   |
| description     | String       | A description of the relic.     | Maximum length: 300 characters.  |
| data_count      | u32          | Number of data entries.         | -                                 |
| size            | u64          | Size of the relic.              | -                                 |
| visibility      | bool         | Visibility status of the relic. | -                                 |
| storage_pointer | Option<Pubkey>| Pointer to storage.            | -                                 |

### Fragments

| Name           | Type         | Description                     | Validation                        |
|----------------|--------------|---------------------------------|-----------------------------------|
| owner          | Pubkey       | The owner of the fragment.      | -                                 |
| authority      | Pubkey       | The authority of the fragment.  | -                                 |
| fragment       | Vec<u8>      | Fragment data.                  | -                                 |
| data_alloc     | u64          | Allocated data size.            | -                                 |
| next_fragments | Option<Pubkey>| Pointer to the next fragment.  | -                                 |

### Vault

| Name      | Type         | Description                     | Validation                        |
|-----------|--------------|---------------------------------|-----------------------------------|
| owner     | Pubkey       | The owner of the vault.         | -                                 |
| authority | Pubkey       | The authority of the vault.     | -                                 |
| token     | u64          | Number of tokens in the vault.  | -                                 |

### Vault Lamport

| Name | Type | Description              | Validation |
|------|------|--------------------------|------------|
| -    | -    | Empty struct for lamport storage. | - |

---

## Events

### DataNotify

- **Description**: Emitted when data is created, updated, or deleted.
- **Fields**:
  - `by` (Pubkey): The public key of the user performing the operation.
  - `account` (Pubkey): The public key of the affected account.
  - `message` (String): A message describing the operation.
  - `operation` (Operation): The type of operation (`Create`, `Update`, `Delete`).

### TokenNotify

- **Description**: Emitted when tokens are transferred or manipulated.
- **Fields**:
  - `by` (Pubkey): The public key of the user performing the operation.
  - `account` (Pubkey): The public key of the affected account.
  - `message` (String): A message describing the operation.
  - `amount` (u64): The amount of tokens involved in the operation.
