use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858585: FileFormat = FileFormat {
    id: 105_858_585,
    puid: "wikidata/105858585",
    name: "Pebble application Binary",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x42, 0x4C, 0x41, 0x50, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
