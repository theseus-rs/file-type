use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857914: FileFormat = FileFormat {
    id: 105_857_914,
    puid: "wikidata/105857914",
    name: "Commodore CP/M disk image (CBM)",
    extensions: &["d64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x42, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x20, 0x84, 0xFF,
                    0xA9, 0x3E, 0x8D, 0x00, 0xFF, 0xA9, 0xC3, 0x8D, 0xEE, 0xFF, 0xA9, 0x08, 0x8D,
                    0xEF, 0xFF, 0xA9, 0x00, 0x8D, 0xF0, 0xFF, 0x4C, 0xD0, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
