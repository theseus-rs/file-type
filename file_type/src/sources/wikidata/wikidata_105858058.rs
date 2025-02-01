use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858058: FileFormat = FileFormat {
    id: 105_858_058,
    puid: "wikidata/105858058",
    name: "Android Generic System Image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0xFF, 0x26, 0xED, 0x01, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x00,
                    0x10, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
