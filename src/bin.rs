use bitcoin::{
    secp256k1::SecretKey,
    util::bip32::{ChainCode, ChildNumber, ExtendedPrivKey, Fingerprint},
    Network, PrivateKey,
};
#[cfg(feature = "wallet_file")]
use libelectrum2descriptors::ElectrumExtendedPrivKey;
#[cfg(feature = "wallet_file")]
use std::str::FromStr;

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    args.next(); // first is program name
    let err_msg =
        "You must specify an extended public or private key or an electrum wallet file as first argument".to_string();
    let electrum_x = args.next().ok_or_else(|| err_msg.clone())?;
    // println!("{:?}", electrum_x);

    let x = SecretKey::from_str(&electrum_x).unwrap();
    let kind = "wpkh";
    let xprv = ExtendedPrivKey {
        network: Network::Bitcoin,
        depth: 1,
        parent_fingerprint: Fingerprint::from_str("a288f4a1").unwrap(),
        child_number: ChildNumber::from_hardened_idx(0).unwrap(),
        chain_code: ChainCode::from_str(
            "860574f7d3b4bf94c000348085d236516fff1cb3de0eaef9385019902ae7ba2e",
        )
        .unwrap(),
        private_key: PrivateKey {
            network: Network::Bitcoin,
            compressed: true,
            key: x,
        },
    };
    println!("Network: {}", xprv.network);
    println!("Depth: {}", xprv.depth);
    println!("Parent Fingerprint: {}", xprv.parent_fingerprint);
    println!("Child Number: {}", xprv.child_number);
    println!("Chain Code: {}", xprv.chain_code);
    println!("Private Key Compressed: {}", xprv.private_key.compressed);
    println!("Private Key Hex: {}", xprv.private_key.key.to_string());
    println!("Kind: {}", kind);
    let descriptor = ElectrumExtendedPrivKey::new(xprv, String::from(kind));

    println!("{}", descriptor.electrum_xprv()?);
    Ok(())
}
