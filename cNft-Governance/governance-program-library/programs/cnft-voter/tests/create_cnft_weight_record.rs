use gpl_cnft_voter::{ state::*, error::CompressedNftVoterError };
use program_test::cnft_voter_test::*;
use crate::program_test::tools::{ assert_cnft_voter_err, assert_compression_err };
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_account_compression::AccountCompressionError;
mod program_test;

#[tokio::test]
async fn test_create_cnft_weight_record() -> Result<(), TransportError> {
    let mut cnft_voter_test = CompressedNftVoterTest::start_new().await;
    let realm_cookie = cnft_voter_test.governance.with_realm().await?;
    let registrar_cookie = cnft_voter_test.with_registrar(&realm_cookie).await?;
    let max_voter_weight_record_cookie = cnft_voter_test.with_max_voter_weight_record(
        &registrar_cookie
    ).await?;
    let cnft_collection_cookie = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    cnft_voter_test.with_collection(
        &registrar_cookie,
        &cnft_collection_cookie,
        &max_voter_weight_record_cookie,
        Some(ConfigureCollectionArgs {
            weight: 3,
            size: 11,
        })
    ).await?;

    let voter_cookie = cnft_voter_test.bench.with_wallet().await;
    let voter_weight_record_cookie = cnft_voter_test.with_voter_weight_record(
        &registrar_cookie,
        &voter_cookie
    ).await?;

    // mint compressed nft
    let mut tree_cookie = cnft_voter_test.merkle_tree.with_merkle_tree(None).await?;
    let leaf_cookie = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    cnft_voter_test.bench.advance_clock().await;

    let (leaf_verification_cookie, proofs, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie,
            5,
            8
        ).await?;

    cnft_voter_test.with_create_cnft_weight_record(
        &registrar_cookie,
        &voter_weight_record_cookie,
        &voter_cookie,
        &[&leaf_cookie],
        &[&leaf_verification_cookie],
        &[&proofs]
    ).await?;

    let asset_id = &leaf_cookie.asset_id;
    let cnft_weight_record = get_cnft_weight_record_address(asset_id).0;
    let cnft_weight_record_info = cnft_voter_test.get_cnft_weight_record_account(
        &cnft_weight_record
    ).await;
    println!("cnft_weight_record_info weight: {}", cnft_weight_record_info.weight);
    assert!(cnft_weight_record_info.weight == 3);

    Ok(())
}

#[tokio::test]
async fn test_create_cnft_weight_record_with_multiple_nfts() -> Result<(), TransportError> {
    let mut cnft_voter_test = CompressedNftVoterTest::start_new().await;
    let realm_cookie = cnft_voter_test.governance.with_realm().await?;
    let registrar_cookie = cnft_voter_test.with_registrar(&realm_cookie).await?;
    let max_voter_weight_record_cookie = cnft_voter_test.with_max_voter_weight_record(
        &registrar_cookie
    ).await?;
    let cnft_collection_cookie = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    cnft_voter_test.with_collection(
        &registrar_cookie,
        &cnft_collection_cookie,
        &max_voter_weight_record_cookie,
        Some(ConfigureCollectionArgs {
            weight: 3,
            size: 11,
        })
    ).await?;

    let voter_cookie = cnft_voter_test.bench.with_wallet().await;
    let voter_weight_record_cookie = cnft_voter_test.with_voter_weight_record(
        &registrar_cookie,
        &voter_cookie
    ).await?;

    // mint compressed nft
    let mut tree_cookie = cnft_voter_test.merkle_tree.with_merkle_tree(None).await?;
    let leaf_cookie1 = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    let leaf_cookie2 = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    cnft_voter_test.bench.advance_clock().await;

    let (leaf_verification_cookie1, proofs1, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie1,
            5,
            8
        ).await?;

    let (leaf_verification_cookie2, proofs2, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie2,
            5,
            8
        ).await?;

    cnft_voter_test.with_create_cnft_weight_record(
        &registrar_cookie,
        &voter_weight_record_cookie,
        &voter_cookie,
        &[&leaf_cookie1, &leaf_cookie2],
        &[&leaf_verification_cookie1, &leaf_verification_cookie2],
        &[&proofs1, &proofs2]
    ).await?;

    let asset_id = &leaf_cookie1.asset_id;
    let cnft_weight_record = get_cnft_weight_record_address(asset_id).0;
    let cnft_weight_record_info = cnft_voter_test.get_cnft_weight_record_account(
        &cnft_weight_record
    ).await;
    println!("cnft_weight_record_info weight: {}", cnft_weight_record_info.weight);
    assert!(cnft_weight_record_info.weight == 3);

    Ok(())
}

#[tokio::test]
async fn test_create_cnft_weight_record_with_unverified_collection_error() -> Result<
    (),
    TransportError
