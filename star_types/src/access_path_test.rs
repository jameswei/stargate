use types::account_address::AccountAddress;

use crate::access_path::*;
use types::account_config::account_struct_tag;
use crypto::hash::CryptoHash;

#[test]
fn test_access_path() {
    let access_path = AccessPath::new_for_account_resource(AccountAddress::random());
    println!("{:#?}", access_path);

    let resource_tag_hash = access_path.resource_tag_hash().unwrap();
    assert_eq!(resource_tag_hash, account_struct_tag().hash());

    let access_path2 = access_path.into_libra_access_path();
    println!("{:#?}", access_path2);
}