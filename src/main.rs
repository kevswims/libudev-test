extern crate libudev;

use std::io;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

fn main() {
    let context = libudev::Context::new().unwrap();
    list_devices(&context).unwrap();
}

fn list_devices(context: &libudev::Context) -> io::Result<()> {
    let mut enumerator = libudev::Enumerator::new(&context)?;

    for device in enumerator.scan_devices()? {
        println!("");
        println!("initialized: {:?}", device.is_initialized());
        println!("     devnum: {:?}", device.devnum());
        println!("    syspath: {:?}", device.syspath());
        println!("    devpath: {:?}", device.devpath());
        println!("  subsystem: {:?}", device.subsystem());
        println!("    sysname: {:?}", device.sysname());
        println!("     sysnum: {:?}", device.sysnum());
        println!("    devtype: {:?}", device.devtype());
        println!("     driver: {:?}", device.driver());
        println!("    devnode: {:?}", device.devnode());

        if let Some(parent) = device.parent() {
            println!("     parent: {:?}", parent.syspath());
        }
        else {
            println!("     parent: None");
        }

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    let token_str = claims.sign_with_key(&key).unwrap();

    assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");

    Ok(())
}