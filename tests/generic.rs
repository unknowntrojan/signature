use signature::{dynamic_signature, static_signature, Signature};

#[test]
fn dynamic_static_equality() {
    let mut dynamic_sig = dynamic_signature!("mtxex.dll", 0xB8, 0x4D, 0x5A, 0x00, 0x00);

    let mut static_sig = static_signature!("mtxex.dll", 0xB8, 0x4D, 0x5A, 0x00, 0x00);

    let dynamic_addr = dynamic_sig.get();
    let static_addr = static_sig.get();

    print!("{}", dynamic_sig);
    print!("{}", static_sig);

    assert_eq!(dynamic_addr, static_addr);
}
