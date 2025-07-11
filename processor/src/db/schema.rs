// @generated automatically by Diesel CLI.

diesel::table! {
    account_transactions (account_address, transaction_version) {
        transaction_version -> Int8,
        #[max_length = 66]
        account_address -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_lookup (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 64]
        domain -> Varchar,
        #[max_length = 64]
        subdomain -> Varchar,
        #[max_length = 66]
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Nullable<Timestamp>,
        #[max_length = 140]
        token_name -> Varchar,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_lookup_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 64]
        domain -> Varchar,
        #[max_length = 64]
        subdomain -> Varchar,
        #[max_length = 10]
        token_standard -> Varchar,
        #[max_length = 66]
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Nullable<Timestamp>,
        #[max_length = 140]
        token_name -> Varchar,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        subdomain_expiration_policy -> Nullable<Int8>,
    }
}

diesel::table! {
    ans_primary_name (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        registered_address -> Varchar,
        #[max_length = 64]
        domain -> Nullable<Varchar>,
        #[max_length = 64]
        subdomain -> Nullable<Varchar>,
        #[max_length = 140]
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_primary_name_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        registered_address -> Varchar,
        #[max_length = 64]
        domain -> Nullable<Varchar>,
        #[max_length = 64]
        subdomain -> Nullable<Varchar>,
        #[max_length = 10]
        token_standard -> Varchar,
        #[max_length = 140]
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    auth_key_account_addresses (account_address) {
        #[max_length = 66]
        auth_key -> Varchar,
        #[max_length = 66]
        account_address -> Varchar,
        last_transaction_version -> Int8,
        is_auth_key_used -> Bool,
    }
}

diesel::table! {
    backfill_processor_status (backfill_alias) {
        #[max_length = 100]
        backfill_alias -> Varchar,
        #[max_length = 50]
        backfill_status -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
        backfill_start_version -> Int8,
        backfill_end_version -> Nullable<Int8>,
    }
}

diesel::table! {
    block_metadata_transactions (version) {
        version -> Int8,
        block_height -> Int8,
        #[max_length = 66]
        id -> Varchar,
        round -> Int8,
        epoch -> Int8,
        previous_block_votes_bitvec -> Jsonb,
        #[max_length = 66]
        proposer -> Varchar,
        failed_proposer_indices -> Jsonb,
        timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    coin_activities (transaction_version, event_account_address, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        #[max_length = 66]
        event_account_address -> Varchar,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 5000]
        coin_type -> Varchar,
        amount -> Numeric,
        #[max_length = 200]
        activity_type -> Varchar,
        is_gas_fee -> Bool,
        is_transaction_success -> Bool,
        #[max_length = 1000]
        entry_function_id_str -> Nullable<Varchar>,
        block_height -> Int8,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        event_index -> Nullable<Int8>,
        #[max_length = 66]
        gas_fee_payer_address -> Nullable<Varchar>,
        storage_refund_amount -> Numeric,
    }
}

diesel::table! {
    coin_balances (transaction_version, owner_address, coin_type_hash) {
        transaction_version -> Int8,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 64]
        coin_type_hash -> Varchar,
        #[max_length = 5000]
        coin_type -> Varchar,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    coin_infos (coin_type_hash) {
        #[max_length = 64]
        coin_type_hash -> Varchar,
        #[max_length = 5000]
        coin_type -> Varchar,
        transaction_version_created -> Int8,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 10]
        symbol -> Varchar,
        decimals -> Int4,
        transaction_created_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        #[max_length = 66]
        supply_aggregator_table_handle -> Nullable<Varchar>,
        supply_aggregator_table_key -> Nullable<Text>,
    }
}

