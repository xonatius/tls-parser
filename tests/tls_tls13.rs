extern crate nom;
extern crate tls_parser;

mod tls_13 {
use tls_parser::*;
use nom::IResult;

// Test vectors from https://tools.ietf.org/html/draft-thomson-tls-tls13-vectors-01

static TV_CLIENT_HELLO_1: &'static [u8] = &[
    0x16, 0x03, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01, 0xfc, 0x03, 0x03, 0xce, 0x05, 0xcf, 0xa3, 0xd9,
    0x21, 0x70, 0xcb, 0xc2, 0x46, 0x5c, 0xdc, 0x3e, 0x3a, 0x2f, 0x57, 0x7f, 0x6e, 0xac, 0x80, 0x93,
    0x61, 0x70, 0x8a, 0xb2, 0x44, 0xb0, 0x7d, 0x8f, 0xad, 0x86, 0x16, 0x00, 0x00, 0x3e, 0x13, 0x01,
    0x13, 0x03, 0x13, 0x02, 0xc0, 0x2b, 0xc0, 0x2f, 0xcc, 0xa9, 0xcc, 0xa8, 0xc0, 0x0a, 0xc0, 0x09,
    0xc0, 0x13, 0xc0, 0x23, 0xc0, 0x27, 0xc0, 0x14, 0x00, 0x9e, 0xcc, 0xaa, 0x00, 0x33, 0x00, 0x32,
    0x00, 0x67, 0x00, 0x39, 0x00, 0x38, 0x00, 0x6b, 0x00, 0x16, 0x00, 0x13, 0x00, 0x9c, 0x00, 0x2f,
    0x00, 0x3c, 0x00, 0x35, 0x00, 0x3d, 0x00, 0x0a, 0x00, 0x05, 0x00, 0x04, 0x01, 0x00, 0x01, 0x95,
    0x00, 0x15, 0x00, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x0b, 0x00, 0x09, 0x00, 0x00, 0x06, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0xff,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x0a, 0x00, 0x14, 0x00, 0x12, 0x00, 0x1d, 0x00, 0x17, 0x00, 0x18,
    0x00, 0x19, 0x01, 0x00, 0x01, 0x01, 0x01, 0x02, 0x01, 0x03, 0x01, 0x04, 0x00, 0x0b, 0x00, 0x02,
    0x01, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x28, 0x00, 0x26, 0x00, 0x24, 0x00, 0x1d, 0x00, 0x20,
    0x2a, 0x98, 0x1d, 0xb6, 0xcd, 0xd0, 0x2a, 0x06, 0xc1, 0x76, 0x31, 0x02, 0xc9, 0xe7, 0x41, 0x36,
    0x5a, 0xc4, 0xe6, 0xf7, 0x2b, 0x31, 0x76, 0xa6, 0xbd, 0x6a, 0x35, 0x23, 0xd3, 0xec, 0x0f, 0x4c,
    0x00, 0x2b, 0x00, 0x07, 0x06, 0x7f, 0x12, 0x03, 0x03, 0x03, 0x02, 0x00, 0x0d, 0x00, 0x20, 0x00,
    0x1e, 0x04, 0x03, 0x05, 0x03, 0x06, 0x03, 0x02, 0x03, 0x08, 0x04, 0x08, 0x05, 0x08, 0x06, 0x04,
    0x01, 0x05, 0x01, 0x06, 0x01, 0x02, 0x01, 0x04, 0x02, 0x05, 0x02, 0x06, 0x02, 0x02, 0x02, 0x00,
    0x2d, 0x00, 0x02, 0x01, 0x01,
];

static TV_SERVER_HELLO_1: &'static [u8] = &[
    0x16, 0x03, 0x01, 0x00, 0x52, 0x02, 0x00, 0x00, 0x4e, 0x7f, 0x12, 0x20, 0xb9, 0xc9, 0x20, 0x1c,
    0xd1, 0x71, 0xa1, 0x5a, 0xbb, 0xa4, 0xe7, 0xed, 0xdc, 0xf3, 0xe8, 0x48, 0x8e, 0x71, 0x92, 0xff,
    0xe0, 0x1e, 0xa5, 0xc1, 0x9f, 0x3d, 0x4b, 0x52, 0xff, 0xee, 0xbe, 0x13, 0x01, 0x00, 0x28, 0x00,
    0x28, 0x00, 0x24, 0x00, 0x1d, 0x00, 0x20, 0x9c, 0x1b, 0x0a, 0x74, 0x21, 0x91, 0x9a, 0x73, 0xcb,
    0x57, 0xb3, 0xa0, 0xad, 0x9d, 0x68, 0x05, 0x86, 0x1a, 0x9c, 0x47, 0xe1, 0x1d, 0xf8, 0x63, 0x9d,
    0x25, 0x32, 0x3b, 0x79, 0xce, 0x20, 0x1c,
];

#[test]
fn test_tls13_ch() {
    let empty = &b""[..];
    let bytes = TV_CLIENT_HELLO_1;
    let ciphers = vec![
        0x1301, 0x1303, 0x1302,
        0xc02b, 0xc02f, 0xcca9, 0xcca8, 0xc00a,
        0xc009, 0xc013, 0xc023, 0xc027, 0xc014,
        0x009e, 0xccaa, 0x0033, 0x0032, 0x0067,
        0x0039, 0x0038, 0x006b, 0x0016, 0x0013,
        0x009c, 0x002f, 0x003c, 0x0035, 0x003d,
        0x000a, 0x0005, 0x0004,
    ];
    let expected_ch = TlsPlaintext{
        hdr: TlsRecordHeader{
            record_type: TlsRecordType::Handshake,
            version: 0x0301,
            len: 512,
        },
        msg: vec![TlsMessage::Handshake(
            TlsMessageHandshake::ClientHello(
                    TlsClientHelloContents {
                        version: TlsVersion::Tls12,
                        rand_time: 0xce05cfa3,
                        rand_data: &bytes[15..15+28],
                        session_id: None,
                        ciphers: ciphers,
                        comp: vec![0],
                        ext: Some(&bytes[112..]),
                    })
        )]
    };
    let ires = parse_tls_plaintext(&bytes);
    assert_eq!(ires, IResult::Done(empty, expected_ch));
}

#[test]
fn test_tls13_sh() {
    let empty = &b""[..];
    let bytes = TV_SERVER_HELLO_1;
    let expected_sh = TlsPlaintext{
        hdr: TlsRecordHeader{
            record_type: TlsRecordType::Handshake,
            version: 0x0301,
            len: 82,
        },
        msg: vec![TlsMessage::Handshake(
            TlsMessageHandshake::ServerHelloV13(
                    TlsServerHelloV13Contents {
                        version: TlsVersion::Tls13Draft18,
                        random: &bytes[11..11+32],
                        cipher: 0x1301,
                        ext: Some(&bytes[47..]),
                    })
        )]
    };
    let expected_ext = vec![
        TlsExtension::KeyShare(&bytes[51..]),
    ];
    let ires = parse_tls_plaintext(&bytes);
    assert_eq!(ires, IResult::Done(empty, expected_sh));
    let res = ires.unwrap();

    let msg = &res.1.msg[0];
    let ext_raw = match msg {
        &TlsMessage::Handshake(TlsMessageHandshake::ServerHelloV13(ref sh)) => sh.ext.unwrap(),
        _ => { assert!(false); empty },
    };
    let res_ext = parse_tls_extensions(ext_raw);
    assert_eq!(res_ext, IResult::Done(empty, expected_ext));
}

} // mod tls_13