> {
    let mut cnft_voter_test = CompressedNftVoterTest::start_new().await;
    let realm_cookie = cnft_voter_test.governance.with_realm().await?;
    let registrar_cookie = cnft_voter_test.with_registrar(&realm_cookie).await?;
    let max_voter_weight_record_cookie = cnft_voter_test.with_max_voter_weight_record(
        &registrar_cookie
    ).await?;
    let cnft_collection_cookie = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    cnft_voter_test.with_collection(
        &registrar_cookie,
        &cnft_collection_cookie,
        &max_voter_weight_record_cookie,
        Some(ConfigureCollectionArgs {
            weight: 3,
            size: 11,
        })
    ).await?;

    let voter_cookie = cnft_voter_test.bench.with_wallet().await;
    let voter_weight_record_cookie = cnft_voter_test.with_voter_weight_record(
        &registrar_cookie,
        &voter_cookie
    ).await?;

    let mut tree_cookie = cnft_voter_test.merkle_tree.with_merkle_tree(None).await?;
    let leaf_cookie = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    cnft_voter_test.bench.advance_clock().await;

    let (mut leaf_verification_cookie, proofs, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie,
            5,
            8
        ).await?;

    if let Some(collection) = leaf_verification_cookie.collection.as_mut() {
        collection.verified = false;
    }

    let err = cnft_voter_test
        .with_create_cnft_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &voter_cookie,
            &[&leaf_cookie],
            &[&leaf_verification_cookie],
            &[&proofs]
        ).await
        .err()
        .unwrap();

    assert_cnft_voter_err(err, CompressedNftVoterError::CollectionMustBeVerified);
    Ok(())
}

#[tokio::test]
async fn test_create_cnft_weight_record_with_invalid_metadata_error() -> Result<
    (),
    TransportError
> {
    let mut cnft_voter_test = CompressedNftVoterTest::start_new().await;
    let realm_cookie = cnft_voter_test.governance.with_realm().await?;
    let registrar_cookie = cnft_voter_test.with_registrar(&realm_cookie).await?;
    let max_voter_weight_record_cookie = cnft_voter_test.with_max_voter_weight_record(
        &registrar_cookie
    ).await?;
    let cnft_collection_cookie = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    cnft_voter_test.with_collection(
        &registrar_cookie,
        &cnft_collection_cookie,
        &max_voter_weight_record_cookie,
        Some(ConfigureCollectionArgs {
            weight: 3,
            size: 11,
        })
    ).await?;

    let voter_cookie = cnft_voter_test.bench.with_wallet().await;
    let voter_weight_record_cookie = cnft_voter_test.with_voter_weight_record(
        &registrar_cookie,
        &voter_cookie
    ).await?;

    // mint compressed nft
    let mut tree_cookie = cnft_voter_test.merkle_tree.with_merkle_tree(None).await?;

    let leaf_cookie = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    cnft_voter_test.bench.advance_clock().await;

    let (mut leaf_verification_cookie, proofs, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie,
            5,
            8
        ).await?;

    let leaf_cookie2 = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    leaf_verification_cookie = CompressedNftAsset {
        name: leaf_cookie2.metadata.name.clone(),
        symbol: leaf_cookie2.metadata.symbol.clone(),
        uri: leaf_cookie2.metadata.uri.clone(),
        seller_fee_basis_points: leaf_cookie2.metadata.seller_fee_basis_points,
        primary_sale_happened: leaf_cookie2.metadata.primary_sale_happened,
        is_mutable: leaf_cookie2.metadata.is_mutable,
        edition_nonce: leaf_cookie2.metadata.edition_nonce,
        nonce: leaf_cookie2.nonce,
        index: leaf_cookie2.index,
        ..leaf_verification_cookie
    };

    let err = cnft_voter_test
        .with_create_cnft_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &voter_cookie,
            &[&leaf_cookie],
            &[&leaf_verification_cookie],
            &[&proofs]
        ).await
        .err()
        .unwrap();

    assert_compression_err(err, AccountCompressionError::ConcurrentMerkleTreeError);
    Ok(())
}

#[tokio::test]
async fn test_create_cnft_weight_record_with_invalid_collection_error() -> Result<
    (),
    TransportError
> {
    let mut cnft_voter_test = CompressedNftVoterTest::start_new().await;
    let realm_cookie = cnft_voter_test.governance.with_realm().await?;
    let registrar_cookie = cnft_voter_test.with_registrar(&realm_cookie).await?;
    let max_voter_weight_record_cookie = cnft_voter_test.with_max_voter_weight_record(
        &registrar_cookie
    ).await?;
    let cnft_collection_cookie = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    cnft_voter_test.with_collection(
        &registrar_cookie,
        &cnft_collection_cookie,
        &max_voter_weight_record_cookie,
        Some(ConfigureCollectionArgs {
            weight: 3,
            size: 11,
        })
    ).await?;

    let voter_cookie = cnft_voter_test.bench.with_wallet().await;
    let voter_weight_record_cookie = cnft_voter_test.with_voter_weight_record(
        &registrar_cookie,
        &voter_cookie
    ).await?;

    let cnft_collection_cookie2 = cnft_voter_test.token_metadata.with_cnft_collection(10).await?;

    let mut tree_cookie = cnft_voter_test.merkle_tree.with_merkle_tree(None).await?;
    let leaf_cookie = cnft_voter_test.token_metadata.with_compressed_nft_to_collection(
        &cnft_collection_cookie2,
        &mut tree_cookie,
        &voter_cookie
    ).await?;

    cnft_voter_test.bench.advance_clock().await;

    let (leaf_verification_cookie, proofs, _) =
        cnft_voter_test.merkle_tree.get_leaf_verification_info(
            &mut tree_cookie,
            &leaf_cookie,
            5,
            8
        ).await?;

    let err = cnft_voter_test
        .with_create_cnft_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &voter_cookie,
            &[&leaf_cookie],
            &[&leaf_verification_cookie],
            &[&proofs]
        ).await
        .err()
        .unwrap();

    assert_cnft_voter_err(err, CompressedNftVoterError::CollectionNotFound);
    Ok(())
}