diesel::table! {
    coin_supply (transaction_version, coin_type_hash) {
        transaction_version -> Int8,
        #[max_length = 64]
        coin_type_hash -> Varchar,
        #[max_length = 5000]
        coin_type -> Varchar,
        supply -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_epoch -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    collection_datas (collection_data_id_hash, transaction_version) {
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        transaction_version -> Int8,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        description -> Text,
        #[max_length = 512]
        metadata_uri -> Varchar,
        supply -> Numeric,
        maximum -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        inserted_at -> Timestamp,
        #[max_length = 66]
        table_handle -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    collections_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        collection_id -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        description -> Text,
        #[max_length = 512]
        uri -> Varchar,
        current_supply -> Numeric,
        max_supply -> Nullable<Numeric>,
        total_minted_v2 -> Nullable<Numeric>,
        mutable_description -> Nullable<Bool>,
        mutable_uri -> Nullable<Bool>,
        #[max_length = 66]
        table_handle_v1 -> Nullable<Varchar>,
        #[max_length = 10]
        token_standard -> Varchar,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        collection_properties -> Nullable<Jsonb>,
    }
}

diesel::table! {
    current_ans_lookup (domain, subdomain) {
        #[max_length = 64]
        domain -> Varchar,
        #[max_length = 64]
        subdomain -> Varchar,
        #[max_length = 66]
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        #[max_length = 140]
        token_name -> Varchar,
        is_deleted -> Bool,
    }
}

diesel::table! {
    current_ans_lookup_v2 (domain, subdomain, token_standard) {
        #[max_length = 64]
        domain -> Varchar,
        #[max_length = 64]
        subdomain -> Varchar,
        #[max_length = 10]
        token_standard -> Varchar,
        #[max_length = 140]
        token_name -> Nullable<Varchar>,
        #[max_length = 66]
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        subdomain_expiration_policy -> Nullable<Int8>,
    }
}

diesel::table! {
    current_ans_primary_name (registered_address) {
        #[max_length = 66]
        registered_address -> Varchar,
        #[max_length = 64]
        domain -> Nullable<Varchar>,
        #[max_length = 64]
        subdomain -> Nullable<Varchar>,
        #[max_length = 140]
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_ans_primary_name_v2 (registered_address, token_standard) {
        #[max_length = 66]
        registered_address -> Varchar,
        #[max_length = 10]
        token_standard -> Varchar,
        #[max_length = 64]
        domain -> Nullable<Varchar>,
        #[max_length = 64]
        subdomain -> Nullable<Varchar>,
        #[max_length = 140]
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_coin_balances (owner_address, coin_type_hash) {
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 64]
        coin_type_hash -> Varchar,
        #[max_length = 5000]
        coin_type -> Varchar,
        amount -> Numeric,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_collection_datas (collection_data_id_hash) {
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        description -> Text,
        #[max_length = 512]
        metadata_uri -> Varchar,
        supply -> Numeric,
        maximum -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        #[max_length = 66]
        table_handle -> Varchar,
        last_transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    current_collections_v2 (collection_id) {
        #[max_length = 66]
        collection_id -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        description -> Text,
        #[max_length = 512]
        uri -> Varchar,
        current_supply -> Numeric,
        max_supply -> Nullable<Numeric>,
        total_minted_v2 -> Nullable<Numeric>,
        mutable_description -> Nullable<Bool>,
        mutable_uri -> Nullable<Bool>,
        #[max_length = 66]
        table_handle_v1 -> Nullable<Varchar>,
        #[max_length = 10]
        token_standard -> Varchar,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        collection_properties -> Nullable<Jsonb>,
    }
}

diesel::table! {
    current_delegated_staking_pool_balances (staking_pool_address) {
        #[max_length = 66]
        staking_pool_address -> Varchar,
        total_coins -> Numeric,
        total_shares -> Numeric,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        operator_commission_percentage -> Numeric,
        #[max_length = 66]
        inactive_table_handle -> Varchar,
        #[max_length = 66]
        active_table_handle -> Varchar,
    }
}

diesel::table! {
    current_delegated_voter (delegation_pool_address, delegator_address) {
        #[max_length = 66]
        delegation_pool_address -> Varchar,
        #[max_length = 66]
        delegator_address -> Varchar,
        #[max_length = 66]
        table_handle -> Nullable<Varchar>,
        #[max_length = 66]
        voter -> Nullable<Varchar>,
        #[max_length = 66]
        pending_voter -> Nullable<Varchar>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_delegator_balances (delegator_address, pool_address, pool_type, table_handle) {
        #[max_length = 66]
        delegator_address -> Varchar,
        #[max_length = 66]
        pool_address -> Varchar,
        #[max_length = 100]
        pool_type -> Varchar,
        #[max_length = 66]
        table_handle -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        shares -> Numeric,
        #[max_length = 66]
        parent_table_handle -> Varchar,
    }
}

diesel::table! {
    current_fungible_asset_balances (storage_id) {
        #[max_length = 66]
        storage_id -> Varchar,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 66]
        asset_type_v2 -> Nullable<Varchar>,
        #[max_length = 1000]
        asset_type_v1 -> Nullable<Varchar>,
        is_primary -> Bool,
        is_frozen -> Bool,
        amount_v1 -> Nullable<Numeric>,
        amount_v2 -> Nullable<Numeric>,
        amount -> Numeric,
        last_transaction_version_v1 -> Nullable<Int8>,
        last_transaction_version_v2 -> Nullable<Int8>,
        last_transaction_version -> Nullable<Int8>,
        last_transaction_timestamp_v1 -> Nullable<Timestamp>,
        last_transaction_timestamp_v2 -> Nullable<Timestamp>,
        last_transaction_timestamp -> Nullable<Timestamp>,
        inserted_at -> Timestamp,
        #[max_length = 1000]
        asset_type -> Varchar,
        #[max_length = 10]
        token_standard -> Varchar,
    }
}

diesel::table! {
    current_fungible_asset_balances_legacy (storage_id) {
        #[max_length = 66]
        storage_id -> Varchar,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 1000]
        asset_type -> Varchar,
        is_primary -> Bool,
        is_frozen -> Bool,
        amount -> Numeric,
        last_transaction_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        #[max_length = 10]
        token_standard -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_objects (object_address) {
        #[max_length = 66]
        object_address -> Varchar,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 66]
        state_key_hash -> Varchar,
        allow_ungated_transfer -> Bool,
        last_guid_creation_num -> Numeric,
        last_transaction_version -> Int8,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        untransferrable -> Bool,
    }
}

diesel::table! {
    current_staking_pool_voter (staking_pool_address) {
        #[max_length = 66]
        staking_pool_address -> Varchar,
        #[max_length = 66]
        voter_address -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        #[max_length = 66]
        operator_address -> Varchar,
    }
}

diesel::table! {
    current_table_items (table_handle, key_hash) {
        #[max_length = 66]
        table_handle -> Varchar,
        #[max_length = 64]
        key_hash -> Varchar,
        key -> Text,
        decoded_key -> Jsonb,
        decoded_value -> Nullable<Jsonb>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_token_datas (token_data_id_hash) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        maximum -> Numeric,
        supply -> Numeric,
        largest_property_version -> Numeric,
        #[max_length = 512]
        metadata_uri -> Varchar,
        #[max_length = 66]
        payee_address -> Varchar,
        royalty_points_numerator -> Numeric,
        royalty_points_denominator -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        properties_mutable -> Bool,
        royalty_mutable -> Bool,
        default_properties -> Jsonb,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        last_transaction_timestamp -> Timestamp,
        description -> Text,
    }
}

diesel::table! {
    current_token_datas_v2 (token_data_id) {
        #[max_length = 66]
        token_data_id -> Varchar,
        #[max_length = 66]
        collection_id -> Varchar,
        #[max_length = 128]
        token_name -> Varchar,
        maximum -> Nullable<Numeric>,
        supply -> Nullable<Numeric>,
        largest_property_version_v1 -> Nullable<Numeric>,
        #[max_length = 512]
        token_uri -> Varchar,
        description -> Text,
        token_properties -> Jsonb,
        #[max_length = 10]
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        decimals -> Nullable<Int8>,
        is_deleted_v2 -> Nullable<Bool>,
    }
}

diesel::table! {
    current_token_ownerships (token_data_id_hash, property_version, owner_address) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        amount -> Numeric,
        token_properties -> Jsonb,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        table_type -> Text,
        last_transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    current_token_ownerships_v2 (token_data_id, property_version_v1, owner_address, storage_id) {
        #[max_length = 66]
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 66]
        storage_id -> Varchar,
        amount -> Numeric,
        #[max_length = 66]
        table_type_v1 -> Nullable<Varchar>,
        token_properties_mutated_v1 -> Nullable<Jsonb>,
        is_soulbound_v2 -> Nullable<Bool>,
        #[max_length = 10]
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        non_transferrable_by_owner -> Nullable<Bool>,
    }
}

diesel::table! {
    current_token_pending_claims (token_data_id_hash, property_version, from_address, to_address) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        #[max_length = 66]
        from_address -> Varchar,
        #[max_length = 66]
        to_address -> Varchar,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        amount -> Numeric,
        #[max_length = 66]
        table_handle -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        last_transaction_timestamp -> Timestamp,
        #[max_length = 66]
        token_data_id -> Varchar,
        #[max_length = 66]
        collection_id -> Varchar,
    }
}

diesel::table! {
    current_token_royalty_v1 (token_data_id) {
        #[max_length = 66]
        token_data_id -> Varchar,
        #[max_length = 66]
        payee_address -> Varchar,
        royalty_points_numerator -> Numeric,
        royalty_points_denominator -> Numeric,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_token_v2_metadata (object_address, resource_type) {
        #[max_length = 66]
        object_address -> Varchar,
        #[max_length = 128]
        resource_type -> Varchar,
        data -> Jsonb,
        #[max_length = 66]
        state_key_hash -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    delegated_staking_activities (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        #[max_length = 66]
        delegator_address -> Varchar,
        #[max_length = 66]
        pool_address -> Varchar,
        event_type -> Text,
        amount -> Numeric,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    delegated_staking_pool_balances (transaction_version, staking_pool_address) {
        transaction_version -> Int8,
        #[max_length = 66]
        staking_pool_address -> Varchar,
        total_coins -> Numeric,
        total_shares -> Numeric,
        inserted_at -> Timestamp,
        operator_commission_percentage -> Numeric,
        #[max_length = 66]
        inactive_table_handle -> Varchar,
        #[max_length = 66]
        active_table_handle -> Varchar,
    }
}

diesel::table! {
    delegated_staking_pools (staking_pool_address) {
        #[max_length = 66]
        staking_pool_address -> Varchar,
        first_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    delegator_balances (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        delegator_address -> Varchar,
        #[max_length = 66]
        pool_address -> Varchar,
        #[max_length = 100]
        pool_type -> Varchar,
        #[max_length = 66]
        table_handle -> Varchar,
        shares -> Numeric,
        #[max_length = 66]
        parent_table_handle -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    event_size_info (transaction_version, index) {
        transaction_version -> Int8,
        index -> Int8,
        type_tag_bytes -> Int8,
        total_bytes -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    events (transaction_version, event_index) {
        sequence_number -> Int8,
        creation_number -> Int8,
        #[max_length = 66]
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        data -> Jsonb,
        inserted_at -> Timestamp,
        event_index -> Int8,
        #[max_length = 300]
        indexed_type -> Varchar,
    }
}

diesel::table! {
    fungible_asset_activities (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        #[max_length = 66]
        owner_address -> Nullable<Varchar>,
        #[max_length = 66]
        storage_id -> Varchar,
        #[max_length = 1000]
        asset_type -> Nullable<Varchar>,
        is_frozen -> Nullable<Bool>,
        amount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Varchar,
        is_gas_fee -> Bool,
        #[max_length = 66]
        gas_fee_payer_address -> Nullable<Varchar>,
        is_transaction_success -> Bool,
        #[max_length = 1000]
        entry_function_id_str -> Nullable<Varchar>,
        block_height -> Int8,
        #[max_length = 10]
        token_standard -> Varchar,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        storage_refund_amount -> Numeric,
    }
}

diesel::table! {
    fungible_asset_balances (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        storage_id -> Varchar,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 1000]
        asset_type -> Varchar,
        is_primary -> Bool,
        is_frozen -> Bool,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        #[max_length = 10]
        token_standard -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    fungible_asset_metadata (asset_type) {
        #[max_length = 1000]
        asset_type -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 32]
        symbol -> Varchar,
        decimals -> Int4,
        #[max_length = 512]
        icon_uri -> Nullable<Varchar>,
        #[max_length = 512]
        project_uri -> Nullable<Varchar>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        #[max_length = 66]
        supply_aggregator_table_handle_v1 -> Nullable<Varchar>,
        supply_aggregator_table_key_v1 -> Nullable<Text>,
        #[max_length = 10]
        token_standard -> Varchar,
        inserted_at -> Timestamp,
        is_token_v2 -> Nullable<Bool>,
        supply_v2 -> Nullable<Numeric>,
        maximum_v2 -> Nullable<Numeric>,
    }
}

diesel::table! {
    fungible_asset_to_coin_mappings (fungible_asset_metadata_address) {
        #[max_length = 66]
        fungible_asset_metadata_address -> Varchar,
        #[max_length = 1000]
        coin_type -> Varchar,
        last_transaction_version -> Int8,
    }
}

diesel::table! {
    gas_fees (transaction_version) {
        transaction_version -> Int8,
        #[max_length = 66]
        owner_address -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        #[max_length = 66]
        gas_fee_payer_address -> Nullable<Varchar>,
        is_transaction_success -> Bool,
        #[max_length = 1000]
        entry_function_id_str -> Nullable<Varchar>,
        block_height -> Int8,
        transaction_timestamp -> Timestamp,
        storage_refund_amount -> Numeric,
    }
}

diesel::table! {
    indexer_status (db) {
        #[max_length = 50]
        db -> Varchar,
        is_indexer_up -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    move_modules (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        name -> Text,
        #[max_length = 66]
        address -> Varchar,
        bytecode -> Nullable<Bytea>,
        friends -> Nullable<Jsonb>,
        exposed_functions -> Nullable<Jsonb>,
        structs -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    move_resources (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        name -> Text,
        #[max_length = 66]
        address -> Varchar,
        #[sql_name = "type"]
        type_ -> Text,
        module -> Text,
        generic_type_params -> Nullable<Jsonb>,
        data -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        #[max_length = 66]
        state_key_hash -> Varchar,
    }
}

diesel::table! {
    nft_points (transaction_version) {
        transaction_version -> Int8,
        #[max_length = 66]
        owner_address -> Varchar,
        token_name -> Text,
        point_type -> Text,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    objects (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        object_address -> Varchar,
        #[max_length = 66]
        owner_address -> Varchar,
        #[max_length = 66]
        state_key_hash -> Varchar,
        guid_creation_num -> Numeric,
        allow_ungated_transfer -> Bool,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        untransferrable -> Bool,
    }
}

diesel::table! {
    processor_status (processor, processor) {
        #[max_length = 100]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    proposal_votes (transaction_version, proposal_id, voter_address) {
        transaction_version -> Int8,
        proposal_id -> Int8,
        #[max_length = 66]
        voter_address -> Varchar,
        #[max_length = 66]
        staking_pool_address -> Varchar,
        num_votes -> Numeric,
        should_pass -> Bool,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    public_key_auth_keys (auth_key, public_key, public_key_type) {
        #[max_length = 500]
        public_key -> Varchar,
        #[max_length = 50]
        public_key_type -> Varchar,
        #[max_length = 66]
        auth_key -> Varchar,
        is_public_key_used -> Bool,
        last_transaction_version -> Int8,
        #[max_length = 50]
        signature_type -> Varchar,
        #[max_length = 13000] // update MAX_ACCOUNT_PUBLIC_KEY_LENGTH when changing this value
        account_public_key -> Nullable<Varchar>,
    }
}

diesel::table! {
    signatures (transaction_version, multi_agent_index, multi_sig_index, is_sender_primary) {
        transaction_version -> Int8,
        multi_agent_index -> Int8,
        multi_sig_index -> Int8,
        transaction_block_height -> Int8,
        #[max_length = 66]
        signer -> Varchar,
        is_sender_primary -> Bool,
        #[sql_name = "type"]
        type_ -> Varchar,
        public_key -> Text,
        signature -> Text,
        threshold -> Int8,
        public_key_indices -> Jsonb,
        inserted_at -> Timestamp,
        any_signature_type -> Nullable<Varchar>,
        public_key_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    spam_assets (asset) {
        #[max_length = 1100]
        asset -> Varchar,
        is_spam -> Bool,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    table_items (transaction_version, write_set_change_index) {
        key -> Text,
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        #[max_length = 66]
        table_handle -> Varchar,
        decoded_key -> Jsonb,
        decoded_value -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    table_metadatas (handle) {
        #[max_length = 66]
        handle -> Varchar,
        key_type -> Text,
        value_type -> Text,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    token_activities (transaction_version, event_account_address, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        #[max_length = 66]
        event_account_address -> Varchar,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 50]
        transfer_type -> Varchar,
        #[max_length = 66]
        from_address -> Nullable<Varchar>,
        #[max_length = 66]
        to_address -> Nullable<Varchar>,
        token_amount -> Numeric,
        coin_type -> Nullable<Text>,
        coin_amount -> Nullable<Numeric>,
        inserted_at -> Timestamp,
        transaction_timestamp -> Timestamp,
        event_index -> Nullable<Int8>,
    }
}

diesel::table! {
    token_activities_v2 (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        #[max_length = 66]
        event_account_address -> Varchar,
        #[max_length = 66]
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        #[sql_name = "type"]
        type_ -> Varchar,
        #[max_length = 66]
        from_address -> Nullable<Varchar>,
        #[max_length = 66]
        to_address -> Nullable<Varchar>,
        token_amount -> Numeric,
        before_value -> Nullable<Text>,
        after_value -> Nullable<Text>,
        #[max_length = 1000]
        entry_function_id_str -> Nullable<Varchar>,
        #[max_length = 10]
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    token_datas (token_data_id_hash, transaction_version) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        transaction_version -> Int8,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        maximum -> Numeric,
        supply -> Numeric,
        largest_property_version -> Numeric,
        #[max_length = 512]
        metadata_uri -> Varchar,
        #[max_length = 66]
        payee_address -> Varchar,
        royalty_points_numerator -> Numeric,
        royalty_points_denominator -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        properties_mutable -> Bool,
        royalty_mutable -> Bool,
        default_properties -> Jsonb,
        inserted_at -> Timestamp,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
        description -> Text,
    }
}

diesel::table! {
    token_datas_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        token_data_id -> Varchar,
        #[max_length = 66]
        collection_id -> Varchar,
        #[max_length = 128]
        token_name -> Varchar,
        maximum -> Nullable<Numeric>,
        supply -> Nullable<Numeric>,
        largest_property_version_v1 -> Nullable<Numeric>,
        #[max_length = 512]
        token_uri -> Varchar,
        token_properties -> Jsonb,
        description -> Text,
        #[max_length = 10]
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        decimals -> Nullable<Int8>,
        is_deleted_v2 -> Nullable<Bool>,
    }
}

diesel::table! {
    token_ownerships (token_data_id_hash, property_version, transaction_version, table_handle) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        transaction_version -> Int8,
        #[max_length = 66]
        table_handle -> Varchar,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 66]
        owner_address -> Nullable<Varchar>,
        amount -> Numeric,
        table_type -> Nullable<Text>,
        inserted_at -> Timestamp,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    token_ownerships_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        #[max_length = 66]
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        #[max_length = 66]
        owner_address -> Nullable<Varchar>,
        #[max_length = 66]
        storage_id -> Varchar,
        amount -> Numeric,
        #[max_length = 66]
        table_type_v1 -> Nullable<Varchar>,
        token_properties_mutated_v1 -> Nullable<Jsonb>,
        is_soulbound_v2 -> Nullable<Bool>,
        #[max_length = 10]
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        non_transferrable_by_owner -> Nullable<Bool>,
    }
}

diesel::table! {
    tokens (token_data_id_hash, property_version, transaction_version) {
        #[max_length = 64]
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        transaction_version -> Int8,
        #[max_length = 66]
        creator_address -> Varchar,
        #[max_length = 128]
        collection_name -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        token_properties -> Jsonb,
        inserted_at -> Timestamp,
        #[max_length = 64]
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    transaction_size_info (transaction_version) {
        transaction_version -> Int8,
        size_bytes -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    transactions (version) {
        version -> Int8,
        block_height -> Int8,
        #[max_length = 66]
        hash -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        payload -> Nullable<Jsonb>,
        #[max_length = 66]
        state_change_hash -> Varchar,
        #[max_length = 66]
        event_root_hash -> Varchar,
        #[max_length = 66]
        state_checkpoint_hash -> Nullable<Varchar>,
        gas_used -> Numeric,
        success -> Bool,
        vm_status -> Text,
        #[max_length = 66]
        accumulator_root_hash -> Varchar,
        num_events -> Int8,
        num_write_set_changes -> Int8,
        inserted_at -> Timestamp,
        epoch -> Int8,
        #[max_length = 50]
        payload_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_transactions (version) {
        version -> Int8,
        block_height -> Int8,
        #[max_length = 50]
        parent_signature_type -> Varchar,
        #[max_length = 66]
        sender -> Varchar,
        sequence_number -> Nullable<Int8>,
        max_gas_amount -> Numeric,
        expiration_timestamp_secs -> Timestamp,
        gas_unit_price -> Numeric,
        timestamp -> Timestamp,
        #[max_length = 1000]
        entry_function_id_str -> Varchar,
        inserted_at -> Timestamp,
        epoch -> Int8,
        #[max_length = 66]
        entry_function_contract_address -> Nullable<Varchar>,
        #[max_length = 255]
        entry_function_module_name -> Nullable<Varchar>,
        #[max_length = 255]
        entry_function_function_name -> Nullable<Varchar>,
        replay_protection_nonce -> Nullable<Numeric>,
    }
}

diesel::table! {
    write_set_changes (transaction_version, index) {
        transaction_version -> Int8,
        index -> Int8,
        #[max_length = 66]
        hash -> Varchar,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        #[max_length = 66]
        address -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    write_set_size_info (transaction_version, index) {
        transaction_version -> Int8,
        index -> Int8,
        key_bytes -> Int8,
        value_bytes -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account_transactions,
    ans_lookup,
    ans_lookup_v2,
    ans_primary_name,
    ans_primary_name_v2,
    auth_key_account_addresses,
    backfill_processor_status,
    block_metadata_transactions,
    coin_activities,
    coin_balances,
    coin_infos,
    coin_supply,
    collection_datas,
    collections_v2,
    current_ans_lookup,
    current_ans_lookup_v2,
    current_ans_primary_name,
    current_ans_primary_name_v2,
    current_coin_balances,
    current_collection_datas,
    current_collections_v2,
    current_delegated_staking_pool_balances,
    current_delegated_voter,
    current_delegator_balances,
    current_fungible_asset_balances,
    current_fungible_asset_balances_legacy,
    current_objects,
    current_staking_pool_voter,
    current_table_items,
    current_token_datas,
    current_token_datas_v2,
    current_token_ownerships,
    current_token_ownerships_v2,
    current_token_pending_claims,
    current_token_royalty_v1,
    current_token_v2_metadata,
    delegated_staking_activities,
    delegated_staking_pool_balances,
    delegated_staking_pools,
    delegator_balances,
    event_size_info,
    events,
    fungible_asset_activities,
    fungible_asset_balances,
    fungible_asset_metadata,
    fungible_asset_to_coin_mappings,
    gas_fees,
    indexer_status,
    ledger_infos,
    move_modules,
    move_resources,
    nft_points,
    objects,
    processor_status,
    proposal_votes,
    public_key_auth_keys,
    signatures,
    spam_assets,
    table_items,
    table_metadatas,
    token_activities,
    token_activities_v2,
    token_datas,
    token_datas_v2,
    token_ownerships,
    token_ownerships_v2,
    tokens,
    transaction_size_info,
    transactions,
    user_transactions,
    write_set_changes,
    write_set_size_info,
);
