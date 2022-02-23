#![no_main]

use libfuzzer_sys::fuzz_target;
use pkcs10::CertReq;

fuzz_target!(|input: &[u8]| {
    let _ = CertReq::try_from(input);
});
