use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853482: FileFormat = FileFormat {
    id: 105_853_482,
    puid: "wikidata/105853482",
    name: "ZX1 bitstream/core",
    extensions: &["zx1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xAA, 0x99, 0x55, 0x66, 0x30, 0xA1, 0x00, 0x07, 0x20, 0x00,
                    0x31, 0xA1, 0x03, 0x80, 0x31, 0x41, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
