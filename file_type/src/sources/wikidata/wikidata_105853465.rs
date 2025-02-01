use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853465: FileFormat = FileFormat {
    id: 105_853_465,
    puid: "wikidata/105853465",
    name: "ZX-Editor Second Edition document",
    extensions: &["zxe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x2D, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30,
                    0x30, 0x20, 0x28, 0x43, 0x29, 0x20, 0x32, 0x30, 0x30, 0x35, 0x20, 0x62, 0x79,
                    0x20, 0x43, 0x6C, 0x61, 0x75, 0x73, 0x20, 0x4A, 0x61, 0x68, 0x6E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
